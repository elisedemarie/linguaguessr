# Plan: Deploy to Render + Cloudflare Pages

**Status**: Active

## Architecture

```
Browser → Cloudflare Pages (frontend WASM, free)
        → Render Web Service (backend Axum, free tier)
              → Wikipedia API
```

## Code changes needed (done in this plan)

- [ ] Backend reads `PORT` from env var (Render injects this — hardcoded 3000 won't work)
- [ ] `BACKEND_URL` in frontend uses `option_env!("BACKEND_URL")` with localhost fallback
- [ ] `Dockerfile` for backend (Render needs this to build and run it)
- [ ] `.dockerignore` (keeps builds fast)

## What you do manually (step by step)

### Step 1 — Deploy backend to Render

1. Go to [render.com](https://render.com) and sign up / log in with GitHub
2. Click **New → Web Service**
3. Connect your `linguaguessr` GitHub repo
4. Settings:
   - **Name**: `linguaguessr-backend` (or whatever you like)
   - **Runtime**: Docker
   - **Branch**: `main`
   - **Dockerfile path**: `./Dockerfile`
5. Under **Environment Variables**, add:
   - `RUST_LOG` = `info`
6. Click **Create Web Service**
7. Wait for the build (~5 min first time — Rust is slow to compile)
8. Copy the URL Render gives you — looks like `https://linguaguessr-backend.onrender.com`

### Step 2 — Deploy frontend to Cloudflare Pages

1. Go to [pages.cloudflare.com](https://pages.cloudflare.com) and sign up / log in
2. Click **Create a project → Connect to Git**
3. Connect your `linguaguessr` GitHub repo
4. Build settings:
   - **Framework preset**: None
   - **Build command**: `curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable && source $HOME/.cargo/env && rustup target add wasm32-unknown-unknown && cargo install trunk && cd frontend && trunk build --release`
   - **Build output directory**: `frontend/dist`
5. Under **Environment Variables**, add:
   - `BACKEND_URL` = `https://linguaguessr-backend.onrender.com` (your Render URL from Step 1)
6. Click **Save and Deploy**
7. Wait for first build (~10 min — installs Rust toolchain + compiles WASM)

### Step 3 — Lock down CORS (optional but good)

Once you have the Cloudflare Pages URL (e.g. `https://linguaguessr.pages.dev`), update the
backend CORS from `permissive` to only allow that origin. Add a `FRONTEND_URL` env var in Render
and read it in `main.rs`. Do this after both services are confirmed working.

## Slices

### Slice 1: Code changes (no manual steps)

- Update `backend/src/main.rs` to bind on `$PORT`
- Update `frontend/src/lib.rs` to use `option_env!("BACKEND_URL")`
- Add `Dockerfile`
- Add `.dockerignore`

### Slice 2: You deploy (manual)

Follow the steps above. No code changes.

---
*Delete this file when deployed and working.*
