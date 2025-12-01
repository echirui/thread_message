# Tasks: LINE風スレッドチャットとGitHub課題連携

**Feature Branch**: `001-threaded-issue-chat`
**Spec**: [specs/001-threaded-issue-chat/spec.md](spec.md)

## Phase 1: Setup
*Goal: Initialize project structure, install dependencies, and configure development environment.*

- [ ] T001 Create project directories (`backend`, `frontend`) per plan.md
- [ ] T002 Initialize Rust backend project with `cargo init backend`
- [ ] T003 Install Backend dependencies (`axum`, `tokio`, `sqlx`, `serde`, `tracing`, `tower-http`) in `backend/Cargo.toml`
- [ ] T004 Initialize React frontend project with `npm create vite@latest frontend`
- [ ] T005 Install Frontend dependencies (`axios`, `socket.io-client`, `react-router-dom`, `lucide-react`, `tailwindcss`) in `frontend/package.json`
- [ ] T006 Configure Tailwind CSS in `frontend/tailwind.config.js` and `frontend/src/index.css`
- [ ] T007 Setup SQLite database and install `sqlx-cli`
- [ ] T008 Create `backend/.env` and `frontend/.env` with configuration from quickstart.md

## Phase 2: Foundation
*Goal: Establish core database connection, error handling, and basic frontend layout.*

- [ ] T009 Create `backend/src/error.rs` defining `AppError` and `Result` types
- [ ] T010 Create `backend/src/db.rs` for SQLite connection pool setup
- [ ] T011 [P] Implement basic server entry point with tracing subscriber in `backend/src/main.rs`
- [ ] T012 [P] Create `frontend/src/api/client.ts` axios instance configuration
- [ ] T013 [P] Create `frontend/src/layouts/MainLayout.tsx` with basic split-pane structure (Main Chat / Thread Sidebar)

## Phase 3: User Story 1 - Real-time Messages (P1)
*Goal: Users can send and receive messages in real-time.*
*Tests: Backend API tests, WebSocket broadcast tests, Component rendering tests.*

- [ ] T014 [US1] Create SQL migration for `messages` table in `backend/migrations/001_create_messages.sql`
- [ ] T015 [US1] Create `Message` struct in `backend/src/models.rs`
- [ ] T016 [US1] Implement `backend/tests/chat_test.rs` for message CRUD operations (Red)
- [ ] T017 [US1] Implement `create_message` and `get_messages` handlers in `backend/src/handlers/chat.rs` (Green)
- [ ] T018 [US1] Implement `backend/tests/ws_test.rs` for WebSocket connection and broadcast (Red)
- [ ] T019 [US1] Implement WebSocket handler and `AppState` broadcast logic in `backend/src/ws/mod.rs` (Green)
- [ ] T020 [US1] Wire up HTTP and WS routes in `backend/src/main.rs`
- [ ] T021 [P] [US1] Implement `MessageList` component in `frontend/src/components/MessageList.tsx`
- [ ] T022 [P] [US1] Implement `ChatInput` component in `frontend/src/components/ChatInput.tsx`
- [ ] T023 [US1] Implement `useChat` hook in `frontend/src/hooks/useChat.ts` handling HTTP fetch and WS events
- [ ] T024 [US1] Integrate Chat components into `frontend/src/App.tsx`

## Phase 4: User Story 2 - Threaded Replies (P1)
*Goal: Users can reply to messages in threads.*
*Tests: Thread retrieval tests, Reply creation tests.*

- [ ] T025 [US1] Update `messages` table migration to include `parent_id` (if not done) or create new migration
- [ ] T026 [US2] Implement `backend/tests/thread_test.rs` for fetching thread replies (Red)
- [ ] T027 [US2] Implement `get_thread_messages` handler in `backend/src/handlers/chat.rs` (Green)
- [ ] T028 [US2] Update `create_message` handler to accept optional `parent_id`
- [ ] T029 [P] [US2] Implement `ThreadPanel` component in `frontend/src/components/ThreadPanel.tsx`
- [ ] T030 [US2] Update `useChat` hook to manage active thread state and fetch replies
- [ ] T031 [US2] Add "Reply" button to `MessageList` items to open `ThreadPanel`

## Phase 5: User Story 3 - GitHub Authentication (P2)
*Goal: Users can authenticate via GitHub.*
*Tests: OAuth flow tests (mocked).*

- [ ] T032 [US3] Create SQL migration for `users` table in `backend/migrations/002_create_users.sql`
- [ ] T033 [US3] Create `User` struct in `backend/src/models.rs`
- [ ] T034 [US3] Implement `backend/tests/auth_test.rs` for login flow (Red)
- [ ] T035 [US3] Implement `login` and `callback` handlers in `backend/src/handlers/auth.rs` using `oauth2` crate (Green)
- [ ] T036 [US3] Implement `get_me` handler to return current user context
- [ ] T037 [P] [US3] Implement `useAuth` hook in `frontend/src/hooks/useAuth.ts`
- [ ] T038 [US3] Create `LoginButton` component and protect `App` routes in `frontend/src/App.tsx`

## Phase 6: User Story 4 - Create GitHub Issue (P2)
*Goal: Create issues from threads.*
*Tests: Issue creation API tests.*

- [ ] T039 [US4] Implement `backend/tests/github_test.rs` for issue creation (Red)
- [ ] T040 [US4] Implement `list_repos` handler in `backend/src/handlers/github.rs`
- [ ] T041 [US4] Implement `create_issue` handler in `backend/src/handlers/github.rs` using `reqwest` (Green)
- [ ] T042 [US4] Update `create_issue` to post a system message back to the thread linking the issue
- [ ] T043 [P] [US4] Create `IssueCreator` form component in `frontend/src/components/IssueCreator.tsx`
- [ ] T044 [US4] Integrate `IssueCreator` into `ThreadPanel` header

## Phase 7: User Story 5 - Link Existing Issue (P3)
*Goal: Unfurl GitHub issue links.*

- [ ] T045 [US5] Implement logic in `backend/src/handlers/chat.rs` to detect GitHub URLs in message content
- [ ] T046 [US5] Implement metadata fetching for detected URLs (requires GitHub API client)
- [ ] T047 [P] [US5] Create `IssueCard` component in `frontend/src/components/ui/IssueCard.tsx`
- [ ] T048 [US5] Update `MessageList` to render `IssueCard` when message contains issue metadata

## Phase 8: Polish & Review
*Goal: UI Refinement and cleanup.*

- [ ] T049 Improve UI styling with Tailwind (Avatars, Bubbles, spacing) in `frontend/src/components/`
- [ ] T050 Add error toast notifications in `frontend/src/App.tsx`
- [ ] T051 Verify all "Independent Test" criteria from spec.md
- [ ] T052 Run `cargo clippy` and `npm run lint` for final code quality check

## Dependencies

- Phase 1 & 2 must complete before any User Story.
- US1 (Real-time) is prerequisite for US2 (Threads).
- US3 (Auth) is prerequisite for US4 (Create Issue) and US5 (Link Issue).
- US2 and US3 can be started in parallel after Phase 2, but integration requires both.

## Implementation Strategy

- **MVP Scope**: Phase 1, 2, 3, 4 (Basic Chat + Threading).
- **TDD Approach**: For every backend logic, write a test in `tests/` first. For complex frontend logic, write a test in `frontend/src/__tests__/` (optional but recommended).
- **Incremental**: Commit after each green state.
