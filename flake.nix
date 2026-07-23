{
  description = "stevedore — move secrets between password managers and vaults (library + CLI + MCP)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Track the recent stable toolchain (no pinned MSRV yet). Components
        # cover editor + lint/format tooling out of the box.
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
            "rustfmt"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.cargo-deny # dependency license / advisory / ban policy (deny.toml)
            pkgs.bacon # background `cargo check`/clippy/test runner
            pkgs.gitleaks # secret scanner for the pre-commit guard (.githooks/pre-commit)
            pkgs.actionlint # GitHub Actions workflow linter (.github/workflows)
            pkgs.yamllint
          ];

          env.RUST_BACKTRACE = "1";

          shellHook = ''
            echo "stevedore dev shell — $(rustc --version)"
          '';
        };

        # `nix fmt` formats the Nix files in this repo.
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
