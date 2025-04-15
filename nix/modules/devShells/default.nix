{ pkgs, config, ... }:
{
  devShells.default = pkgs.mkShell {
    inputsFrom = [ config.git-msg-bin ];
    buildInputs = with pkgs; [
      rust-analyzer
      rustfmt
    ];
    RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
}
