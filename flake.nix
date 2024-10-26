{
  description = "A Nix flake for building and running a Rust binary";

  inputs = {
    # Nixpkgs provides the Rust toolchain and other dependencies
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        # Package to build the Rust project
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "tp_per_fast";
          version = "0.1.0";
          src = ./.;

          # Add protobuf as a build dependency
          nativeBuildInputs = [ pkgs.protobuf ];

          # Optional: extra arguments for cargo
          cargoBuildOptions = ["--release"];
          cargoLock = {lockFile = ./Cargo.lock; };
        };

        # Development shell with Rust toolchain
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.pkg-config
            pkgs.protobuf
          ];
        };

        # Run the binary
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/tp_per_fast";
        };
      });
}
