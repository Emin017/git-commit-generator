{
  pkgs,
  rustPlatform,
  pkg-config,
  openssl,
  rustPackages,
  ...
}:
rustPlatform.buildRustPackage {
  pname = "git-commit-generator";
  version = "0.1.0.0";

  cargoLock = {
    lockFile = ./../../Cargo.lock;
  };
  src = ./../..;
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    rustPackages.clippy
    openssl
  ];
}
