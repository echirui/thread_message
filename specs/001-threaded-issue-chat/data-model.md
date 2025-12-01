# Data Model

## Entities

### User
Represents a user authenticated via GitHub.

| Field | Type | Description | Attributes |
|-------|------|-------------|------------|
| `id` | UUID / Integer | Primary Key | PK |
| `github_id` | Integer | GitHub User ID | Unique, Not Null |
| `username` | String | GitHub Login/Username | Not Null |
| `avatar_url` | String | URL to GitHub avatar | |
| `access_token` | String | GitHub OAuth Access Token | Encrypted/Protected |
| `created_at` | DateTime | Account creation time | Default: Now |

### Message
A text message sent by a user. Can be a top-level message or a thread reply.

| Field | Type | Description | Attributes |
|-------|------|-------------|------------|
| `id` | UUID / Integer | Primary Key | PK |
| `user_id` | UUID / Integer | Foreign Key to User | FK, Not Null |
| `content` | Text | The message text | Not Null |
| `parent_id` | UUID / Integer | ID of parent message | FK, Nullable |
| `created_at` | DateTime | Message timestamp | Default: Now |

*Note*: If `parent_id` is NULL, it is a main channel message. If `parent_id` is set, it belongs to that thread.

## Database Schema (SQLite/SQLx)

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    github_id INTEGER NOT NULL UNIQUE,
    username TEXT NOT NULL,
    avatar_url TEXT,
    access_token TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    parent_id INTEGER,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (parent_id) REFERENCES messages(id)
);

-- Index for fetching threads quickly
CREATE INDEX idx_messages_parent_id ON messages(parent_id);
```
