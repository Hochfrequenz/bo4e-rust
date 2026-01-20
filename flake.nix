{
  description = "PM Tool - Project Management Application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    disko = {
      url = "github:nix-community/disko";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    sops-nix = {
      url = "github:Mic92/sops-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils, disko, sops-nix, ... }:
    let
      # NixOS configurations (system-independent)
      nixosConfigurations = {
        pm-tool-server = nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            disko.nixosModules.disko
            sops-nix.nixosModules.sops
            ./nix/hosts/pm-tool-server.nix
            ({ pkgs, ... }: {
              nixpkgs.overlays = [
                (final: prev: {
                  pm-api = self.packages.x86_64-linux.pm-api;
                })
              ];
            })
          ];
        };
      };
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common build inputs
        buildInputs = with pkgs; [
          openssl
          sqlite
          webkitgtk_4_1
          gtk3
          libsoup_3
          glib
          cairo
          pango
          gdk-pixbuf
          atk
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.CoreServices
          darwin.apple_sdk.frameworks.CoreFoundation
          darwin.apple_sdk.frameworks.WebKit
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
          nodejs_22
          pnpm
        ];

      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "PM Tool development environment"
            echo "Rust: $(rustc --version)"
            echo "Node: $(node --version)"
          '';
        };

        packages = {
          pm-api = craneLib.buildPackage {
            src = ./.;
            pname = "pm-api";
            cargoExtraArgs = "-p pm-api";
            inherit buildInputs nativeBuildInputs;
          };

          pm-tauri = craneLib.buildPackage {
            src = ./.;
            pname = "pm-tauri";
            cargoExtraArgs = "-p pm-tauri";
            inherit buildInputs nativeBuildInputs;

            # Build frontend first
            preBuild = ''
              cd frontend
              pnpm install --frozen-lockfile
              pnpm build
              cd ..
            '';
          };

          default = self.packages.${system}.pm-api;
        };
      }
    ) // { inherit nixosConfigurations; };
}
