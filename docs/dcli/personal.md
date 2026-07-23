# Personal

## Authentication

Follow Dashlane's
[authentication guide](https://cli.dashlane.com/personal/authentication) to
register your machine.

## What `dcli` can reach

🟢 available · 🔴 not available

| Dashlane UI   | Via `dcli` |
| ------------- | :--------: |
| Logins        |     🟢     |
| Secure notes  |     🟢     |
| Passkeys      |     🔴     |
| Payments      |     🔴     |
| Personal info |     🔴     |
| IDs           |     🔴     |

`dcli` ships listers for logins (`dcli password`) and secure notes
(`dcli note`) only.

### Logins

**Extracted:** title, website address, username, password, an attached note,
and a 2FA (TOTP) token where one is set.

**Not extracted:** the collection a login belongs to.

### Secure notes

**Extracted:** title and content. Notes marked *protected* are included —
that setting does not restrict what `dcli` returns.

**Not extracted:** the collection a note belongs to, and attachments — an
attached file stays in Dashlane, and only a reference to it is visible.
