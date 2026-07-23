# stevedore

> Move secrets between password managers and vaults.

A stevedore moves cargo between vessels. This one moves **secrets** between
stores — reading them out of one password manager or vault and writing them into
another, as a one-shot, verifiable migration. It's a Rust workspace with three
deliverables: a **library**, a **CLI**, and an **MCP server** for agents.

The first route under construction is **Dashlane → Proton Pass**.

## Stores

The stores stevedore moves secrets between, each driven through its own
command-line tool and documented under [`docs/`](docs/):

- [Dashlane](docs/dcli/) — source, via the Dashlane CLI (`dcli`)

## What it is (and isn't)

- **A mover, not a resolver.** stevedore *migrates* secret values from a source
  store to a sink store. Runtime secret *resolution* — an app fetching its own
  key at start-up — is [secretspec](https://github.com/cachix/secretspec)'s job;
  stevedore composes with that world.
- **Safe by default.** Secret values redact themselves in logs by construction,
  and `migrate` is a dry-run unless you pass `--apply`.

## Layout

| Crate | What it is |
|-------|-----------|
| [`crates/stevedore`](crates/stevedore) | The core library: `SecretRecord`, redacting `SecretValue`, `Plan`, and the concrete modules. |
| [`crates/stevedore-cli`](crates/stevedore-cli) | The command-line binary. |
| [`crates/stevedore-mcp`](crates/stevedore-mcp) | The MCP server. |

## Design & scope

The decisions that fix stevedore's shape. They live here (and in `CLAUDE.md`).

**First route: Dashlane → Proton Pass.** A concrete migration the owner actually
needs, which keeps v0 honest — real formats and edge cases instead of an abstract
framework with no user. The reverse direction (Proton → Dashlane) is out of scope
for now; directions are added deliberately, not assumed symmetric.

**A mover, not a resolver.** stevedore does a one-shot, verifiable *migration* of
secret values from a source to a sink.

**Dry-run is the default.** Because the payload is secret material and the write
is hard to undo, `migrate` plans by default (reads the source, reports what would
move) and requires an explicit `--apply` to write to a sink.

**Workspace layout.** A single Cargo workspace, three member crates; dependencies
point one way, toward the library. Consumers depend on `stevedore` **by workspace
path**, so a breaking library change can't compile-pass its consumers without
updating them — that compile-time coupling is the primary "stay in sync" guarantee.
Shared dependency versions and lints are declared once in `[workspace.dependencies]`
/ `[workspace.lints]`. Every crate is `publish = false` until the owner cuts a
release.

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
