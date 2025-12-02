use backend::{create_app, models::{Message, CreateMessage}};
use std::net::SocketAddr;
use axum::serve;
use tokio::net::TcpListener;

#[sqlx::test]
async fn test_create_and_get_messages(pool: sqlx::SqlitePool) {
    let app = create_app(pool.clone()).await;
    
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    tokio::spawn(async move {
        serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let base_url = format!("http://{}", addr);

    // 1. Initial get - empty
    let resp = client.get(format!("{}/messages", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let messages: Vec<Message> = resp.json().await.unwrap();
    assert!(messages.is_empty());

    // 2. Create message
    let new_msg = CreateMessage {
        content: "Hello World".to_string(),
        sender_id: "user1".to_string(),
        parent_id: None, // 追加
    };
    let resp = client.post(format!("{}/messages", base_url))
        .json(&new_msg)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let created: Message = resp.json().await.unwrap();
    assert_eq!(created.content, "Hello World");
    assert_eq!(created.sender_id, "user1");
    assert!(created.parent_id.is_none()); // 確認も追加

    // 3. Get messages again - should have 1
    let resp = client.get(format!("{}/messages", base_url))
        .send()
        .await
        .unwrap();
    let messages: Vec<Message> = resp.json().await.unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Hello World");
}
