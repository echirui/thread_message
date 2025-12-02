use backend::{create_app, models::{Message, CreateMessage}};
use std::net::SocketAddr;
use axum::serve;
use tokio::net::TcpListener;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use futures::StreamExt;
use url::Url;

#[sqlx::test]
async fn test_ws_broadcast(pool: sqlx::SqlitePool) {
    let app = create_app(pool.clone()).await;
    
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    tokio::spawn(async move {
        serve(listener, app).await.unwrap();
    });

    // Connect WebSocket
    let ws_url = Url::parse(&format!("ws://{}/ws", addr)).unwrap();
    let (ws_stream, _) = connect_async(ws_url.to_string()).await.expect("Failed to connect");
    let (_, mut read) = ws_stream.split();

    // Send message via HTTP
    let client = reqwest::Client::new();
    let base_url = format!("http://{}", addr);
    let new_msg = CreateMessage {
        content: "Broadcast Test".to_string(),
        sender_id: "user2".to_string(),
    };
    
    let resp = client.post(format!("{}/messages", base_url))
        .json(&new_msg)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    // Receive from WebSocket
    if let Some(msg) = read.next().await {
        let msg = msg.expect("Error reading message");
        if let WsMessage::Text(text) = msg {
            let received: Message = serde_json::from_str(&text).expect("Failed to parse JSON");
            assert_eq!(received.content, "Broadcast Test");
            assert_eq!(received.sender_id, "user2");
        } else {
            panic!("Expected text message");
        }
    } else {
        panic!("Stream closed without message");
    }
}
