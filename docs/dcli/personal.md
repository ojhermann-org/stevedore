# Personal

## Authentication

Follow Dashlane's
[authentication guide](https://cli.dashlane.com/personal/authentication) to
register your machine.

## How stevedore reads it

stevedore reads the vault by running `dcli` and keeping the results in memory for
the run. It never asks Dashlane to export the vault: an export would be a
plaintext copy of every secret on disk.

## What stevedore can read

🟢 available · 🔴 not available

| Dashlane UI   | Available |
| ------------- | :-------: |
| Logins        |    🟢     |
| Secure notes  |    🟢     |
| Passkeys      |    🔴     |
| Payments      |    🔴     |
| Personal info |    🔴     |
| IDs           |    🔴     |

### Logins

**Read:** title, website address, username, password, an attached note, and a
2FA (TOTP) token where one is set.

**Treated as secret:** the password, the 2FA (TOTP) token, and the attached note.

**Not read:** the collection a login belongs to.

### Secure notes

**Read:** title and content, including notes marked *protected*.

**Treated as secret:** the note's content. If a note has an attachment, stevedore
reads a reference to it (not the file), and that reference's access keys are
treated as secret too.

**Not read:** the collection a note belongs to, and the attached file itself — it
stays in Dashlane.
