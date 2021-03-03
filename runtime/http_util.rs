// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

use deno_core::error::generic_error;
use deno_core::error::AnyError;
use deno_fetch::reqwest;
use deno_fetch::reqwest::header::HeaderMap;
use deno_fetch::reqwest::header::USER_AGENT;
use deno_fetch::reqwest::redirect::Policy;
use deno_fetch::reqwest::Client;

/// Create new instance of async reqwest::Client. This client supports
/// proxies and doesn't follow redirects.
pub fn create_http_client(
  user_agent: String,
  ca_data: Option<Vec<u8>>,
  proxy: Option<Proxy>,
) -> Result<Client, AnyError> {
  let mut headers = HeaderMap::new();
  headers.insert(USER_AGENT, user_agent.parse().unwrap());
  let mut builder = Client::builder()
    .redirect(Policy::none())
    .default_headers(headers)
    .use_rustls_tls();

  if let Some(ca_data) = ca_data {
    let cert = reqwest::Certificate::from_pem(&ca_data)?;
    builder = builder.add_root_certificate(cert);
  }

  if let Some(proxy) = proxy {
    let mut reqwest_proxy  = reqwest::Proxy::all(&proxy.url)?;
    if let Some(basic_auth) = &proxy.basic_auth {
      reqwest_proxy = reqwest_proxy.basic_auth(&basic_auth.username, &basic_auth.password);
    }
    builder = builder.proxy(reqwest_proxy);
  }

  builder
    .build()
    .map_err(|e| generic_error(format!("Unable to build http client: {}", e)))
}

pub struct Proxy {
  pub url: String,
  pub basic_auth: Option<BasicAuth>,
}

pub struct BasicAuth {
  pub username: String,
  pub password: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_test_client() {
    create_http_client("test_client".to_string(), None, None).unwrap();
  }
}
