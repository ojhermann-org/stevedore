# stevedore — Claude Code guidance

Rust workspace: a library that **moves secrets between password managers and
vaults**, a CLI, and an MCP server. [`README.md`](README.md) is the user-facing
orientation; the decisions that fix the shape (first route Dashlane → Proton;
mover-not-resolver; no `Store` trait yet) are recorded in **this file and the
project memories**, not the README. The Dashlane source's one-time `dcli` setup
(personal auth) is documented in [`docs/dcli/`](docs/dcli/).

These rules layer on top of the global permission model (auto mode + classifier).
Their job is to tell the classifier what counts as **"important"** to delete or
create *in this repo*; the global config only guards universal catastrophes.

## Vocabulary: store, source, sink

Three deliberately distinct words — don't collapse them:

- **store** — the general noun for a password manager or vault stevedore talks
  to (Dashlane, Proton Pass, …). Use it in **role-neutral** contexts: the error
  type (`error.rs` is "store-neutral" because it serves both reading and
  writing), the CLI `stores` listing, the README's Stores table.
- **source** — a store in the **read** role (stevedore reads secrets *from* it).
  Use it for read-side prose — e.g. `docs/security.md` is about reading, so it
  says "source".
- **sink** — a store in the **write** role (stevedore writes secrets *to* it).

`source`/`sink` are the two *roles*; `store` is the umbrella. A blanket
find-and-replace of one for another is wrong: "the store is not authenticated"
must stay `store` because a sink can need auth too, while a doc about reading
should say `source`. Match the word to whether the context is reading, writing,
or neutral.

## The one rule that's non-negotiable: never leak a secret value

This tool exists to handle secret *values*. Everything else is secondary to not
spilling them. The user-facing explanation of how this is enforced lives in
[`docs/security.md`](docs/security.md); keep it current when the mechanisms
change.

- **Never print, log, or commit a secret value.** `SecretValue` redacts itself in
  `Debug`/`Display` by construction — don't add a code path that calls `.expose()`
  into a log line, an error message, a test assertion's failure output, or the
  terminal. Verify by length / equality, not by echoing plaintext.
- **Never delete, move, or print a git-ignored secret file** (`.envrc`,
  `.envrc.local`, `.env*`) or a vault export (`*.dashlane`, `exports/`). These may
  hold real credentials. `.gitignore` and `.githooks/pre-commit` guard them; keep
  it that way and don't `git add -f` around the guard.
- **Never pass a secret to a store CLI as a command-line argument.** A process's
  arguments are readable by other processes while it runs, so a secret goes on
  **stdin** — `crate::cli::run_with_stdin` is the only channel. This is a
  constraint on which store operations are available at all, not just on how they
  are called: Proton Pass items are *created* (`--from-template -` reads stdin)
  and never *updated*, because `pass-cli item update` takes values as `--field`
  arguments. When adding a store, find its stdin door first; if it has none, that
  operation doesn't land.
- Store credentials for a run are supplied deliberately (flags/prompts/env for one
  invocation), never loaded ambiently in `.envrc.shared`.

## Keep documentation current

Documentation is part of the change, not a follow-up. Before opening a PR, check
whether it touches user-facing behavior, the public API, or a design decision,
and update the affected docs in the same PR.

**User-facing docs carry no developer notes.** The workspace and per-crate
`README.md`s, `docs/`, rustdoc, and `--help` are clear, succinct, and
*exclusively* about what stevedore offers and how to use it. Keep design
rationale, project sequencing, what a thing "waits for", and which external
subcommands get invoked internally out of them — that belongs in this file or a
memory. When a fact is useful but reads as rationale, keep the fact and cut the
reasoning: say attachments cannot be read, not why.

- **Workspace `README.md`** and **per-crate `README.md`** — usage, the store/route
  table, status.
- **Crate-level rustdoc** (`//!`) and item docs — the docs.rs front page.
- **CLI `--help`** (clap doc comments) and, once it exists, **MCP tool
  descriptions** — these *are* the docs for those surfaces.
- **Store CLI docs (`docs/dcli/`)** — how a user sets up an external store CLI
  stevedore drives. These track a third-party tool that drifts, so keep them tight
  and defer the canonical flow to the vendor's own docs. Name a store CLI's
  subcommands **in full** everywhere — docs, code, commit messages — so
  `dcli password`, never `dcli p`. The short aliases are incomplete (`secret`,
  `status`, and `read` have none), so full names are the only convention that
  applies uniformly.
- **Design decisions** — this repo keeps **no separate decision log (no ADRs)**,
  and the `README.md` is **user-facing**, not a decision record. A non-trivial
  design/process decision goes into `CLAUDE.md` (working rules) and/or a Claude
  Code **memory** (durable context); touch the README only when the decision
  changes how someone *uses* stevedore.

## Comments: let the code speak first

