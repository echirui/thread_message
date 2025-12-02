use backend::{create_app, models::{CreateMessage, Message}};
use std::net::SocketAddr;
use axum::serve;
use tokio::net::TcpListener;

#[sqlx::test]
async fn test_threaded_messages_crud(pool: sqlx::SqlitePool) {
    let app = create_app(pool.clone()).await;
    
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    tokio::spawn(async move {
        serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let base_url = format!("http://{}", addr);

    // 1. Create a parent message
    let parent_msg_payload = CreateMessage {
        content: "Parent message".to_string(),
        sender_id: "userA".to_string(),
        parent_id: None,
    };
    let resp = client.post(format!("{}/messages", base_url))
        .json(&parent_msg_payload)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let parent_msg: Message = resp.json().await.unwrap();
    assert!(parent_msg.parent_id.is_none());

    // 2. Create a reply message
    let reply_msg_payload = CreateMessage {
        content: "Reply to parent".to_string(),
        sender_id: "userB".to_string(),
        parent_id: Some(parent_msg.id),
    };
    let resp = client.post(format!("{}/messages", base_url))
        .json(&reply_msg_payload)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let reply_msg: Message = resp.json().await.unwrap();
    assert_eq!(reply_msg.parent_id, Some(parent_msg.id));

    // 3. Get main chat messages (should only have parent_msg)
    let resp = client.get(format!("{}/messages", base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let main_messages: Vec<Message> = resp.json().await.unwrap();
    assert_eq!(main_messages.len(), 1);
    assert_eq!(main_messages[0].id, parent_msg.id);
    assert!(main_messages[0].parent_id.is_none());

    // 4. Get thread replies (should only have reply_msg)
    let resp = client.get(format!("{}/messages/{}/replies", base_url, parent_msg.id))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let replies: Vec<Message> = resp.json().await.unwrap();
    assert_eq!(replies.len(), 1);
    assert_eq!(replies[0].id, reply_msg.id);
    assert_eq!(replies[0].parent_id, Some(parent_msg.id));
}