# Deployment Plan: EC2 + Cloudflare Pages + CI/CD

## Overview

Backend on EC2, frontend on Cloudflare Pages (already running — no migration needed),
CI/CD via GitHub Actions. Sentry for error logging, GitHub Issues (via bot account) for
user feedback. Domain: linguaguessr.io purchased via Cloudflare ($50/year).

- `linguaguessr.io` → Cloudflare Pages (frontend, custom domain)
- `api.linguaguessr.io` → EC2 Elastic IP (backend, behind Nginx + SSL)

**Progress:** Phase 1 ✅ | Phase 2 ✅ | Phase 3 next (domain + SSL + Nginx)

---

## Phase 1 — AWS Account + EC2 ✅

### 1.1 Create AWS account ✅
- MFA enabled on root account
- Billing alert set at $20/month

### 1.2 Allocate an Elastic IP ✅
- Elastic IP associated with instance: `13.55.116.110`
- Free while associated; $0.005/hr if unassociated — don't leave it dangling

### 1.3 Launch EC2 instance ✅
- Region: ap-southeast-2 (Sydney)
- AMI: Ubuntu 24.04 LTS
- Instance type: t3.micro
- Security group inbound rules:
  - SSH (22) from specific IPs only
  - HTTP (80) from anywhere
  - HTTPS (443) from anywhere
  - Custom TCP (3000) from anywhere — temporary, removed once Nginx is in front

### 1.4 Deploy keypair ✅
- `~/.ssh/linguaguessr_deploy` — public key on EC2, private key for GitHub Actions

---

## Phase 2 — Backend on EC2 ✅

### 2.1 Server setup ✅
- Rust, build-essential, pkg-config, libssl-dev installed

### 2.2 Clone and build ✅
```bash
git clone https://github.com/elisedemarie/linguaguessr.git
cd linguaguessr
cargo build --release -p backend
```

### 2.3 Systemd service ✅
`/etc/systemd/system/linguaguessr.service`:
```ini
[Unit]
Description=LinguaGuessr backend
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/home/ubuntu/linguaguessr
ExecStart=/home/ubuntu/linguaguessr/target/release/backend
Restart=on-failure
RestartSec=5
Environment=FRONTEND_URL=https://linguaguessr.io
Environment=SENTRY_DSN=<sentry-dsn>
Environment=GITHUB_FEEDBACK_TOKEN=<bot-account-pat>

[Install]
WantedBy=multi-user.target
```

### 2.4 Sentry integration ✅
- `sentry = "0.48"` in `backend/Cargo.toml`
- Initialised in `backend/src/main.rs` — DSN read from `SENTRY_DSN` env var at runtime

---

## Phase 3 — Domain + Nginx + SSL

### 3.1 DNS in Cloudflare
- Add A record: `api.linguaguessr.io` → `13.55.116.110` (proxy OFF — orange cloud off, grey cloud)
  - Proxy must be off for Let's Encrypt cert validation to work
- Add CNAME: `www` → `linguaguessr.io`

### 3.2 Install Nginx on EC2
```bash
sudo apt install -y nginx
```

Create `/etc/nginx/sites-available/linguaguessr`:
```nginx
server {
    listen 80;
    server_name api.linguaguessr.io;

    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```
```bash
sudo ln -s /etc/nginx/sites-available/linguaguessr /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 3.3 SSL with Let's Encrypt
```bash
sudo apt install -y certbot python3-certbot-nginx
sudo certbot --nginx -d api.linguaguessr.io
```
Certbot will auto-update the Nginx config for HTTPS and set up auto-renewal.

### 3.4 Lock down port 3000
Once Nginx is in front and HTTPS is working:
- EC2 security group → remove the port 3000 inbound rule
- Backend is now only reachable via Nginx on 443

### 3.5 Update env vars
Update the systemd service file:
```ini
Environment=FRONTEND_URL=https://linguaguessr.io
```
```bash
sudo systemctl daemon-reload && sudo systemctl restart linguaguessr
```

---

## Phase 4 — Frontend Custom Domain (Cloudflare Pages)

### 4.1 Add custom domain in Cloudflare Pages
- Cloudflare dashboard → Pages → linguaguessr project → Custom domains
- Add `linguaguessr.io` — Cloudflare will configure DNS automatically since the domain is also in Cloudflare

### 4.2 Update BACKEND_URL
In Cloudflare Pages → Settings → Environment variables:
```
BACKEND_URL=https://api.linguaguessr.io
```
Trigger a new deploy for it to take effect.

### 4.3 Verify end-to-end
- Visit `https://linguaguessr.io` — frontend loads
- Start a game — requests go to `https://api.linguaguessr.io`
- Check Sentry dashboard for any errors

