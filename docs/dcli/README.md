# Dashlane CLI (`dcli`)

stevedore's Dashlane **source** reads a vault through Dashlane's official,
open-source CLI, [`dcli`](https://github.com/Dashlane/dashlane-cli). Reading
through `dcli` keeps secret values **in-process** — structured JSON on stdout, no
plaintext export file on disk — and vault access is **read-only**, which is
exactly what a migration source needs.

**stevedore never authenticates.** Registering a device, entering the Master
Password, and passing 2FA are a **one-time setup the user performs with `dcli`
directly** — see [Personal](personal.md). stevedore assumes an already
authenticated, unlocked `dcli` and only preflights it.

## Scope

This directory maps only the slices of `dcli` that stevedore depends on. Today
that's one:

| Slice | Doc | Covers |
|-------|-----|--------|
| Personal | [personal.md](personal.md) | The one-time auth setup and read-only vault, for a personal Dashlane account. |

Personal is **Master Password + 2FA** (email code, TOTP, or Duo) only.
Self-hosted SSO and Confidential SSO (Nitro Enclaves) are **out of scope** — this
tool doesn't target them. Other slices (e.g. Business) can be added as sibling
docs later without reshuffling.

## Install

Install `dcli` by following Dashlane's own
[installation guide](https://cli.dashlane.com/install), then see
[Personal](personal.md) for the one-time setup.
