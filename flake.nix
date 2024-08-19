{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    flake-parts,
    rust-overlay,
    ...
  } @ inputs: let
    overlays = [(import rust-overlay)];
  in
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];

      perSystem = {system, ...}: let
        pkgs = import nixpkgs {inherit system overlays;};
      in {
        devShells.default = let
          rust = pkgs.rust-bin.stable.latest.default.override {
            targets = ["thumbv7em-none-eabihf"];
            extensions = ["rust-src" "rust-analyzer" "llvm-tools-preview"];
          };
        in
          with pkgs;
            mkShell {
              nativeBuildInputs = [
                cargo-binutils
                pkg-config
                dfu-util
                cargo-make
                rust
              ];
            };
      };
    };
}