---

## Phase 5 — CI/CD with GitHub Actions

### 5.1 Restricted deploy key on EC2
In EC2 `~/.ssh/authorized_keys`, restrict the deploy key to one script only:
```
command="/home/ubuntu/deploy.sh",no-port-forwarding,no-X11-forwarding,no-agent-forwarding <deploy-public-key>
```

`/home/ubuntu/deploy.sh`:
```bash
#!/bin/bash
set -e
cd /home/ubuntu/linguaguessr
git pull origin main
cargo build --release -p backend
sudo systemctl restart linguaguessr
```

### 5.2 GitHub Actions secrets
Add to repo Settings → Secrets → Actions:
- `EC2_HOST` — `13.55.116.110`
- `EC2_USER` — `ubuntu`
- `EC2_DEPLOY_KEY` — contents of `~/.ssh/linguaguessr_deploy` (private key)

### 5.3 Workflow: `.github/workflows/ci.yml`
```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace

  deploy-backend:
    runs-on: ubuntu-latest
    needs: test
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: webfactory/ssh-agent@v0.9.0
        with:
          ssh-private-key: ${{ secrets.EC2_DEPLOY_KEY }}
      - run: |
          ssh -o StrictHostKeyChecking=no ${{ secrets.EC2_USER }}@${{ secrets.EC2_HOST }}
```

Note: Cloudflare Pages handles its own frontend CI/CD — it auto-deploys on push to main. No GitHub Actions needed for the frontend.

---

## Phase 6 — GitHub Bot Account + Feedback Endpoint

### 6.1 Create bot account
- New GitHub account: e.g. `linguaguessr-bot`
- Add as collaborator with **Triage** role (can create issues, cannot push code)
- Generate a fine-grained PAT:
  - Repository access: `linguaguessr` only
  - Permissions: Issues → Read and Write. Nothing else.
- Store as `GITHUB_FEEDBACK_TOKEN` in the systemd service file

### 6.2 Feedback endpoint on backend
New route: `POST /api/feedback`

Request payload:
```json
{
  "message": "string (required)",
  "email": "string (optional)",
  "game_id": "uuid (optional)",
  "round_id": "uuid (optional)"
}
```

Handler opens a GitHub Issue on `elisedemarie/linguaguessr`:
- Title: `[Feedback] <first 60 chars of message>`
- Body: full message + email if provided + game/round context
- Label: `feedback` (create this label on the repo beforehand)

### 6.3 Frontend footer
Small fixed bar at the bottom of every screen:
- Left: GitHub repo link
- Right: "Report an issue" → opens a modal with free-text + optional email + submit
- Modal submits to `POST /api/feedback` with current game_id/round_id if mid-game

---

## Future phases (not now)

- **Second EC2 + ALB** — only when you need horizontal scale
- **DynamoDB/RDS** — only when you need persistence beyond in-memory sessions
- **ECS/Kubernetes** — only when EC2 management becomes genuinely painful
- **CloudFront + S3** — natural upgrade if you need more edge caching than Cloudflare Pages provides

---

## Cost estimate (Sydney region, small scale)

| Item | Cost |
|------|------|
| t3.micro EC2 | ~$10/month |
| Elastic IP (associated) | Free |
| Cloudflare Pages | Free |
| GitHub Actions (public repo) | Free |
| Sentry (free tier) | Free |
| Data transfer (low volume) | ~$1-2/month |
| linguaguessr.io domain (Cloudflare) | ~$4/month ($50/year) |
| **Total** | **~$16/month** |
