use actix_web::{HttpRequest, HttpResponse, web};

async fn health_check() -> HttpResponse {
  HttpResponse::Ok().body("netimitor healthz ok")
}

async fn proxy_handler(
  req: HttpRequest,
  body: web::Bytes,
  client: web::Data<crate::client::ProxyClient>,
) -> HttpResponse {
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

  let body_vec = body.to_vec();

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
