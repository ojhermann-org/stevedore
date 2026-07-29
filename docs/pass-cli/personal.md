# Personal

## How stevedore writes to it

stevedore creates items in a vault you name, by running `pass-cli` and passing
each item as JSON on the tool's standard input. Nothing is written to disk, and
no secret ever appears in a command line.

## What stevedore can write

🟢 available · 🔴 not available

| Proton Pass UI | Available |
| -------------- | :-------: |
| Logins         |    🟢     |
| Secure notes   |    🟢     |
| Credit cards   |    🔴     |
| Identities     |    🔴     |
| SSH keys       |    🔴     |
| WiFi           |    🔴     |
| Custom items   |    🔴     |
| Aliases        |    🔴     |

### Logins

**Written:** title, username, email, password, a 2FA (TOTP) token, and website
addresses.

**Treated as secret:** the password and the 2FA (TOTP) token.

**Not written:** a note attached to the login — Proton's item template has no
field for one.

### Secure notes

**Written:** title and content.

**Treated as secret:** the note's content.

**Not written:** attachments.

## Items are created, never changed

stevedore only creates items. It does not update or delete an existing one, and
it does not check whether an item of the same title is already in the vault —
creating one twice leaves two items.
