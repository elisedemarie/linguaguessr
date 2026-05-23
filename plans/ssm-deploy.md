# Plan: Switch CI/CD Deploy to AWS SSM Session Manager

## Why

The current `deploy-backend` job SSHs to EC2 from GitHub Actions runners. Those runners use
a rotating pool of IPs, so port 22 must be open to the world to make SSH work — which exposes
OpenSSH to brute-force attempts and any future OpenSSH CVEs.

SSM Session Manager solves this cleanly:
- No inbound rules needed — port 22 stays locked to your personal IPs only
- Auth is IAM (AWS access key stored in GitHub secrets), not an SSH key
- SSM agent ships with Ubuntu 24.04 and is already running on the instance
- Port 3000 stays closed (Nginx handles public traffic)

---

## Phase 1 — IAM role for EC2

### 1.1 Create the IAM role

In AWS Console → IAM → Roles → Create role:
- Trusted entity: **AWS service → EC2**
- Attach policy: `AmazonSSMManagedInstanceCore`
- Role name: `linguaguessr-ec2-ssm-role`

### 1.2 Attach the role to the EC2 instance

AWS Console → EC2 → Instances → select instance → Actions → Security → Modify IAM role
→ select `linguaguessr-ec2-ssm-role` → Update IAM role

No reboot needed; the role takes effect within a few minutes.

### 1.3 Verify SSM agent is running

SSH in from your personal machine (port 22 is still open to your IPs):
```bash
sudo systemctl status snap.amazon-ssm-agent.amazon-ssm-agent.service
```
Should be `active (running)`. If not:
```bash
sudo snap install amazon-ssm-agent --classic
sudo systemctl enable --now snap.amazon-ssm-agent.amazon-ssm-agent.service
```

### 1.4 Verify the instance appears in SSM

AWS Console → Systems Manager → Fleet Manager → your instance should appear.
Or via CLI: `aws ssm describe-instance-information --region ap-southeast-2`

---

## Phase 2 — IAM user for GitHub Actions

### 2.1 Create a deploy IAM user

AWS Console → IAM → Users → Create user:
- Username: `linguaguessr-github-deploy`
- No console access

### 2.2 Attach an inline policy

Attach the following least-privilege inline policy (allows only `ssm:SendCommand` on this
one instance, and `ssm:GetCommandInvocation` to poll for the result):

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "ssm:SendCommand"
      ],
      "Resource": [
        "arn:aws:ec2:ap-southeast-2:<account-id>:instance/<instance-id>",
        "arn:aws:ssm:ap-southeast-2::document/AWS-RunShellScript"
      ]
    },
    {
      "Effect": "Allow",
      "Action": [
        "ssm:GetCommandInvocation"
      ],
      "Resource": "*"
    }
  ]
}
```

Replace `<account-id>` and `<instance-id>` with real values.

### 2.3 Generate an access key

IAM → Users → `linguaguessr-github-deploy` → Security credentials → Create access key
→ Use case: Application running outside AWS → save the key ID and secret.

---

## Phase 3 — GitHub secrets

In the repo Settings → Secrets → Actions, add:
- `AWS_ACCESS_KEY_ID` — key ID from step 2.3
- `AWS_SECRET_ACCESS_KEY` — secret from step 2.3
- `EC2_INSTANCE_ID` — the EC2 instance ID (e.g. `i-0abc123def456`)

The existing `EC2_HOST`, `EC2_USER`, `EC2_DEPLOY_KEY` secrets can be removed once the new
workflow is confirmed working.

---

## Phase 4 — Update the GitHub Actions workflow

Replace the `deploy-backend` job in `.github/workflows/ci.yml`:

```yaml
deploy-backend:
  runs-on: ubuntu-latest
  needs: test
  if: github.ref == 'refs/heads/main'
  steps:
    - name: Configure AWS credentials
      uses: aws-actions/configure-aws-credentials@v4
      with:
        aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
        aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
        aws-region: ap-southeast-2

    - name: Deploy via SSM
      run: |
        COMMAND_ID=$(aws ssm send-command \
          --instance-ids "${{ secrets.EC2_INSTANCE_ID }}" \
          --document-name "AWS-RunShellScript" \
          --parameters 'commands=["/home/ubuntu/deploy.sh"]' \
          --query "Command.CommandId" \
          --output text)

        echo "SSM command ID: $COMMAND_ID"

        aws ssm wait command-executed \
          --command-id "$COMMAND_ID" \
          --instance-id "${{ secrets.EC2_INSTANCE_ID }}"

        aws ssm get-command-invocation \
          --command-id "$COMMAND_ID" \
          --instance-id "${{ secrets.EC2_INSTANCE_ID }}" \
          --query "StandardOutputContent" \
          --output text
```

`aws ssm wait command-executed` polls until the command finishes (or times out after ~60 attempts).
The final `get-command-invocation` prints the deploy script output to the Actions log.

---

## Phase 5 — Cleanup

Once the new deploy is confirmed working on a push to main:

1. Remove the restricted key from EC2 `~/.ssh/authorized_keys` (the `command=...` line for the deploy key)
2. Delete the `EC2_DEPLOY_KEY`, `EC2_HOST`, `EC2_USER` GitHub secrets
3. The port 22 security group rule stays as-is (your personal IPs only) — that's the goal

---

## Security posture after this change

| Vector | Before | After |
|--------|--------|-------|
| Port 22 | Open to world (needed for GH Actions) | Locked to your IPs only |
| Deploy auth | SSH private key in GH secrets | IAM key scoped to one SSM action on one instance |
| What the CI credential can do | Run any `authorized_keys` command | Run `/home/ubuntu/deploy.sh` only, on this one instance |
| OpenSSH attack surface | World-reachable | Not reachable from internet |
