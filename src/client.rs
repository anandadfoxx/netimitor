use wreq::{Client, Error, Method, Response, header::HeaderMap};
use wreq_util::Emulation;

#[derive(Clone)]
pub struct ProxyClient {
  client: wreq::Client,
}

impl Default for ProxyClient {
  fn default() -> Self {
    ProxyClient::new(Emulation::Chrome137)
  }
}

impl ProxyClient {
  pub fn new(emulation: Emulation) -> Self {
    ProxyClient {
      client: Client::builder()
        .emulation(emulation)
        .build()
        .expect("unable to instantiate wreq client"),
    }
  }

  pub async fn send(
    &self,
    method: Method,
    url: &str,
    headers: HeaderMap,
    body: Vec<u8>,
  ) -> Result<Response, Error> {
    let mut req = self.client.request(method, url);
    for (name, value) in headers.iter() {
      req = req.header(name, value);
    }
    if !body.is_empty() {
      req = req.body(body);
    }
    req.send().await
  }
}
