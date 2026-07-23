# ADR-0003: First target, and how stores are modelled

- **Status:** Accepted
- **Date:** 2026-07-23
- **Deciders:** Project owner

## Context

`stevedore` is a tool for moving secrets between password managers and vaults.
"Between arbitrary stores" is the eventual shape, but a v0 has to start
somewhere, and starting with the wrong abstraction is the classic way to make a
tool that does everything badly. Two questions need answering before code: which
route do we build first, and how much store-abstraction do we commit to now.

There is also prior art to be clear about. The owner contributes to
[secretspec](https://github.com/cachix/secretspec), which is a secret
**resolver**: it already has a Proton Pass provider (driving `pass-cli`), write
support, and FFI SDKs for several languages. stevedore is not that and must not
drift into rebuilding it.

## Decision

**First route: Dashlane → Proton Pass.** It's a concrete migration the owner
actually needs (consolidating a Dashlane vault into Proton), which keeps the v0
honest — real formats, real edge cases — instead of an abstract framework with
no user.

**stevedore is a mover, not a resolver.** Its job is a one-shot, verifiable
*migration* of secret values from a source store to a sink store. Runtime
secret *resolution* (an app fetching its own key at start-up) is secretspec's
job; stevedore composes with that world rather than cloning it. We will not
reopen "just fork/rebuild secretspec" without new evidence.

**No `Store` trait yet.** The source and sink are **concrete modules**
(`dashlane`, `proton`). With exactly two stores, a `Store`/`Source`/`Sink` trait
would be a guess fitted to a sample size of one route. We defer the abstraction
until a third or fourth store shows where the real seams are.

**Dry-run is the default.** Because the payload is secret material and the write
is hard to undo, `migrate` plans by default (reads the source, reports what
would move) and requires an explicit `--apply` to write to a sink.

**The MCP surface is deferred.** The `stevedore-mcp` crate exists (workspace
shape, ADR-0002) but ships as a compiling placeholder. Defining tools over a
migration API that doesn't exist yet would be inventing a surface we'd only
rework. When the core (source read → plan → sink write) stabilizes, the crate
adopts an `rmcp` stdio server mirroring `ferric-fred-mcp`, and MCP-surface
changes are then released promptly and on their own.

## Consequences

- The v0 is small and real: one route, concrete code, nothing speculative.
- Adding a second sink or source before the trait exists means a little
  duplication — accepted deliberately as the price of not guessing the
  abstraction early.
- The reverse direction (Proton → Dashlane, or exporting back out) is out of
  scope for now; directions are added deliberately, not assumed symmetric.
- Anyone reading the code sees honest "unimplemented" stubs rather than a
  framework pretending to be finished.

## Alternatives considered

- **Build a generic `Store` trait first, then implement Dashlane/Proton against
  it.** Rejected: premature abstraction from one data point; the trait would
  encode accidental properties of the first route.
- **Start by extending secretspec** (add Infisical / cross-store transfer
  upstream). Considered and set aside: secretspec is the resolver and moves fast
  under a single maintainer; the differentiated piece — verifiable cross-store
  *migration* with a CLI and MCP — is better owned here and composed with it.
- **Make the MCP server first**, to get an agent surface early. Rejected: the
  tools would be defined against a non-existent core and reworked wholesale.
