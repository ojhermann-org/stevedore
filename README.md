# stevedore

A stevedore moves cargo between vessels; `stevedore` moves **secrets** between
stores — reading them out of one and writing them into another. It's a Rust
workspace with three deliverables: a **library**, a **CLI**, and an **MCP
server**.

## Stores

🟢 working · 🟡 in progress · 🔵 planned · 🔴 not planned

| Store                  | Read | Write |
| ---------------------- | :--: | :---: |
| [Dashlane](docs/dcli/) |  🟢  |  🔴   |

## What it is (and isn't)

- **A mover.** stevedore _migrates_ secret values from a source store to a sink
  store.
- **Not a resolver.** Runtime secret _resolution_ — an app fetching its own key
  at start-up — is what [secretspec](https://github.com/cachix/secretspec) does.
- **Safe by default.** Secret values redact themselves in logs by construction —
  passwords, note contents, 2FA seeds and attachment keys alike — and nothing is
  ever exported to disk.

## Layout

| Crate                                          | What it is               |
| ---------------------------------------------- | ------------------------ |
| [`crates/stevedore`](crates/stevedore)         | The core library.        |
| [`crates/stevedore-cli`](crates/stevedore-cli) | The command-line binary. |
| [`crates/stevedore-mcp`](crates/stevedore-mcp) | The MCP server.          |

## Develop

The dev environment is a Nix flake; [direnv](https://direnv.net/) loads it on
entry.

```console
# one-time, per clone:
cp .envrc.example .envrc && direnv allow   # loads the flake dev shell
git config core.hooksPath .githooks        # arm the git hooks (pre-commit secret scan + pre-push fmt/clippy)

# or without direnv:
nix develop

# then, the usual loop:
cargo test
cargo clippy --all-targets -- -D warnings
cargo run -p stevedore-cli -- stores
```

CI runs `fmt`, `clippy`, `test`, and `cargo deny check` through the same flake.

### A note on secrets

stevedore's inputs are _your other stores' credentials_. They are supplied
deliberately for a single run — never committed, never loaded ambiently into
every shell. The `.gitignore` and the `.githooks/pre-commit` guard exist to keep
it that way.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
