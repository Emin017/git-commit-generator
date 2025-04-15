{
  lib,
  config,
  pkgs,
  system,
  ...
}:
let
  self = {
    packages = (lib.mapAttrs (name: pkg: config.mkGitMsgWrapper name pkg) config.packagesSet) // {
      git-msg = config.git-msg-bin;
      default = self.packages.git-msg;
    };
  };
in
self
