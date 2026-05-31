{
  pkgs,
  rustToolchain ? pkgs.rust-bin.stable.latest.default,
}:

pkgs.rustPlatform.buildRustPackage {
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
}
