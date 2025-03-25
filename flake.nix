{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = inputs: let
    system = "x86_64-linux";
    overlays = [(import inputs.rust-overlay)];
    pkgs = import inputs.nixpkgs {inherit system overlays;};

    rust = pkgs.rust-bin.stable.latest.default.override {
      targets = ["thumbv7em-none-eabihf"];
      extensions = ["rust-src" "rust-analyzer" "llvm-tools-preview"];
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        cargo-binutils
        pkg-config
        dfu-util
        rust
        (import ./flash.nix pkgs)
      ];
    };
  };
}
