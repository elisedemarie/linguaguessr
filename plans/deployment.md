# Deployment Plan: EC2 + GitHub Pages + CI/CD

## Overview

Move off Render to a self-managed AWS stack. Backend on EC2, frontend on GitHub Pages,
CI/CD via GitHub Actions. Sentry for error logging, GitHub Issues (via bot account) for
user feedback. Domain purchased: linguaguessr.io via Cloudflare ($50/year).

**Progress:** Phase 1 ✅ | 2.1 ✅ | 2.2 ✅ | 2.3–2.4 next

---

## Phase 1 — AWS Account + EC2

### 1.1 Create AWS account
- Sign up at aws.amazon.com
- Enable MFA on the root account immediately
- Create an IAM user for day-to-day use (never use root credentials again)
- Set a billing alert at $20/month (CloudWatch → Billing alarm) so you catch runaway costs early

### 1.2 Allocate an Elastic IP
- EC2 → Elastic IPs → Allocate
- Note the IP — this is what the frontend will bake in as `BACKEND_URL`
- Elastic IPs are free while associated with a running instance; $0.005/hr if unassociated, so don't leave it dangling

### 1.3 Launch EC2 instance
- Region: ap-southeast-2 (Sydney)
- AMI: Ubuntu 24.04 LTS
- Instance type: t3.micro (free tier eligible for 12 months, ~$8-12/month after)
- Storage: 8GB gp3 (default)
- Security group — inbound rules:
  - SSH (22) from your IP only (not 0.0.0.0/0)
  - HTTP (80) from anywhere
  - HTTPS (443) from anywhere — for when domain + SSL is added
- Associate the Elastic IP to the instance

### 1.4 Generate a deploy keypair (do not use your personal SSH key)
```bash
ssh-keygen -t ed25519 -C "linguaguessr-deploy" -f ~/.ssh/linguaguessr_deploy
```
- Add the public key to EC2 `~/.ssh/authorized_keys`
- Restrict it to the deploy command only (see Phase 3)
- Keep the private key for the GitHub Actions secret

---

## Phase 2 — Backend on EC2

### 2.1 Initial server setup
```bash
ssh -i ~/.ssh/linguaguessr_deploy ubuntu@<elastic-ip>
sudo apt update && sudo apt upgrade -y
sudo apt install -y build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2.2 Clone and build
```bash
git clone https://github.com/elisedemarie/linguaguessr.git
cd linguaguessr
cargo build --release -p backend
```

### 2.3 Systemd service
Create `/etc/systemd/system/linguaguessr.service`:
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
Environment=FRONTEND_URL=https://elisedemarie.github.io
Environment=SENTRY_DSN=<your-sentry-dsn>
Environment=GITHUB_FEEDBACK_TOKEN=<bot-account-pat>

[Install]
WantedBy=multi-user.target
```
```bash
sudo systemctl enable linguaguessr
sudo systemctl start linguaguessr
```

### 2.4 Sentry integration
- Create account at sentry.io, new Rust project
- Add to `backend/Cargo.toml`:
  ```toml
  sentry = "0.48"
  ```
- Initialise in `backend/src/main.rs`:
  ```rust
  let _guard = sentry::init((
      std::env::var("SENTRY_DSN").unwrap_or_default(),
      sentry::ClientOptions {
          release: sentry::release_name!(),
          ..Default::default()
      },
  ));
  ```
- `SENTRY_DSN` lives as an env var in the systemd service file (not in source)

---

## Phase 3 — GitHub Bot Account + Feedback Endpoint

### 3.1 Create bot account
- New GitHub account: e.g. `linguaguessr-bot`
- Add as collaborator on `linguaguessr` repo with **Triage** role (can create issues, cannot push code)
- Generate a **fine-grained PAT** from the bot account:
  - Repository access: `linguaguessr` only
  - Permissions: Issues → Read and Write. Nothing else.
- Store as `GITHUB_FEEDBACK_TOKEN` in the systemd service file

### 3.2 Feedback endpoint on backend
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

Handler opens a GitHub Issue on `elisedemarie/linguaguessr` via the API:
- Title: `[Feedback] <first 60 chars of message>`
- Body: full message + email if provided + game/round context
- Label: `feedback` (create this label on the repo beforehand)

### 3.3 Frontend footer
Small fixed bar at the bottom of every screen:
- Left: GitHub repo link
- Right: "Report an issue" → opens a small modal with free-text field + optional email + submit
- Modal submits to `POST /api/feedback` with current game_id/round_id attached if mid-game

---

## Phase 4 — Frontend on GitHub Pages

### 4.1 Build configuration
`BACKEND_URL` baked in at compile time pointing at the Elastic IP:
```
BACKEND_URL=http://<elastic-ip>:3000
```
Once domain + SSL is set up this becomes `https://linguaguessr.io`.

### 4.2 GitHub Pages setup
- Repo Settings → Pages → Source: GitHub Actions
- The CI workflow (Phase 5) builds the WASM frontend and deploys to GitHub Pages
- Pages URL: `https://elisedemarie.github.io/linguaguessr`

### 4.3 CORS
- Update `FRONTEND_URL` env var on EC2 to `https://elisedemarie.github.io`
- Backend already reads this for CORS config

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
- `EC2_HOST` — the Elastic IP
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

  build-frontend:
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - uses: Swatinem/rust-cache@v2
      - run: cargo install trunk
      - run: trunk build --release
        working-directory: frontend
        env:
          BACKEND_URL: http://${{ secrets.EC2_HOST }}:3000
      - uses: actions/upload-pages-artifact@v3
        with:
          path: frontend/dist

  deploy-frontend:
    runs-on: ubuntu-latest
    needs: build-frontend
    if: github.ref == 'refs/heads/main'
    permissions:
      pages: write
      id-token: write
    environment:
      name: github-pages
    steps:
      - uses: actions/deploy-pages@v4

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

---

## Phase 6 — Domain (linguaguessr.io — purchased via Cloudflare)

- Cloudflare DNS → add A record: `linguaguessr.io` → Elastic IP
- Add CNAME: `www` → `linguaguessr.io`
- Enable Cloudflare proxy (orange cloud) for DDoS protection
- Update `BACKEND_URL` GitHub Actions secret to `https://linguaguessr.io`
- Update `FRONTEND_URL` env var on EC2 to `https://linguaguessr.io`
- Add SSL on EC2 with certbot / Let's Encrypt
- GitHub Pages custom domain: repo Settings → Pages → Custom domain → `linguaguessr.io`

---

## Future phases (not now)

- **Second EC2 + ALB** — only when you need horizontal scale
- **DynamoDB/RDS** — only when you need persistence beyond in-memory sessions
- **ECS/Kubernetes** — only when EC2 management becomes genuinely painful
- **CloudFront + S3** — natural upgrade from GitHub Pages if you need edge caching

---

## Cost estimate (Sydney region, small scale)

| Item | Cost |
|------|------|
| t3.micro EC2 | ~$10/month |
| Elastic IP (associated) | Free |
| GitHub Pages | Free |
| GitHub Actions (public repo) | Free |
| Sentry (free tier) | Free |
| Data transfer (low volume) | ~$1-2/month |
| linguaguessr.io domain (Cloudflare) | ~$4/month ($50/year) |
| **Total** | **~$16/month** |
