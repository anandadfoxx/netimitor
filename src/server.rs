use actix_web::{HttpRequest, HttpResponse, web};
use futures_util::{SinkExt, StreamExt};

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().body("netimitor healthz ok")
}

fn is_websocket_upgrade_request(req: &HttpRequest) -> bool {
  req
    .headers()
    .get("Upgrade")
    .and_then(|v| v.to_str().ok())
    .is_some_and(|v| v.eq_ignore_ascii_case("websocket"))
    && req
      .headers()
      .get("Connection")
      .and_then(|v| v.to_str().ok())
      .is_some_and(|v| v.to_lowercase().contains("upgrade"))
}

fn actix_to_wreq_message(msg: actix_ws::Message) -> Option<wreq::Message> {
  match msg {
    actix_ws::Message::Text(text) => Some(wreq::Message::text(text.to_string())),
    actix_ws::Message::Binary(data) => Some(wreq::Message::Binary(data)),
    actix_ws::Message::Ping(data) => Some(wreq::Message::Ping(data)),
    actix_ws::Message::Pong(data) => Some(wreq::Message::Pong(data)),
    actix_ws::Message::Close(reason) => {
      let frame = reason.map(|r| {
        let code_u16: u16 = r.code.into();
        wreq::CloseFrame {
          code: wreq::CloseCode(code_u16),
          reason: r.description.unwrap_or_default().into(),
        }
      });
      Some(wreq::Message::Close(frame))
    }
    actix_ws::Message::Continuation(_) | actix_ws::Message::Nop => None,
  }
}

async fn try_forward_to_session(
  session: &mut actix_ws::Session,
  msg: wreq::Message,
) -> Result<(), actix_ws::Closed> {
  match msg {
    wreq::Message::Text(text) => session.text(text.to_string()).await,
    wreq::Message::Binary(data) => session.binary(data).await,
    wreq::Message::Ping(data) => session.ping(&data).await,
    wreq::Message::Pong(data) => session.pong(&data).await,
    wreq::Message::Close(frame) => {
      let reason = frame.map(|f| {
        let code_u16 = u16::from(f.code);
        actix_ws::CloseReason {
          code: actix_ws::CloseCode::from(code_u16),
          description: Some(f.reason.to_string()),
        }
      });
      session.send(actix_ws::Message::Close(reason)).await
    }
  }
}

async fn handle_websocket_upgrade(
  req: HttpRequest,
  body: web::Payload,
  client: &crate::client::ProxyClient,
) -> HttpResponse {
  let path = req.uri().to_string();
  let target_url = match path.strip_prefix('/') {
    Some(url) if url.starts_with("ws://") || url.starts_with("wss://") => url.to_string(),
    _ => {
      return HttpResponse::BadRequest()
        .body("send a request like: /ws://target or /wss://target");
    }
  };

  let (response, session, msg_stream) = match actix_ws::handle(&req, body) {
    Ok(upgraded) => upgraded,
    Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
  };

  let ws = match client.websocket(&target_url).await {
    Ok(ws) => ws,
    Err(e) => {
      log::error!("failed to connect to target WebSocket: {e}");
      return HttpResponse::BadGateway().body(e.to_string());
    }
  };

  actix_web::rt::spawn(relay_websocket_messages(session, msg_stream, ws));

  response
}

async fn relay_websocket_messages(
  session: actix_ws::Session,
  msg_stream: actix_ws::MessageStream,
  ws: wreq::WebSocket,
) {
  let mut session = session;
  let mut msg_stream = msg_stream;
  let (mut ws_tx, mut ws_rx) = ws.split();

  loop {
    tokio::select! {
      msg = msg_stream.recv() => {
        match msg {
          Some(Ok(actix_msg)) => {
            if let Some(wreq_msg) = actix_to_wreq_message(actix_msg) {
              if ws_tx.send(wreq_msg).await.is_err() {
                break;
              }
            }
          }
          Some(Err(_)) | None => break,
        }
      }
      msg = ws_rx.next() => {
        match msg {
          Some(Ok(wreq_msg)) => {
            if try_forward_to_session(&mut session, wreq_msg).await.is_err() {
              break;
            }
          }
          Some(Err(_)) | None => break,
        }
      }
    }
  }

  let _ = session.close(None).await;
}

async fn proxy_handler(
  req: HttpRequest,
  mut payload: web::Payload,
  client: web::Data<crate::client::ProxyClient>,
) -> HttpResponse {
  if is_websocket_upgrade_request(&req) {
    return handle_websocket_upgrade(req, payload, client.get_ref()).await;
  }

  let path = req.uri().to_string();
  let target_url = match path.strip_prefix('/') {
    Some(url) if url.starts_with("http://") || url.starts_with("https://") => url.to_string(),
    _ => {
      return HttpResponse::BadRequest()
        .body("send a request like: /http://target or /https://target");
    }
  };

  let actix_method = req.method().clone();
  let method: wreq::Method = match actix_method.as_str() {
    "GET" => wreq::Method::GET,
    "POST" => wreq::Method::POST,
    "PUT" => wreq::Method::PUT,
    "DELETE" => wreq::Method::DELETE,
    "PATCH" => wreq::Method::PATCH,
    "HEAD" => wreq::Method::HEAD,
    "OPTIONS" => wreq::Method::OPTIONS,
    "CONNECT" => return HttpResponse::NotImplemented().body("CONNECT not supported"),
    _ => wreq::Method::GET,
  };

  let mut wreq_headers = wreq::header::HeaderMap::new();
  for (name, value) in req.headers().iter() {
    let lower = name.as_str().to_lowercase();
    if matches!(
      lower.as_str(),
      "authorization" | "cookie" | "x-api-key" | "x-auth-token"
    ) {
      if let (Ok(n), Ok(v)) = (
        wreq::header::HeaderName::from_bytes(name.as_ref()),
        wreq::header::HeaderValue::from_bytes(value.as_ref()),
      ) {
        wreq_headers.append(n, v);
      }
    }
  }

  let mut body_vec = Vec::new();
  while let Some(chunk) = payload.next().await {
    match chunk {
      Ok(bytes) => body_vec.extend_from_slice(&bytes),
      Err(_) => return HttpResponse::BadRequest().finish(),
    }
  }

  match client
    .send(method, &target_url, wreq_headers, body_vec)
    .await
  {
    Ok(resp) => {
      let status = actix_web::http::StatusCode::from_u16(resp.status().as_u16())
        .unwrap_or(actix_web::http::StatusCode::BAD_GATEWAY);
      let mut builder = HttpResponse::build(status);
      for (name, value) in resp.headers().iter() {
        let lower = name.as_str().to_lowercase();
        if matches!(
          lower.as_str(),
          "transfer-encoding" | "connection" | "keep-alive"
        ) {
          continue;
        }
        if let (Ok(n), Ok(v)) = (
          actix_web::http::header::HeaderName::from_bytes(name.as_ref()),
          actix_web::http::header::HeaderValue::from_bytes(value.as_ref()),
        ) {
          builder.insert_header((n, v));
        }
      }
      match resp.bytes().await {
        Ok(b) => builder.body(b),
        Err(_) => HttpResponse::InternalServerError().finish(),
      }
    }
    Err(e) => HttpResponse::BadGateway().body(e.to_string()),
  }
}

pub fn configure_app(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/healthz").route(web::get().to(health_check)));
  cfg.default_service(web::to(proxy_handler));
}
