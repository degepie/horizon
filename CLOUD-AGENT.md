# Instructions for cloud Claude sessions working in degepie/horizon

This branch (`cloud-instructions`) holds only this file. It is not part of any pull request: never merge it into
another branch and never copy it into a PR. Read it at the start of every session:
`git fetch origin cloud-instructions && git show origin/cloud-instructions:CLOUD-AGENT.md`.

The repository is degepie's fork of https://github.com/horizon-rust/horizon (Minecraft Java 26.3 rewritten in Rust,
"vanilla is the specification"). Upstream is `horizon-rust/horizon`; add it as a remote named `upstream`.

## Hard rules

1. **Nothing public in upstream without degepie's explicit approval.** Don't open issues, comment, `/claim`,
   review, react or open pull requests in `horizon-rust/horizon`. Prepare the texts as files in your branch; the
   coordinator (degepie's local Claude session) shows them to degepie and publishes them.
2. **Push only to your own branches** in this fork, named `cloud/<slug>` (e.g. `cloud/fix-8848-raider-celebrate`).
   Never push to `main`, to `fix/*`, `feature/*`, `pr-assets` or any branch you didn't create: `fix/*` branches carry
   open upstream pull requests. No force-push.
3. **Never commit Minecraft's code or files** (no jar, no decompiled code, no extracted assets) — not even to this
   fork. Don't quote decompiled code in any text; name `Class.method` and describe the behaviour in your own words.
4. **Vanilla behaviour is the spec, but only what you checked counts.** If you couldn't check it in the vanilla
   jar or a vanilla game, write "expected from `Class.method`", never "vanilla does X" as an observation. Numbers
   only from measurements.
5. No secrets in commits or texts.

## Before you start a task

- Check prior work every time: `gh issue view N -R horizon-rust/horizon --json closedByPullRequestsReferences,assignees,labels,comments`;
  list open AND recently merged PRs directly (`gh pr list -R horizon-rust/horizon --state all -L 300 --json number,title,body,files`)
  — `gh search` lags. Check `git log upstream/main`. If someone claimed it, has a PR, or main already fixed it:
  stop and report.
- Upstream rules (CONTRIBUTING.md): an issue must be claimed (`/claim`) before work; at most 8 open claimed issues
  without a PR per account; a claim without a linked PR is released after 5 days.
- Base every branch on fresh `upstream/main`. Minimal diff: no drive-by refactors or reformatting; insert enum
  variants / match arms in their natural order.

## What you can and can't do in the cloud

- You can: read code, find root causes, write fixes and unit tests, run `cargo test`, `cargo clippy`, `cargo fmt`.
- You can't (no files, no GPU, no screen): the vanilla jar, `tools/rea_vanilla.py`, parity harnesses that need the
  jar, and in-game checks. Say so plainly in your report; the coordinator runs those on its machines.
- Good fits: advancement/predicate logic, commands, parsing, protocol encode/decode, data-driven registries —
  anything a unit test can prove. Leave tasks that need in-game proof to the coordinator.

## Deliverables (in your branch `cloud/<slug>`)

- The fix as commits (author degepie, message ending with the `Co-Authored-By:` line for your model).
- `cloud-notes/<slug>/ISSUE.md` — only if no issue exists. First line `<!-- Title: … -->`, then:
  `> 🤖 Opened by degepie's AI agent (Claude Code, <model>), not by degepie personally.`; minimal repro (a tiny data
  pack or commands), expected (with the `Class.method` you relied on), actual, Horizon file:line.
- `cloud-notes/<slug>/PR.md` — the PR body in the shape of upstream's `.github/pull_request_template.md`:
  `## What and why` (with `Fixes #N` or `Fixes #TBD`), `## Kind` (all options listed, exactly one ticked),
  `## Vanilla reference` (plus a line `No harness: …` when no parity harness applies), `## Other projects' code`
  (`none`), `## Self-check` (output of `python3 tools/pr_ready.py --quick --base upstream/main` if it runs here,
  else the commands you ran and their results). First line as in ISSUE.md. Last line
  `🤖 Generated with [Claude Code](https://claude.com/claude-code)`.
- `cloud-notes/<slug>/REPORT.md` — for the coordinator: prior-work result, root cause, what you verified and how,
  what you couldn't verify, overlaps with open PRs (files/functions), risks. Keep it short.
- The coordinator removes `cloud-notes/` before anything goes upstream; keep all notes inside that folder.
