# WebSocket Protocol

**Endpoint**: `/ws`
**Authentication**: Cookie-based (Session ID)

## Connection

1. Client connects to `ws://host/ws`.
2. Server validates session from cookie.
3. If valid, connection upgraded.
4. If invalid, connection closed with 401.

## Message Format

All messages are JSON objects with a `type` field.

### Client -> Server

#### `SEND_MESSAGE`
Send a new message to the channel or a thread.

```json
{
  "type": "SEND_MESSAGE",
  "content": "Hello world",
  "parent_id": 123  // Optional. If present, this is a reply.
}
```

### Server -> Client

#### `NEW_MESSAGE`
Broadcast when a new message is successfully received and saved.

```json
{
  "type": "NEW_MESSAGE",
  "message": {
    "id": 456,
    "content": "Hello world",
    "user": {
      "id": 1,
      "username": "octocat",
      "avatar_url": "..."
    },
    "parent_id": 123,
    "created_at": "2023-10-27T10:00:00Z"
  }
}
```

#### `ERROR`
Sent when an operation fails.

```json
{
  "type": "ERROR",
  "code": "INVALID_PAYLOAD",
  "message": "Content cannot be empty"
}
```
