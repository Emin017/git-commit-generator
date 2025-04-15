{
  pkgs,
  pkg-config,
  stdenv,
  lib,
}:

let
  libiconvLib = pkgs.pkgsStatic.libiconv.dev;
in
pkgs.pkgsStatic.rustPlatform.buildRustPackage {
  pname = "git-msg";
  version = "0.2.0";

  cargoLock = {
    lockFile = ./../../../Cargo.lock;
  };
  src =
    with lib.fileset;
    toSource {
      root = ./../../..;
      fileset = unions [
        ./../../../src
        ./../../../Cargo.lock
        ./../../../Cargo.toml
      ];
    };

  PKG_CONFIG_PATH = "${pkgs.pkgsStatic.openssl.dev}/lib/pkgconfig";

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    pkgs.pkgsStatic.openssl.dev
  ];

  RUSTFLAGS = lib.optionalString stdenv.hostPlatform.isDarwin ''
    -C link-args=-L${libiconvLib}/lib
  '';

  NIX_LDFLAGS = lib.optionalString stdenv.hostPlatform.isDarwin ''
    -L${libiconvLib}/lib -liconv -lcharset
  '';
}
