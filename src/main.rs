use axum::{extract::ConnectInfo, routing::get, Json, Router};
use hyper::{
    header::{ACCEPT_LANGUAGE, USER_AGENT},
    HeaderMap,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, str::FromStr};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // read the port from env or use the port default port(8080)
    let port = std::env::var("PORT").unwrap_or(String::from("8080"));
    // convert the port to a socket address
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();

    // build our application with a route
    let app = Router::new().route("/api/whoami", get(who_am_i));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("server failed");
}

#[derive(Deserialize, Serialize)]
struct Response {
    ipaddress: String,
    language: String,
    software: String,
}

async fn who_am_i(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Json<Response> {
    let ipaddress = addr.ip().to_string();

    let language = match headers.get(ACCEPT_LANGUAGE) {
        Some(lang) => lang.to_str().unwrap().to_string(),
        None => "Unknown".to_string(),
    };

    let software = match headers.get(USER_AGENT) {
        Some(ua) => ua.to_str().unwrap().to_string(),
        None => "Unknown".to_string(),
    };

    Json(Response {
        ipaddress,
        language,
        software,
    })
}
