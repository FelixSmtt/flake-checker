{
  description = "Nix flake for flake-checker with flake-parts";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{
      nixpkgs,
      rust-overlay,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      imports = [
        ./nix/nixos-module.nix
        ./nix/hm-module.nix
      ];

      perSystem =
        {
          system,
          ...
        }:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system;
            overlays = overlays;
          };
          rustToolchain = pkgs.rust-bin.stable.latest.default;
          flake-checker-pkg = pkgs.callPackage ./nix/package.nix { inherit rustToolchain; };
        in
        {
          packages.default = flake-checker-pkg;
        };
    };
}
