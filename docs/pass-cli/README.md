# Proton Pass CLI (`pass-cli`)

stevedore's Proton Pass **sink** writes into a vault through Proton's
`pass-cli`, handing secret values to it **on stdin** — never as command-line
arguments, which other processes on the machine can read.

**stevedore never authenticates.** Logging in is a **one-time setup the user
performs with `pass-cli` directly**. stevedore assumes a working session.

## Install

Install `pass-cli` by following Proton's own
[installation guide](https://proton.me/support/pass-cli).

## Log in

```console
pass-cli login          # opens a browser
pass-cli test           # "Connection successful"
```

## How stevedore writes to a vault

stevedore creates items in a vault you name, by running `pass-cli` and passing
each item as JSON on the tool's standard input. Nothing is written to disk, and
no secret ever appears in a command line.

## What can be written

🟢 supported · 🔵 planned · 🔴 not possible

| Proton Pass UI | `pass-cli` | stevedore |
| -------------- | :--------: | :-------: |
| Logins         |     🟢     |    🟢     |
| Secure notes   |     🟢     |    🟢     |
| Credit cards   |     🟢     |    🔵     |
| Identities     |     🟢     |    🔵     |
| SSH keys       |     🟢     |    🔵     |
| WiFi           |     🟢     |    🔵     |
| Custom items   |     🟢     |    🔵     |
| Aliases        |     🟢     |    🔵     |
| Attachments    |     🔴     |    🔴     |

`pass-cli` can download an attachment but not upload one.

### Logins

**Written:** title, username, email, password, a 2FA (TOTP) token, and website
addresses.

**Treated as secret:** the password and the 2FA (TOTP) token.

**Not written:** a note attached to the login. A Proton login carries one, but
`pass-cli`'s item template has no field for it — supplying one anyway is accepted
and silently discarded — and the only command that would set it takes the value
as a command-line argument.

### Secure notes

**Written:** title and content.

**Treated as secret:** the note's content.

## Items are created, never changed

stevedore only creates items. It does not update or delete an existing one, and
it does not check whether an item of the same title is already in the vault —
creating one twice leaves two items.

## Unknown fields are dropped in silence

`pass-cli` creates an item from a template carrying a field it does not know,
without the field and without a word. A name that does not match the template
loses data quietly, so the names stevedore sends are pinned by test to
`pass-cli item create <kind> --get-template`.
