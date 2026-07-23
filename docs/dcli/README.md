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

## Scope

- [Personal](personal.md)
