# Dashlane CLI (`dcli`)

stevedore's Dashlane **source** reads a vault through Dashlane's
[`dcli`](https://github.com/Dashlane/dashlane-cli), keeping secret values
**in-process** and vault access **read-only**.

**stevedore never authenticates.** Registering a device, entering the Master
Password, and passing 2FA are a **one-time setup the user performs with `dcli`
directly**. stevedore assumes an already authenticated, unlocked `dcli`.

## Install

Install `dcli` by following Dashlane's own
[installation guide](https://cli.dashlane.com/install).

## What `dcli` keeps on disk

`dcli` holds a **full local copy of your vault**, and it does so whether or not
stevedore ever runs. On Linux that copy is
`~/.local/share/dashlane-cli/userdata.db` (`~/Library/Application Support/` on
macOS, `%APPDATA%` on Windows): an ordinary SQLite database carrying every login,
secure note and secret, resynchronised hourly. stevedore reads from it and adds
nothing to it.

Each item's contents are encrypted individually. The database around them is
not — anyone who can open the file can read its structure, which includes your
account email and the identifier, type and change time of every item.

Unless you turn it off with `dcli configure save-master-password false`, that
database also stores your Dashlane Master Password, encrypted. The key that
unwraps it is kept in your operating system's keyring, and is itself derivable
from the Master Password.

The file is created **readable by every account on the machine** (`0644`, in a
`0755` directory), so on a shared computer this is worth knowing about. Tightening
it with `chmod 600` is harmless and survives until `dcli` next recreates the file.
Observed on macOS, 2026-07-29.

## Scope

- [Personal](personal.md)
