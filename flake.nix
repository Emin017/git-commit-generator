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
          inputs',
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
          imports = [
            ./nix
          ];
          formatter = treefmtEval.config.build.wrapper;
          checks = {
            formatting = treefmtEval.config.build.check self;
          };
        };
    };
}
