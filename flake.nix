{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    parts.url = "github:hercules-ci/flake-parts";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      parts,
      treefmt-nix,
      ...
    }:
    parts.lib.mkFlake { inherit inputs; } {
      imports = [
        treefmt-nix.flakeModule
      ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      perSystem =
        {
          pkgs,
          system,
          ...
        }:
        let
          treefmtEval = treefmt-nix.lib.evalModule pkgs {
            projectRootFile = "Cargo.toml";
            programs = {
              nixfmt.enable = true; # nix
              rustfmt.enable = true; # rust
              yamlfmt.enable = true; # yaml
            };
          };
        in
        {
          packages = {
            git-msg = pkgs.callPackage ./nix/pkgs/git-msg.nix { };
            default = self.packages.${system}.git-msg;
          };
          devShells.default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.git-msg ];
            buildInputs = with pkgs; [
              rust-analyzer
              rustfmt
            ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
          formatter = treefmtEval.config.build.wrapper;
          checks = {
            formatting = treefmtEval.config.build.check self;
          };
        };
    };
}
