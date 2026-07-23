# ADR-0002: Workspace layout & crate boundaries

- **Status:** Accepted
- **Date:** 2026-07-23
- **Deciders:** Project owner

## Context

`stevedore` comprises three deliverables — a library that moves secrets between
stores, a CLI, and an MCP server — that share types. We want the consumers to
stay in lockstep with the library at the type level, and a release process that
can version them coherently later.

## Decision

We will use a **single Cargo workspace** with three member crates:

| Crate | Kind | Depends on |
|-------|------|-----------|
| `stevedore` | library (`lib.rs`) | — (the foundation) |
| `stevedore-cli` | binary (`stevedore`) | `stevedore` (by path) |
| `stevedore-mcp` | binary (`stevedore-mcp`) | `stevedore` (by path, when the tool surface lands) |

- The library depends on **neither** binary crate. Dependencies point one way,
  toward the library.
- Consumers depend on the library **by workspace path**, so a breaking library
  change cannot compile-pass its consumers without updating them. This
  compile-time coupling is the primary "stay in sync" guarantee.
- Shared dependency versions and lints are declared once via
  `[workspace.dependencies]` / `[workspace.lints]` and inherited by members.
- Every crate is `publish = false` for now — nothing ships to crates.io until the
  owner cuts a release.

## Consequences

- One `Cargo.lock`, one `cargo test` / `clippy` invocation covers everything.
- Refactoring library types surfaces breakage in consumers immediately.
- Slightly more ceremony than a single crate, justified by the three distinct
  deliverables.

## Alternatives considered

- **Separate repositories per crate** — maximal independence, but loses the
  compile-time sync guarantee and multiplies CI wiring. Rejected.
- **One crate with feature flags** (`cli`, `mcp`) — fewer moving parts, but
  muddies the dependency graph and complicates per-artifact versioning.
