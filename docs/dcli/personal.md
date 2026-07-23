# Personal

The one-time setup that lets stevedore read a **personal** Dashlane vault. You do
this once, directly with `dcli`; stevedore then assumes an authenticated, unlocked
CLI and never handles your Master Password or 2FA itself.

Dashlane's [authentication guide](https://cli.dashlane.com/personal/authentication)
is the source of truth for the exact flow. This page covers only what stevedore
relies on and defers the steps to Dashlane.

## Authenticate (once)

Register this device with your account and run the first sync:

```console
dcli sync
```

You'll be prompted for your email and a **second factor** — email code (the
default), a TOTP authenticator app, or a Duo push. That registers the device and
pulls the vault down locally. Scope is **Master Password + 2FA**; SSO is out of
scope (see the [overview](README.md#scope)).

## Staying unlocked

So you're not re-prompted on every command:

- By default your Master Password is saved in the **OS keychain**. Disable that
  with `dcli configure save-master-password false`.
- macOS can gate unlock behind **biometrics**:
  `dcli configure user-presence --method biometrics` (macOS only).
- Lock at any time with `dcli lock`; the next command re-prompts to unlock.

`dcli` is designed to run on your own workstation, where that keychain exists — a
headless machine has no OS keychain, so unlock is not seamless there. (Making
`dcli` usable on the NixOS devbox is tracked separately; day-to-day stevedore
development uses a macOS machine.)

## Read-only

`dcli`'s vault access is **read-only** for now, which is all a migration source
needs. stevedore reads records through `dcli`'s JSON output (e.g.
`dcli password -o json`), wraps each value in a redacting `SecretValue` on the way
in, and never writes back to Dashlane — nor prints what it read.
