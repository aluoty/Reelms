{
  description = "Reelms — low-poly stylized fishing RPG (Rust + Bevy 2D)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        bevyRuntimeLibs = with pkgs; [
          udev
          alsa-lib-with-plugins
          vulkan-loader
          libx11
          libxcursor
          libxi
          libxrandr
          libxkbcommon
          wayland
        ];

        bevyNativeInputs = with pkgs; [
          pkg-config
          rustToolchain
          clippy
          rustfmt
        ];

        shellEnv = {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath bevyRuntimeLibs;
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";
        };

      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = bevyNativeInputs;
          buildInputs = bevyRuntimeLibs;
          inherit (shellEnv) LD_LIBRARY_PATH RUST_SRC_PATH RUST_BACKTRACE;

          shellHook = ''
            echo "Reelms dev shell — Bevy 2D + Rust $(rustc --version | cut -d' ' -f2)"
            echo "  cargo run          — launch game"
            echo "  cargo run --release  — optimized build"
            echo "  cargo clippy         — lint"
          '';
        };

        apps.default = flake-utils.lib.mkApp {
          drv = pkgs.writeShellScriptBin "reelms" ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath bevyRuntimeLibs}"
            exec ${rustToolchain}/bin/cargo run --manifest-path "${
              self + "/Cargo.toml"
            }" --release "$@"
          '';
        };

        formatter = pkgs.nixpkgs-fmt;
      });
}
