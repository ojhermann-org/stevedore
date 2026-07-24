# Keeping secrets safe

stevedore exists to move secret *values* from one source to another. **A secret value must never leak** — never
printed, logged, serialized, or written to disk. This document explains what that means and how the code enforces it.

## What counts as a secret

stevedore models each source explicitly and marks certain fields of each record
as secret **values** — the material this tool exists to protect. Everything a
source does not mark as secret is treated as ordinary metadata.

Which fields are secret is a property of each source and is listed in that
source's own documentation (for Dashlane, see
[the Personal notes](dcli/personal.md)). The guarantees below apply to every
field marked secret, whatever the source.

## How a leak is prevented

### Secret values redact themselves

Every secret is held in a `SecretValue`, a type that **cannot print itself**.
Formatting one — in a log line, an error, a debug dump — yields `<redacted>`,
never the value. Reading the actual bytes requires calling `.expose()`
explicitly, which is deliberately easy to search the code for. `SecretValue`
also has **no way to be serialized**, so a secret can never be accidentally
written back out as part of a data structure.

### Values never touch the disk

stevedore holds secret values **in memory** for the life of a run and never
writes them to disk — no cache, no temporary file, no export. Where a source
offers a bulk "export the vault" feature, stevedore does not use it: an export is
a plaintext copy of every secret in a file, exactly the artifact this tool
refuses to create. How each source is read without one is covered in that
source's documentation.

### Parser errors can't echo the input

When stevedore parses the data a source returns, a parsing
library will, on a type mismatch, put **the offending value into its error
message** (`invalid type: string "hunter2", …`). If that error were then logged,
the secret would leak — and the redaction above would not help, because the leak
happens while reading the raw text, before any `SecretValue` exists.

Three things close this:

1. **One gateway.** All parsing of source output goes through a single function
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

- **Metadata is not redacted.** Unless identified and modeled as a secret value, fields will be treated as metadata.
- **The source's own tool has its own behavior.** stevedore never authenticates
  and never unlocks a vault; that is set up separately with each source. How that tool
  stores credentials, and what it does with the system clipboard, is outside
  stevedore's control.
- **Diagnostics from a source's tool are surfaced.** When a source's tool fails,
  its own error text (for example "not logged in") is shown so you can act on it.
  This is that tool's diagnostic channel, not the vault contents.

## Verifying it yourself

The mechanisms above live in a few small places in the `stevedore-secrets`
library: the redacting type in `secret.rs`, the parsing gateway and error type in
`error.rs`, and the leak-regression tests alongside each. They are deliberately
compact so the guarantee can be read end to end.
