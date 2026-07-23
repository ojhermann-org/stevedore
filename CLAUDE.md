# stevedore — Claude Code guidance

Rust workspace: a library that **moves secrets between password managers and
vaults**, a CLI, and an MCP server. See [`README.md`](README.md) for
orientation — its **Design & scope** section holds the decisions that fix the
shape (first route Dashlane → Proton; mover-not-resolver; no `Store` trait yet).

These rules layer on top of the global permission model (auto mode + classifier).
Their job is to tell the classifier what counts as **"important"** to delete or
create *in this repo*; the global config only guards universal catastrophes.

## The one rule that's non-negotiable: never leak a secret value

This tool exists to handle secret *values*. Everything else is secondary to not
spilling them.

- **Never print, log, or commit a secret value.** `SecretValue` redacts itself in
  `Debug`/`Display` by construction — don't add a code path that calls `.expose()`
  into a log line, an error message, a test assertion's failure output, or the
  terminal. Verify by length / equality, not by echoing plaintext.
- **Never delete, move, or print a git-ignored secret file** (`.envrc`,
  `.envrc.local`, `.env*`) or a vault export (`*.dashlane`, `exports/`). These may
  hold real credentials. `.gitignore` and `.githooks/pre-commit` guard them; keep
  it that way and don't `git add -f` around the guard.
- Store credentials for a run are supplied deliberately (flags/prompts/env for one
  invocation), never loaded ambiently in `.envrc.shared`.

## Keep documentation current

Documentation is part of the change, not a follow-up. Before opening a PR, check
whether it touches user-facing behavior, the public API, or a design decision,
and update the affected docs in the same PR:

- **Workspace `README.md`** and **per-crate `README.md`** — usage, the store/route
  table, status.
- **Crate-level rustdoc** (`//!`) and item docs — the docs.rs front page.
- **CLI `--help`** (clap doc comments) and, once it exists, **MCP tool
  descriptions** — these *are* the docs for those surfaces.
- **Design decisions** — this repo keeps **no separate decision log (no ADRs)**.
  A non-trivial design/process decision goes into the `README.md` **Design &
  scope** section (and `CLAUDE.md` if it's a working rule); durable context that
  isn't repo-shaped is worth a Claude Code **memory**.

## Comments: let the code speak first

Keep comments minimal and purposeful. The **code, tests, and documentation**
should carry the meaning; a comment earns its place only when it adds something
they can't.

- **Doc comments (`///`, `//!`) stay** — they're the documentation surface
  (rustdoc, docs.rs, `--help`). Keep them, but tight.
- **Cut narration.** Don't restate what the next line plainly does. Reach first
  for a clearer name, a smaller function, or a test — not a comment that explains
  code that could explain itself.
- **Keep the "why."** A comment capturing non-obvious *intent* — a constraint, a
  footgun, the reason behind a choice (e.g. the `SecretValue` redaction contract,
  the `ci`-job-name-must-match-the-check note) — is exactly what belongs in a
  comment, because it isn't recoverable from the code.

## Repo & release conventions

- **Everything lands through a PR** — `main` is branch-protected (requires the
  `ci` check + a PR; no direct commits). Squash-merge → delete branch → `git up`.
  Agent merge authority for this repo is granted in the global `~/.claude/CLAUDE.md`.
- **Repo-level settings are code.** The branch ruleset is
  `.github/rulesets/main.json`, reconciled by `scripts/settings.sh`
  (`--check` / `--apply`, owner-run). Org-wide settings live in
  `ojhermann-org/github-settings`. The two layers compose.
- **Nothing is published yet.** Every crate is `publish = false`. Cutting a
  release — flipping that, versioning, tagging, wiring release-plz / crates.io —
  is the owner's call, not ordinary development.
- **The MCP surface is deferred** (see README **Design & scope**). When it lands,
  release MCP-surface changes promptly and on their own (the `ferric-fred`
  discipline), because listing/scoring builds from `main`.

## Deletion & creation (what's sensitive here)

**Ask before deleting or substantively rewriting:**

- **The `README.md` "Design & scope" section.** It's the repo's decision history
  now that there are no ADRs — revise a decision deliberately (and record what
  changed), don't quietly gut it. Typo/link fixes are fine.
- **Lockfiles (`flake.lock`, `Cargo.lock`).** Regenerate through tooling
  (`nix flake update`, `cargo update`) — never hand-delete.
- **Tracked env config (`.envrc.shared`, `.envrc.example`)** and
  **`scripts/settings.sh`** — changing how the dev env or repo settings load
  affects everyone; confirm first.
- **`flake.nix`, `README.md`, `.github/rulesets/main.json`.**

**When creating:**

- **New design decision:** record it in the `README.md` **Design & scope**
  section (no separate ADR file) — and in `CLAUDE.md` / a memory if it's a working
  rule or durable context.
- **New crate:** follow the workspace layout in the README **Design & scope**
  section — consumers depend on the library by workspace path; keep that
  compile-time coupling intact.
- **New store (source or sink):** it's a concrete module for now — do **not**
  introduce a `Store` trait to add store #2. The abstraction waits for #3/#4
  (README **Design & scope**).
