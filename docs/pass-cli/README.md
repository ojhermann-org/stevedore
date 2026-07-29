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

## Scope

- [Personal](personal.md)
