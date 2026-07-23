# stevedore

> Move secrets between password managers and vaults.

A stevedore moves cargo between vessels. This one moves **secrets** between
stores — reading them out of one password manager or vault and writing them into
another, as a one-shot, verifiable migration. It's a Rust workspace with three
deliverables: a **library**, a **CLI**, and an **MCP server** for agents.

The first route under construction is **Dashlane → Proton Pass**.

> **Status: early scaffold.** The workspace compiles and the shape is fixed, but
> the source readers and sink writers are honest stubs — they report the work as
> unimplemented rather than pretend to succeed. Follow the design in
> [`docs/adr/`](docs/adr/), starting with
> [ADR-0003](docs/adr/0003-first-target-and-store-model.md).

## What it is (and isn't)

- **A mover, not a resolver.** stevedore *migrates* secret values from a source
  store to a sink store. Runtime secret *resolution* — an app fetching its own
  key at start-up — is [secretspec](https://github.com/cachix/secretspec)'s job;
  stevedore composes with that world rather than reinventing it (ADR-0003).
- **Safe by default.** Secret values redact themselves in logs by construction,
  and `migrate` is a dry-run unless you pass `--apply`.

## Layout

| Crate | What it is |
|-------|-----------|
| [`crates/stevedore`](crates/stevedore) | The core library: `SecretRecord`, redacting `SecretValue`, `Plan`, and the concrete `dashlane` / `proton` modules. |
| [`crates/stevedore-cli`](crates/stevedore-cli) | The `stevedore` command-line binary. |
| [`crates/stevedore-mcp`](crates/stevedore-mcp) | The MCP server (scaffold — tool surface deferred, ADR-0003). |

See [ADR-0002](docs/adr/0002-workspace-layout.md) for why it's laid out this way.

## Develop

The dev environment is a Nix flake (Rust toolchain, `cargo-deny`, `bacon`,
`gitleaks`); [direnv](https://direnv.net/) loads it on entry.

```console
# one-time, per clone:
cp .envrc.example .envrc && direnv allow   # loads the flake dev shell
git config core.hooksPath .githooks        # arm the secret-scanning pre-commit guard

# or without direnv:
nix develop

# then, the usual loop:
cargo test
cargo clippy --all-targets -- -D warnings
cargo run -p stevedore-cli -- stores
```

CI runs `fmt`, `clippy`, `test`, and `cargo deny check` through the same flake.

### A note on secrets

stevedore's inputs are *your other stores' credentials*. They are supplied
deliberately for a single run — never committed, never loaded ambiently into
every shell. The `.gitignore` and the `.githooks/pre-commit` guard exist to keep
it that way; don't work around them.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
