use axum::{
    body::Bytes,
    http::{HeaderMap, Method, StatusCode, Uri},
    response::{IntoResponse, Json},
    routing::any,
    serve, Router,
};
use axum_client_ip::SecureClientIp;
use axum_extra::extract::cookie::CookieJar;
use gethostname::gethostname;
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct EchoResponse {
    path: String,
    headers: Vec<(String, String)>,
    method: String,
    body: String,
    cookies: Vec<(String, String)>,
    hostname: String,
    ip: String,
    protocol: String,
    query: String,
    os: OsInfo,
}

impl IntoResponse for EchoResponse {
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());
        (StatusCode::OK, headers, Json(self)).into_response()
    }
}

#[derive(Serialize)]
struct OsInfo {
    hostname: String,
}

async fn echo_handler(
    SecureClientIp(ip): SecureClientIp,
    cookie_jar: CookieJar,
    method: Method,
    uri: Uri,
    header_map: HeaderMap,
    body: Bytes,
) -> EchoResponse {
    let headers = header_map
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    let cookies = cookie_jar
        .iter()
        .map(|cookie| (cookie.name().to_string(), cookie.value().to_string()))
        .collect();

    EchoResponse {
        path: uri.path().to_string(),
        headers,
        method: method.to_string(),
        body: String::from_utf8_lossy(&body).to_string(),
        cookies,
        hostname: uri.host().unwrap_or("").to_string(),
        ip: ip.to_string(),
        protocol: uri.scheme_str().unwrap_or("").to_string(),
        query: uri.query().unwrap_or("").to_string(),
        os: OsInfo {
            hostname: gethostname().to_string_lossy().to_string(),
        },
    }
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", any(echo_handler));
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