Comments are used **only when strictly necessary**, and then they are succinct.
The **code, tests, and documentation** carry the meaning; a comment earns its
place only when it adds something they can't, in as few words as possible.

- **Cut narration.** Don't restate what the next line plainly does. Reach first
  for a clearer name, a smaller function, or a test — not a comment.
- **Keep the "why," but tight.** A non-obvious constraint or footgun (the
  `SecretValue` redaction contract; discarding serde's message because it quotes
  a secret) is worth a comment — one or two lines, not a paragraph. If the why is
  design rationale or project sequencing rather than a footgun at that line, it
  belongs in this file or a memory, not the source.
- **Doc comments (`///`, `//!`) describe the API and data, tightly.** They're the
  docs.rs / `--help` surface, so they stay — but they say what a type or field
  *is*, not why the design went the way it did. No rationale essays.

## Lints: the manifest is authoritative

The lint config follows the cross-project defaults in `~/.claude/rust.md`
(`[workspace.lints.*]`, `clippy.toml`, `rustfmt.toml`, `deny.toml`).

- **Levels live in `Cargo.toml`, never in CI.** CI runs plain `cargo clippy
  --all-targets`, so the local verdict is the CI verdict and `warn` still means
  "advisory". Don't reintroduce `-D warnings`.
- **`cargo doc --no-deps` is a gate.** It is the only command that enforces
  `[workspace.lints.rustdoc]`; without it every entry there is inert. It has
  already caught broken links that clippy and `cargo build` both passed.
- **Suppress with `#[expect(…, reason = "…")]`, never `#[allow]`** — `expect`
  fails once it stops applying, so it deletes itself. Both are machine-checked.
- **`map_err_ignore` is denied but deliberately suppressed twice**, in
  `dashlane/note.rs` and `proton/client.rs`. Discarding the source error there
  *is* the security rule: serde quotes the offending value, which can be a
  secret. These are permanent, not a backlog — if a third site appears, it needs
  the same reason or it is a bug.
- **`rustfmt.toml` is stable-only.** The nightly options (`group_imports`,
  `imports_granularity`, `wrap_comments`) are silently ignored on stable, so
  they are absent rather than aspirational. Adopting them means a pinned nightly
  rustfmt in the flake.
- **`cargo_common_metadata` is inert only because `publish = false`.** It fires
  the moment a release is cut, and the CLI and MCP crates have no `keywords` or
  `categories`.
- **Edition 2024 sets rustfmt's `style_edition` too**, which orders imports
  differently from 2021. An edition bump is therefore also a reformat.

## Errors and rendered output

- **`Error` and `CliError` are `#[non_exhaustive]`.** Both grow with each store,
  and neither is inert once the crates publish. Within this workspace the
  attribute does nothing, so it cannot be tested from here.
- **An error message never restates its source.** Each layer contributes only
  what it uniquely knows, because a reporter joins the chain with `: `. A
  pass-through variant that adds nothing gets `#[error(transparent)]` rather
  than a prefix — `Error::Io` is the example.
- **Build rendered output with a `Display` impl, not a `String`.** A function
  that assembles a `String` with `format!` and returns it forces an allocation
  on the caller and composes with nothing; `write!` into the formatter
  propagates instead. The CLI's plan summary is the worked example, and
  `format_push_string` is denied to keep it that way.

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
- **Crate names are chosen; binary names differ.** The bare `stevedore` name is
  taken on crates.io, so crates publish as `stevedore-secrets` (lib, imported as
  `stevedore_secrets`), `stevedore-secrets-cli`, and `stevedore-secrets-mcp`.
  The **binaries stay `stevedore` and `stevedore-mcp`** via `[[bin]] name` — the
  command a user types is independent of the crate name. Keep them that way.
- **The MCP surface is deferred.** When it lands, release MCP-surface changes
  promptly and on their own (the `ferric-fred` discipline), because
  listing/scoring builds from `main`.

## Deletion & creation (what's sensitive here)

**Ask before deleting or substantively rewriting:**

- **The decision record in `CLAUDE.md` and project memories.** These are the
  repo's decision history now that there are no ADRs — revise a decision
  deliberately (and record what changed), don't quietly drop it.
- **Lockfiles (`flake.lock`, `Cargo.lock`).** Regenerate through tooling
  (`nix flake update`, `cargo update`) — never hand-delete.
- **Tracked env config (`.envrc.shared`, `.envrc.example`)** and
  **`scripts/settings.sh`** — changing how the dev env or repo settings load
  affects everyone; confirm first.
- **`flake.nix`, `README.md`, `.github/rulesets/main.json`.**

**When creating:**

- **New design decision:** record it in `CLAUDE.md` and/or a Claude Code memory
  (no separate ADR file, and not the user-facing README).
- **New crate:** consumers depend on the library by workspace path; keep that
  compile-time coupling intact.
- **New store (source or sink):** it's a concrete module for now — do **not**
  introduce a `Store` trait to add store #2. The abstraction waits for #3/#4.
