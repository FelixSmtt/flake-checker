{
  self,
  inputs,
  ...
}:

{
  perSystem =
    {
      system,
      ...
    }:
    let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ (import inputs.rust-overlay) ];
      };
      rustToolchain = pkgs.rust-bin.stable.latest.default;
    in
    {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "flake-checker";
        version = "0.1.0";
        src = ../.;
        cargoLock = {
          lockFile = ../Cargo.lock;
        };
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [
          pkgs.openssl
          pkgs.dbus
          pkgs.libnotify
        ];
        RUSTC = "${rustToolchain}/bin/rustc";
        CARGO = "${rustToolchain}/bin/cargo";
      };
    };
}
