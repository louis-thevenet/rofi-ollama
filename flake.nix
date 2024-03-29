{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
      ];
      perSystem =
        { config
        , self'
        , pkgs
        , lib
        , system
        , ...
        }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rust-toolchain = pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = [ pkgs.rustc pkgs.cargo pkgs.cargo-watch pkgs.rust-analyzer pkgs.rustPlatform.rustcSrc ];
          };

          buildInputs = with pkgs; [
            glib
            openssl
            cairo
            pango

          ];
          nativeBuildInputs = with pkgs; [
            pkg-config

          ];
        in
        {
          # Rust package
          packages.default = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            RUST_BACKTRACE = 1;
            RUSTFLAGS = "--cfg rofi_next";

            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
          };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.treefmt.build.devShell
            ];
            RUST_BACKTRACE = 1;
            RUSTFLAGS = "--cfg rofi_next";
            ROFI_PLUGIN_PATH = "./target/debug";
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

            nativeBuildInputs = nativeBuildInputs;
            buildInputs =  buildInputs ++ [ rust-toolchain pkgs.clippy ];
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };
        };
    };
}
