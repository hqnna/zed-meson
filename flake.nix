{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    utils.url = "github:numtide/flake-utils";
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
