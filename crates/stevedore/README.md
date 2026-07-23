# stevedore

The core library behind [`stevedore`](https://github.com/ojhermann-org/stevedore):
read secret records from a **source** store, plan the move, and write them to a
**sink** store.

> Early scaffold. The source readers and sink writers are honest stubs today —
> they return an "unimplemented" error rather than pretend to succeed — while the
> first route (Dashlane → Proton Pass) is built.

## Shape

- [`SecretValue`] — a secret that redacts itself in `Debug`/`Display`; read it
  deliberately with `.expose()`.
- [`SecretRecord`] — a named secret plus optional vault metadata (folder,
  username, url, note).
- [`Plan`] — what a migration would move, computed without touching the sink.
- `dashlane`, `proton` — the concrete source/sink modules. There is no `Store`
  trait yet: with two stores it would be a guess.

Licensed under either of Apache-2.0 or MIT at your option.

[`SecretValue`]: https://docs.rs/stevedore
[`SecretRecord`]: https://docs.rs/stevedore
[`Plan`]: https://docs.rs/stevedore
