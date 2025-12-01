# Quickstart Guide

## Prerequisites

- Rust 1.75+
- Node.js 20+
- GitHub OAuth App (Client ID & Secret)

## Setup

1. **Clone & Install**
   ```bash
   # Backend
   cd backend
   cargo build
   
   # Frontend
   cd ../frontend
   npm install
   ```

2. **Environment Variables**
   Create `backend/.env`:
   ```env
   DATABASE_URL=sqlite:data.db
   GITHUB_CLIENT_ID=your_client_id
   GITHUB_CLIENT_SECRET=your_client_secret
   HOST=127.0.0.1
   PORT=3000
   FRONTEND_URL=http://localhost:5173
   ```

   Create `frontend/.env`:
   ```env
   VITE_API_URL=http://localhost:3000
   ```

3. **Database Setup**
   ```bash
   cd backend
   # Install sqlx-cli if needed: cargo install sqlx-cli
   sqlx database create
   sqlx migrate run
   ```

## Running

1. **Start Backend**
   ```bash
   cd backend
   cargo run
   ```

2. **Start Frontend**
   ```bash
   cd frontend
   npm run dev
   ```

3. **Access**
   Open `http://localhost:5173` in your browser.
