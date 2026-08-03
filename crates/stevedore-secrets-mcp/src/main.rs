//! `stevedore-secrets-mcp` — MCP server exposing stevedore's tools. Scaffold
//! only: no tools yet.

// stdout is the MCP wire — a stray byte there corrupts the session — so every
// diagnostic this binary writes goes to stderr.
#![expect(clippy::print_stderr, reason = "stderr is this binary's only output")]

fn main() {
    eprintln!("stevedore-mcp: no tools yet — the MCP surface is not built.");
}
