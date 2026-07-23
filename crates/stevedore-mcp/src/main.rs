//! `stevedore-mcp` — MCP server exposing stevedore's secret-migration tools.
//!
//! **Scaffold only.** The tool surface is deliberately deferred until the core
//! migration API (source read → [`stevedore::Plan`] → sink write) stabilizes, at
//! which point this crate adopts an `rmcp` stdio server the same way
//! `ferric-fred-mcp` does. Shipping an empty-but-compiling crate now fixes the
//! workspace shape (library + CLI + MCP, ADR-0002) without publishing unverified
//! tool definitions against an API that doesn't exist yet. See ADR-0003.

fn main() {
    eprintln!(
        "stevedore-mcp: no tools yet — the MCP surface is deferred until the \
         migration core lands (ADR-0003)."
    );
}
