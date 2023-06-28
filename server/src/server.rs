use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};

pub async fn handle(socket: WebSocket, who: SocketAddr) {
    println!("Connected to {}!", who);

    let (mut sender, mut receiver) = socket.split();

    let handle = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(s) => {
                    println!("Recieved string from {}: {}", who, s);
                    sender
                        .send(Message::Text("Hello!".to_string()))
                        .await
                        .expect("Failed to send!");
                }
                _ => {}
            }
        }
    });

    handle.await.expect("Handle task panicked!");
    println!("Connection from {} terminated.", who);
}
