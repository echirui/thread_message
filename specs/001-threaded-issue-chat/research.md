# Research & Architecture Decisions

**Feature**: Threaded Messaging with GitHub Integration
**Date**: 2025-12-01

## 1. Real-time Communication Strategy

**Decision**: Use **WebSockets** via `axum::extract::ws` with `tokio::sync::broadcast`.

**Rationale**:
- **Requirement**: "Real-time" (SC-001 < 1 second). HTTP polling is too slow and resource-intensive.
- **Tech Stack**: Axum has first-class support for WebSockets.
- **Implementation**:
  - A global `AppState` will hold a `broadcast::Sender`.
  - Each WebSocket connection subscribes to this sender.
  - When a user sends a message, the handler saves to DB -> sends to broadcast channel -> all active connections receive it.
  - This allows for immediate UI updates without polling.

**Alternatives Considered**:
- **Server-Sent Events (SSE)**: Good for one-way (server -> client), but chat is bi-directional. WebSockets are more natural for chat.
- **HTTP Polling**: Too much latency, poor UX.

## 2. Database Schema for Threading

**Decision**: **Adjacency List** pattern (`parent_id` self-reference) in SQLite.

**Rationale**:
- **Requirement**: "Single-level nesting" (FR-002).
- **Implementation**:
  - Table `messages`: `id`, `content`, `user_id`, `created_at`, `parent_id` (nullable).
  - `parent_id IS NULL`: Top-level message.
  - `parent_id IS NOT NULL`: Thread reply.
  - Queries are simple: `SELECT * FROM messages WHERE parent_id = ?` to get a thread.
  - `SELECT * FROM messages WHERE parent_id IS NULL` to get main channel.

**Alternatives Considered**:
- **Nested Sets / Path Enumeration**: Overkill for single-level threading. Complexity of updates outweighs read benefits for this scale.
- **Separate `threads` table**: Could work, but since a "thread" is just a collection of messages starting from a parent, a single table is simpler and more flexible.

## 3. GitHub Authentication & Integration

**Decision**: **OAuth 2.0 Authorization Code Flow** handled by Backend.

**Rationale**:
- **Requirement**: "Authenticate via GitHub" (FR-004) and "Create Issue" (FR-006).
- **Security**: Client secrets must never be exposed to the frontend. The backend must perform the token exchange.
- **Flow**:
  1. Frontend calls `/api/auth/github`.
  2. Backend generates URL using `oauth2` crate and redirects user to GitHub.
  3. GitHub redirects to `/api/auth/github/callback` with `code`.
  4. Backend exchanges `code` for `access_token`.
  5. Backend creates/updates `User` in DB with `github_id` and `access_token`.
  6. Backend issues a session cookie (or JWT) to Frontend.

**Issue Creation**:
- Backend will use the stored `access_token` for the user to make requests to `https://api.github.com/repos/{owner}/{repo}/issues` using `reqwest`.

## 4. Frontend State Management

**Decision**: **React Context + Custom Hooks** (`useChat`, `useAuth`).

**Rationale**:
- **Scope**: Complexity is moderate. Redux/Zustand might be overkill but Context is sufficient for "Global User" and "Current Chat Messages".
- **Optimistic Updates**: For best UX (LINE-like), messages should appear locally immediately while sending to WS.

## 5. User Interface Structure

**Decision**: **Split View** (Main Channel + Sidebar Thread).

**Rationale**:
- **Requirement**: "Nest threads... without cluttering main channel".
- **Layout**:
  - Left/Center: Main Chat Stream.
  - Right Sidebar: Open Thread (slide-in or fixed).
  - This mimics Slack/Teams threading models which are familiar to developers.
