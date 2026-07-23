# Contributing to stevedore

Thanks for your interest. stevedore is early — an honest scaffold with one route
(Dashlane → Proton Pass) under construction — so the most useful contributions
right now are around that first route and the core library shape.

## Ground rules

- **Never commit a secret.** This tool handles secret values; a stray export or a
  pasted credential must never land in git. The `.githooks/pre-commit` guard
  scans staged changes with `gitleaks` and blocks secret files — arm it with
  `git config core.hooksPath .githooks`. Don't work around it.
- **Read the README's [Design & scope](README.md#design--scope) first.** It
  records why things are the way they are — mover-not-resolver; no `Store` trait
  until store #3/#4; store access via each vendor's CLI; MCP deferred.

## Development

The dev environment is a Nix flake, loaded by direnv (or `nix develop`):

```console
cp .envrc.example .envrc && direnv allow
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
cargo deny check
```

CI runs exactly these through the same flake, so green locally means green in CI.

## Landing a change

- Work on a branch and open a PR — `main` requires a PR and the `ci` check.
- Keep the change and its docs together: if you change behavior, the public API,
  or a design decision, update the affected README / rustdoc / `--help` in the
  **same** PR.
- Squash-merge; delete the branch after.

## License

By contributing you agree that your contributions are licensed under the same
terms as the project: Apache-2.0 or MIT, at the user's option.
