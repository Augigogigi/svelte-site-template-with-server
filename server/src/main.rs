#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(async_closure)]

mod server;

use std::net::SocketAddr;

use axum::{
    extract::{connect_info::ConnectInfo, ws::WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};

pub async fn handle(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| server::handle(socket, addr).await)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/server", get(handle));

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 2940)))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
