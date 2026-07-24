# stevedore-secrets

The core library behind [`stevedore`](https://github.com/ojhermann-org/stevedore).

## What it offers

- `SecretValue` — a secret that redacts itself in `Debug` and `Display`. Read
  it deliberately with `.expose()`.
- `dashlane` — reads a Dashlane vault through Dashlane's own `dcli`, keeping
  values in-process and access read-only.

Licensed under either of Apache-2.0 or MIT at your option.
