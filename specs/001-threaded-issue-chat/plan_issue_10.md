# Issue 10 Implementation Plan: GitHub Authentication

## 1. 要件定義 (Requirements)

### 機能要件
- **GitHub OAuth 2.0 認証**: ユーザーはGitHubアカウントを使用してログインできる。
- **ユーザー管理**: 初回ログイン時にユーザー情報をデータベース(`users` テーブル)に保存・更新する。
- **セッション/状態管理**: ログイン状態を維持し、フロントエンドでユーザー情報を利用可能にする。
- **保護されたルート**: 未認証ユーザーはチャット画面にアクセスできず、ログイン画面へ誘導される。

### 前提条件
- GitHub OAuth App が作成されていること (Client ID, Client Secret が必要)。
- 環境変数で機密情報を管理すること。

## 2. 設計 (Design)

### データベース設計 (`users` テーブル)
| Column | Type | Constraints | Description |
|Ref|Type|Constraints|Description|
|---|---|---|---|
| `id` | SERIAL | PRIMARY KEY | 内部ユーザーID |
| `github_id` | BIGINT | UNIQUE, NOT NULL | GitHubのユーザーID |
| `username` | VARCHAR | NOT NULL | GitHubのユーザー名 |
| `avatar_url` | VARCHAR | | アバター画像のURL |
| `created_at` | TIMESTAMP | DEFAULT NOW() | 作成日時 |

### API 設計
| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/auth/login` | GitHubのOAuth認証ページへリダイレクトする。 |
| `GET` | `/auth/callback` | GitHubからのコールバックを受け取り、トークン交換、ユーザー保存、セッションCookie設定を行い、フロントエンドへリダイレクトする。 |
| `GET` | `/auth/me` | 現在ログイン中のユーザー情報を返す (要セッション検証)。 |

### フロントエンド設計
- **`useAuth` Hook**:
    - `/auth/me` を定期的に、または初期ロード時に叩いて認証状態を確認。
    - `user`, `loading`, `login` (リダイレクト処理), `logout` を提供。
- **ルーティング**:
    - `/login`: ログインボタンを表示。
    - `/`: チャット画面 (要認証)。`useAuth` で未認証なら `/login` へリダイレクト。

## 3. 実装フェーズ (Implementation)

### Backend (Rust)
1.  **Migration**: `users` テーブル作成のSQLを作成し実行。
2.  **Dependencies**: `Cargo.toml` に `oauth2`, `reqwest`, `dotenv` (既存確認) を追加。
3.  **Models**: `User` 構造体、`NewUser` 構造体を `models.rs` に追加。
4.  **Handlers**: `handlers/auth.rs` を作成。
    - `login`: `oauth2` クレートを用いて認証URLを生成しリダイレクト。
    - `callback`: Codeを受け取り、GitHub APIを叩いてAccess TokenとUser情報を取得。DBにUpsertし、CookieをセットしてFrontend (`/`) へリダイレクト。
    - `get_me`: Cookie内のセッション情報(今回は簡易的にUserId署名付きなどを想定、あるいはもっとシンプルに)からユーザーを特定しJSONを返す。
        - *Note*: 今回は簡易実装として、Signed Cookie または シンプルなセッションIDを使用する想定。
5.  **Routes**: `main.rs` にルートを追加。

### Frontend (React/TypeScript)
1.  **API Client**: Cookieを含むリクエストを送るように `axios` インスタンスを設定 (`withCredentials: true`)。
2.  **Components**: `LoginButton.tsx` (GitHubログインボタン)。
3.  **Hooks**: `useAuth.ts` の実装。
4.  **Pages**: `LoginPage.tsx` の作成。
5.  **App**: ルーティング設定の変更 (`react-router-dom` 使用)。

## 4. テストフェーズ (Testing)

- **Backend**:
    - `auth_test.rs`: 認証フローの結合テストは難しい(外部依存)ため、モックサーバーを使うか、主要ロジック(DB保存など)の単体テストを行う。
    - 手動テスト: ブラウザで `/auth/login` にアクセスし、GitHub認証を経て正しくリダイレクトされるか確認。
- **Frontend**:
    - ログイン・未ログイン状態での表示切り替え確認。

## 5. デプロイ・設定 (Configuration)

- `.env` ファイルに以下を追加:
    - `GITHUB_CLIENT_ID`
    - `GITHUB_CLIENT_SECRET`
    - `GITHUB_REDIRECT_URL` (例: `http://localhost:3000/auth/callback`)
    - `FRONTEND_URL` (例: `http://localhost:5173`)

## 担当者と期日

- **担当者**: Gemini (Agent)
- **期日**: 2025-12-03 (Tomorrow)
