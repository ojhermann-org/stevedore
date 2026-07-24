# stevedore

The core library behind [`stevedore`](https://github.com/ojhermann-org/stevedore):
read secrets out of one store so they can be written into another.

> Under construction. Stores are modelled one at a time, precisely and on their
> own terms. Dashlane is done; moving secrets *between* stores waits until a
> second store has been modelled with the same care.

## Shape

- [`SecretValue`] — a secret that redacts itself in `Debug`/`Display`; read it
  deliberately with `.expose()`. The only type shared across stores, because it
  encodes a safety rule rather than a guess about what stores have in common.
- [`dashlane`] — reads a Dashlane vault through Dashlane's own `dcli`, keeping
  values in-process and access read-only. Logins and secure notes are separate
  types carrying every field Dashlane emits.

There is no generic record type, no `Store` trait, and no migration engine yet.
Each would be invented from a single example; the shared shape should be
discovered from a second store, not guessed ahead of it.

Licensed under either of Apache-2.0 or MIT at your option.

[`SecretValue`]: https://docs.rs/stevedore
[`dashlane`]: https://docs.rs/stevedore
