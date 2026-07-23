//! `stevedore-mcp` — MCP server exposing stevedore's secret-migration tools.
//!
//! Scaffold only. The tool surface is deferred until the migration core (source
//! read → [`stevedore::Plan`] → sink write) stabilizes, at which point this
//! crate adopts an `rmcp` stdio server like `ferric-fred-mcp`. Shipping an
//! empty-but-compiling crate now fixes the workspace shape (ADR-0002) without
//! publishing tool definitions against an API that doesn't exist yet (ADR-0003).

fn main() {
    eprintln!(
        "stevedore-mcp: no tools yet — the MCP surface is deferred until the \
         migration core lands (ADR-0003)."
    );
}
