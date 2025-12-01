# Feature Specification: Threaded Messaging with GitHub Integration

**Feature Branch**: `001-thread-gh-issues`
**Created**: 2025-11-29
**Status**: Draft
**Input**: User description: "line 風メッセージアプリを作成したいです。メッセージからthreadをネストさせてやりとりできるようにしたいです。また、github のissue と紐づけられるようにしたいです。気軽にディスカッションしてissueを作成できるようにしたいです。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Send and Receive Real-time Messages (Priority: P1)

As a user, I want to exchange messages in a main channel view so that I can communicate with my team in real-time, similar to LINE.

**Why this priority**: Core functionality of a messaging application. Without this, the app does not exist.

**Independent Test**: Can be tested by two users on different clients sending messages and verifying immediate receipt.

**Acceptance Scenarios**:

1. **Given** I am in the main chat view, **When** I type a text message and hit send, **Then** the message appears immediately in my view and other users' views.
2. **Given** I receive a message, **When** the sender is different from me, **Then** the message is displayed on the left side (or distinct style) with their avatar/name.
3. **Given** I send a message, **When** it is my message, **Then** it is displayed on the right side (or distinct style).

---

### User Story 2 - Nested Thread Discussions (Priority: P1)

As a user, I want to reply to a specific message in a nested thread so that detailed discussions do not clutter the main channel.

**Why this priority**: Specifically requested "nest threads from messages" feature to organize conversation.

**Independent Test**: Can be tested by replying to a message and verifying the main channel remains uncluttered while the thread view shows the conversation.

**Acceptance Scenarios**:

1. **Given** a message in the main channel, **When** I click "Reply" or "Start Thread", **Then** a dedicated thread view opens (side panel or overlay).
2. **Given** a thread is active, **When** I send a message in the thread, **Then** it appears only in that thread context, not as a new top-level message in the main channel.
3. **Given** a message has replies, **When** I view it in the main channel, **Then** an indicator shows the number of replies (e.g., "3 replies").

---

### User Story 3 - GitHub Authentication (Priority: P2)

As a user, I want to authenticate with my GitHub account so that the app can access my repositories and issues on my behalf.

**Why this priority**: Prerequisite for issue linking and creation.

**Independent Test**: Can be tested by clicking "Login with GitHub" and verifying the user profile reflects the connected state.

**Acceptance Scenarios**:

1. **Given** I am an unauthenticated user, **When** I attempt to use a GitHub feature, **Then** I am prompted to authorize via GitHub OAuth.
2. **Given** I complete the OAuth flow, **When** I return to the app, **Then** my GitHub username is associated with my app profile.

---

### User Story 4 - Create GitHub Issue from Thread (Priority: P2)

As a user, I want to easily create a GitHub issue from a discussion thread so that actionable items are tracked in our project management tool.

**Why this priority**: Key value proposition ("easily discuss and create issues").

**Independent Test**: Can be tested by clicking "Create Issue" in a thread and verifying a new issue appears in the target GitHub repository.

**Acceptance Scenarios**:

1. **Given** I am in a discussion thread, **When** I click "Create Issue", **Then** a form appears pre-filled with the thread summary or selected messages.
2. **Given** I submit the issue form, **When** the API call succeeds, **Then** a system message is posted in the thread linking to the created GitHub issue.
3. **Given** I am creating an issue, **When** I need to select a repository, **Then** I can choose from a list of repositories I have access to.

---

### User Story 5 - Link Existing GitHub Issue (Priority: P3)

As a user, I want to link an existing GitHub issue to a message or thread so that we can reference external context.

**Why this priority**: Enhances integration but less critical than creation.

**Independent Test**: Can be tested by pasting an issue URL or ID and verifying it resolves to a link preview.

**Acceptance Scenarios**:

1. **Given** a thread, **When** I paste a GitHub issue URL, **Then** the app detects it and displays a rich preview (Title, Status, Assignee).
2. **Given** a linked issue, **When** the issue status changes on GitHub (e.g., closed), **Then** the app reflects this status (eventually or on refresh).

### Edge Cases

- What happens when the user's GitHub token expires? (Should prompt re-auth)
- How does the system handle GitHub API rate limits? (Should queue or notify user)
- What happens if a linked GitHub issue is deleted? (Link should remain but indicate "Not Found")
- What happens to threads if the parent message is deleted? (Thread should likely be preserved or archived)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow users to send text-based messages in a shared channel.
- **FR-002**: System MUST support single-level nesting (threads) originating from any parent message.
- **FR-003**: System MUST display messages in a "LINE-like" bubble interface (user right, others left).
- **FR-004**: System MUST authenticate users via GitHub OAuth 2.0.
- **FR-005**: System MUST allow users to select a target GitHub repository from their available list.
- **FR-006**: System MUST allow creating a new GitHub issue using content from a message or thread.
- **FR-007**: System MUST post a confirmation message with a link back to the GitHub issue upon successful creation.
- **FR-008**: System MUST unfurl/preview GitHub issue links pasted into the chat.

### Key Entities

- **User**: App user account, linked to GitHub User ID.
- **Message**: Text content, Timestamp, Author, Parent Message ID (nullable, for thread replies).
- **Thread**: Virtual entity defined by a Parent Message and its children.
- **GitHub Integration**: Stores OAuth tokens and repository preferences.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can send a message and see it appear on another client in under 1 second (Real-time).
- **SC-002**: Users can initiate the "Create Issue" flow and submit it in under 30 seconds.
- **SC-003**: 100% of created issues contain a back-link or reference to the original chat thread (for context).
- **SC-004**: System handles standard GitHub API rate limit errors with a user-friendly "Try again later" message, not a crash.