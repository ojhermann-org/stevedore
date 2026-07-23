# Personal

What you do once, before stevedore can read a **personal** Dashlane vault.
stevedore never handles your Master Password or 2FA — it assumes an
authenticated, unlocked `dcli` and only checks that it is.

Follow Dashlane's
[authentication guide](https://cli.dashlane.com/personal/authentication) to
register this machine and its unlock and lock options. In short: run `dcli sync`
and complete your **Master Password + 2FA** (email code, TOTP, or Duo); after
that `dcli` stays unlocked between commands until you lock it. Self-hosted and
Confidential SSO are out of scope (see the [overview](README.md#scope)).

`dcli`'s vault access is **read-only**, which is all stevedore needs: it reads
records, wraps each value in a redacting `SecretValue` on the way in, never writes
back to Dashlane, and never prints what it read.
