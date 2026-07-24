# stevedore

The core library behind [`stevedore`](https://github.com/ojhermann-org/stevedore).

## What it offers

- [`SecretValue`] — a secret that redacts itself in `Debug` and `Display`. Read
  it deliberately with `.expose()`.
- [`dashlane`] — reads a Dashlane vault through Dashlane's own `dcli`, keeping
  values in-process and access read-only. Logins and secure notes are separate
  types, each carrying every field Dashlane reports.

Licensed under either of Apache-2.0 or MIT at your option.

[`SecretValue`]: https://docs.rs/stevedore
[`dashlane`]: https://docs.rs/stevedore
