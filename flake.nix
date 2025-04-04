{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/release-24.11";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, utils, fenix }:
    utils.lib.eachDefaultSystem(system:
      let
        rust = fenix.packages.${system}.stable;
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with rust; [ cargo clippy rustc rustfmt rust-src rust-analyzer ];
          RUST_SRC_PATH = "${rust.rust-src}/lib/rustlib/src/rust/library";
        };
      });
}
