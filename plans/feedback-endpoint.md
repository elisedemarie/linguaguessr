# Plan: Feedback Endpoint (Phase 6.2)

**Branch**: feat/feedback-endpoint
**Status**: Active

## Goal

Add `POST /api/feedback` to the backend so users can submit feedback that opens a GitHub Issue tagged `feedback`.

## Context

- Backend: Axum 0.7, existing `WikipediaClient` trait + `reqwest` impl pattern to follow
- GitHub Issues API: `POST https://api.github.com/repos/elisedemarie/linguaguessr/issues`
- Token available at runtime via `GITHUB_FEEDBACK_TOKEN` env var (already in systemd service)
- `wiremock` is already a dev-dependency — use it to mock the GitHub API in tests
- Repo and label are fixed constants (`elisedemarie/linguaguessr`, `feedback`)

## Request shape

```json
{
  "message": "string (required, non-empty)",
  "email":    "string (optional)",
  "game_id":  "uuid  (optional)",
  "round_id": "uuid  (optional)"
}
```

## Acceptance Criteria

- [ ] `POST /api/feedback` with valid message returns 201
- [ ] Empty message returns 422
- [ ] Missing message field returns 422
- [ ] Issue title is `[Feedback] <first 60 chars of message>`
- [ ] Issue body includes full message, email if provided, game/round context if provided
- [ ] Issue is created with the `feedback` label
- [ ] GitHub API failure returns 502
- [ ] Missing `GITHUB_FEEDBACK_TOKEN` returns 502
- [ ] Route is registered and reachable

## Slices

Every slice follows RED-GREEN-MUTATE-KILL MUTANTS-REFACTOR.

---

### Slice 1: GitHub issue formatting logic

**Value**: The issue title and body are correct before we wire up any HTTP.

**Path**: Pure functions `format_title(message: &str) -> String` and `format_body(req: &FeedbackRequest) -> String` in a new `feedback.rs` module. No I/O, fully unit-testable.

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring`.

**Acceptance criteria**:
- Title truncates message at 60 chars with `[Feedback]` prefix
- Title uses the full message when ≤ 60 chars
- Body always contains the full message
- Body includes email line only when email is `Some`
- Body includes game_id line only when game_id is `Some`
- Body includes round_id line only when round_id is `Some`

**RED**: Write failing tests for all cases above.
**GREEN**: Implement `format_title` and `format_body`.
**MUTATE**: Run `cargo mutants -p backend --file backend/src/feedback.rs`.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess only if valuable.
**Done when**: All criteria met, mutation report reviewed, human approves commit.

---

### Slice 2: GitHubClient trait + reqwest implementation

**Value**: The HTTP call to GitHub is injectable and testable via wiremock.

**Path**: `GitHubClient` trait with `create_issue` method → `ReqwestGitHubClient` impl → added to `AppState`.

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring`.

**Acceptance criteria**:
- `GitHubClient` trait has `async fn create_issue(&self, title: &str, body: &str) -> Result<(), GitHubError>`
- `ReqwestGitHubClient` sends correct JSON to GitHub API with Bearer auth header
- `AppState` carries `github: Arc<dyn GitHubClient>`
- `build_router` wired accordingly

**RED**: Write a wiremock test that verifies the correct JSON payload and auth header reach the mock server.
**GREEN**: Implement the trait and reqwest client.
**MUTATE**: Run cargo mutants scoped to the new file.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess only if valuable.
**Done when**: All criteria met, mutation report reviewed, human approves commit.

---

### Slice 3: POST /api/feedback handler

**Value**: The full endpoint is reachable and behaves correctly end-to-end.

**Path**: `post_feedback` handler in `handlers.rs` → registered on router as `POST /api/feedback`.

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring`.

**Acceptance criteria**:
- Valid request → `GitHubClient::create_issue` called with correct title/body → 201
- Empty message → 422, GitHub not called
- Missing message → 422, GitHub not called
- GitHub client returns error → 502
- Missing token (empty string) → 502, GitHub not called

**RED**: Write failing handler tests using a mock `GitHubClient`.
**GREEN**: Implement `post_feedback` handler, register route.
**MUTATE**: Run cargo mutants scoped to handler.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess only if valuable.
**Done when**: All criteria met, mutation report reviewed, human approves commit.

---

## Pre-PR Quality Gate

1. `cargo test --workspace` passes
2. `cargo check` passes
3. Manual smoke test: `curl -X POST https://api.linguaguessr.io/api/feedback -H 'Content-Type: application/json' -d '{"message":"test feedback"}'` → check GitHub Issues for the created issue

---

*Delete this file when the plan is complete. If `plans/` is empty, delete the directory.*
