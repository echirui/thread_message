# Implementation Plan: LINE風スレッドチャットとGitHub課題連携

**Branch**: `001-threaded-issue-chat` | **Date**: 2025-12-01 | **Spec**: [specs/001-threaded-issue-chat/spec.md](../spec.md)
**Input**: Feature specification from `specs/001-threaded-issue-chat/spec.md`

## Summary

This feature implements a real-time messaging application with LINE-like UI, supporting single-level threaded replies and GitHub issue integration.
**Technical Approach**:
- **Backend**: Rust (Axum) for high-performance WebSocket handling and API services.
- **Database**: SQLite (via sqlx) for reliable, lightweight relational data storage.
- **Frontend**: React (TypeScript/Vite) for a responsive SPA experience.
- **Communication**: WebSockets for real-time chat events; REST for authentication and GitHub interactions.

## Technical Context

**Language/Version**: Rust 1.75+ (Backend), Node.js 20+ / TypeScript 5.x (Frontend)
**Primary Dependencies**: 
  - Backend: `axum`, `tokio`, `sqlx`, `serde`, `reqwest`, `oauth2`
  - Frontend: `react`, `vite`, `tailwindcss`, `socket.io-client` (or native WS)
**Storage**: SQLite (local file-based)
**Testing**: `cargo test` (Backend unit/integration), `vitest` (Frontend components)
**Target Platform**: Local Web Environment (Desktop Chrome/Safari/Firefox)
**Project Type**: Web Application (Frontend + Backend monorepo)
**Performance Goals**: Real-time message delivery < 100ms; UI render < 16ms (60fps)
**Constraints**: Local development focus; GitHub OAuth requires callback URL handling.
**Scale/Scope**: MVP scope (Team usage); Single workspace/channel initially.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle 1**: Library-First (N/A - Application Prototype)
- **Principle 2**: CLI Interface (N/A - Web UI, though backend is a binary)
- **Principle 3**: Test-First (Will implement backend tests for logic)
- **Conclusion**: Proceed. No blocking violations for this prototype context.

## Project Structure

### Documentation (this feature)

```text
specs/001-threaded-issue-chat/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
backend/
├── src/
│   ├── main.rs          # Entry point & Server setup
│   ├── db.rs            # Database connection & pool
│   ├── models.rs        # Data structures (DB entities)
│   ├── error.rs         # Custom error handling
│   ├── handlers/        # HTTP Request Handlers
│   │   ├── mod.rs
│   │   ├── auth.rs      # GitHub OAuth flow
│   │   ├── chat.rs      # Message history APIs
│   │   └── github.rs    # GitHub API integration (issues)
│   └── ws/              # WebSocket logic
│       ├── mod.rs
│       └── state.rs     # Connection state management
├── migrations/          # SQLx migrations
├── Cargo.toml
└── tests/

frontend/
├── src/
│   ├── api/             # REST API clients
│   ├── assets/
│   ├── components/
│   │   ├── ui/          # Reusable UI components
│   │   ├── ChatInput.tsx
│   │   ├── MessageList.tsx
│   │   ├── ThreadPanel.tsx
│   │   └── IssueCreator.tsx
│   ├── hooks/           # Custom React hooks (useWebSocket, useAuth)
│   ├── types/           # TypeScript definitions
│   ├── App.tsx
│   └── main.tsx
├── index.html
├── package.json
├── tsconfig.json
└── vite.config.ts
```

**Structure Decision**: Monorepo-style with distinct `backend/` and `frontend/` directories. This separation ensures clear boundaries between the Rust API/WebSocket server and the React SPA, simplifying build tools and dependency management for each ecosystem.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (None)    | -          | -                                   |