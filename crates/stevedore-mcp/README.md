# stevedore-mcp

An MCP (Model Context Protocol) server exposing
[`stevedore`](https://github.com/ojhermann-org/stevedore)'s secret-migration
tools to agents. Installs the `stevedore-mcp` binary.

> **Scaffold only.** The tool surface is deliberately deferred (ADR-0003) until
> the core migration API stabilizes. Today the binary is a compiling placeholder
> that fixes the workspace shape (library + CLI + MCP); when the migration core
> lands it adopts an `rmcp` stdio server mirroring `ferric-fred-mcp`, and the
> same "release MCP-surface changes promptly" discipline applies.

Licensed under either of Apache-2.0 or MIT at your option.
