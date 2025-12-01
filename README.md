# Threaded Issue Chat (LINE風スレッドチャットとGitHub課題連携)

このプロジェクトは、リアルタイムメッセージング機能、スレッド形式の返信、およびGitHub Issueとのシームレスな連携を提供するチャットアプリケーションです。

## 概要

チームコミュニケーションを円滑にするための「LINE風」インターフェースを持ち、会話の流れを乱さずに特定のトピックを深掘りできる「スレッド機能」を搭載しています。
さらに、開発者のワークフローを支援するため、チャット内の会話から直接GitHub Issueを作成したり、既存のIssueをリンク展開する機能を備えています。

## 主な機能

1.  **リアルタイムメッセージング**
    *   WebSocketを使用した低遅延なメッセージ送受信。
    *   相手と自分を左右に配置する直感的なUI。
2.  **スレッド（ネストされた返信）**
    *   任意のメッセージに対して1階層のスレッドを作成可能。
    *   メインのチャットフローを阻害せずに議論が可能。
3.  **GitHub認証**
    *   GitHub OAuthを使用した安全なログイン。
    *   ユーザープロフィール（アバター、ユーザー名）の自動取得。
4.  **GitHub Issue連携**
    *   スレッドの内容を元に、アプリ内から直接GitHub Issueを作成。
    *   作成されたIssueへのリンクを自動投稿。
    *   チャットに貼り付けられたGitHub Issue URLのプレビュー展開。

## 技術スタック

### Backend
*   **Language**: Rust (1.75+)
*   **Framework**: Axum
*   **Async Runtime**: Tokio
*   **Database**: SQLite (via SQLx)
*   **WebSocket**: `axum::extract::ws` + `tokio::sync::broadcast`
*   **Authentication**: OAuth2 crate

### Frontend
*   **Language**: TypeScript
*   **Framework**: React
*   **Build Tool**: Vite
*   **Styling**: Tailwind CSS
*   **HTTP Client**: Axios
*   **Real-time**: Native WebSocket / Socket.io-client

## データモデル

主なエンティティ構成は以下の通りです。

*   **User**: GitHubアカウントと紐付くユーザー情報。
*   **Message**: テキストコンテンツ、送信者、タイムスタンプ。`parent_id`を持つことでスレッド返信を表現します。
    *   `parent_id` が `NULL`: メインチャンネルのメッセージ
    *   `parent_id` が `有`: スレッド内の返信

## API / プロトコル

### WebSocket (`/ws`)
リアルタイムなメッセージ受信に使用されます。
*   `SEND_MESSAGE`: クライアントから送信
*   `NEW_MESSAGE`: サーバーからのブロードキャスト

### REST API
*   `/auth/github/*`: OAuth認証フロー
*   `/api/messages`: メッセージ履歴の取得
*   `/api/repos`: ユーザーのリポジトリ一覧取得
*   `/api/issues`: Issueの作成

## セットアップと実行方法

### 前提条件
*   Rust 1.75+
*   Node.js 20+
*   GitHub OAuth App (Client ID & Secret) の作成

### 環境設定

1.  **リポジトリのクローン**
2.  **環境変数の設定**
    `backend/.env`:
    ```env
    DATABASE_URL=sqlite:data.db
    GITHUB_CLIENT_ID=your_client_id
    GITHUB_CLIENT_SECRET=your_client_secret
    HOST=127.0.0.1
    PORT=3000
    FRONTEND_URL=http://localhost:5173
    ```
    `frontend/.env`:
    ```env
    VITE_API_URL=http://localhost:3000
    ```

### データベースセットアップ

```bash
cd backend
# sqlx-cliのインストール（未インストールの場合）
cargo install sqlx-cli
# データベースとテーブルの作成
sqlx database create
sqlx migrate run
```

### アプリケーションの起動

**Backend**:
```bash
cd backend
cargo run
```

**Frontend**:
```bash
cd frontend
npm install
npm run dev
```

ブラウザで `http://localhost:5173` にアクセスしてください。
