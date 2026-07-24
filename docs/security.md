# Keeping secrets safe

stevedore exists to move secret *values* from one store to another. The single
rule everything else serves is: **a secret value must never leak** — never
printed, logged, serialized, or written to disk. This document explains what
that means and how the code enforces it, so you can trust the tool and verify
the claim yourself.

## What counts as a secret

These are the values stevedore treats as secret and guards:

- **Passwords** on a login.
- **Note contents.**
- **2FA (TOTP) seeds** — carried inside a login's `otpauth://` URL. The field
  name ends in "URL", but the seed sits in it, so it is a secret.
- **Attachment keys** — the encryption key and download key attached to a note.

Everything else a store reports — titles, usernames, website addresses, file
names, timestamps — is treated as ordinary metadata.

## How a leak is prevented

### Secret values redact themselves

Every secret is held in a `SecretValue`, a type that **cannot print itself**.
Formatting one — in a log line, an error, a debug dump — yields `<redacted>`,
never the value. Reading the actual bytes requires calling `.expose()`
explicitly, which is deliberately easy to search the code for. `SecretValue`
also has **no way to be serialized**, so a secret can never be accidentally
written back out as part of a data structure.

### Values never touch the disk

stevedore reads a Dashlane vault by driving Dashlane's own `dcli` tool and
keeping the results **in memory** for the life of the run. It never asks Dashlane
for a vault export, because an export is a plaintext copy of every secret sitting
in a file — exactly the artifact this tool refuses to create.

### Parser errors can't echo the input

This is the subtle one. When stevedore parses the JSON a store returns, a parsing
library will, on a type mismatch, put **the offending value into its error
message** (`invalid type: string "hunter2", …`). If that error were then logged,
the secret would leak — and the redaction above would not help, because the leak
happens while reading the raw text, before any `SecretValue` exists.

Three things close this:

1. **One gateway.** All parsing of store output goes through a single function
   (`from_json`). It is the only place allowed to hold a parser error, and it
   throws the message away, keeping only *which* field failed and the *position*
   of the failure — neither of which contains a value.
2. **Errors that structurally can't hold a value.** The resulting "couldn't
   parse" error stores the field name as a fixed constant, not free text. It is
   impossible, at compile time, to smuggle a runtime value into it.
3. **A test that keeps it honest.** Automated tests feed the parser a marked fake
   secret in malformed input and assert the marker never appears in the error —
   in either its normal or its debug form. If a future change reintroduces a
   value-carrying error, these tests fail.

## What this does not cover

Being precise about the boundary matters as much as the guarantees:

- **Metadata is not redacted.** Titles, usernames, website addresses, file
  names, and the email address attached to a file appear in plain form. These
  are not treated as secret values; if any are sensitive in your vault, know that
  they are handled as ordinary data.
- **The store's own tool has its own behavior.** stevedore never authenticates
  and never unlocks a vault; that is set up separately with `dcli`. How that tool
  stores credentials, and what it does with the system clipboard, is outside
  stevedore's control — see [the `dcli` notes](dcli/).
- **Diagnostics from the store's tool are surfaced.** When `dcli` fails, its own
  error text (for example "not logged in") is shown so you can act on it. This is
  the tool's diagnostic channel, not the vault contents.

## Verifying it yourself

The mechanisms above live in a few small places in the `stevedore-secrets`
library: the redacting type in `secret.rs`, the parsing gateway and error type in
`error.rs`, and the leak-regression tests alongside each. They are deliberately
compact so the guarantee can be read end to end.
