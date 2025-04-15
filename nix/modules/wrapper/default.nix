{
  pkgs,
  lib,
  config,
  inputs',
  ...
}:
let
  message = lib.types.submodule {
    options = {
      style = lib.mkOption {
        type = lib.types.nullOr lib.types.str;
        default = "conventional";
        description = "The Git commit message format style";
        example = "conventional";
      };
      historyLength = lib.mkOption {
        type = lib.types.nullOr lib.types.number;
        default = null;
        description = "The length of the git history to be used for generating commit messages";
        example = 5;
      };
      modelType = lib.mkOption {
        type = lib.types.str;
        default = "deepseek-chat";
        description = "The LLM model type to be used for generating commit messages";
        example = "deepseek-chat"; # Or "deepseek-reasoner"
      };
    };
  };

  pkgsType = lib.types.submodule {
    options = {
      format = lib.mkOption {
        type = message;
        default = { };
        description = "The format style of this pacakge";
      };

      description = lib.mkOption {
        type = lib.types.str;
        default = "";
        description = "Description of this packages";
      };
    };
  };
in
{
  options = {
    git-msg-bin = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./../package/git-msg.nix { };
      description = "The Git commit message generator";
    };

    packagesSet = lib.mkOption {
      type = lib.types.attrsOf pkgsType;
      default = { };
      description = "The available packages for this module";
    };

    defaultPackage = lib.mkOption {
      type = lib.types.package;
      description = "The default package to be used for this module";
    };

    mkGitMsgWrapper = lib.mkOption {
      type = lib.types.functionTo (lib.types.functionTo lib.types.package);
      description = "function to wrap different Git commit message generator in shell scripts";
    };
  };

  config = {
    packagesSet = {
      conventional = {
        format = {
          style = "conventional";
          modelType = "deepseek-chat";
          historyLength = 5;
        };
        description = "Git commit message generator with conventional style";
      };

      bracketed = {
        format = {
          style = "bracketed";
          modelType = "deepseek-chat";
          historyLength = 5;
        };
        description = "Git commit message generator with bracketed style";
      };

      plain = {
        format = {
          style = "plain";
          modelType = "deepseek-chat";
          historyLength = 5;
        };
        description = "Git commit message generator with plain style";
      };
    };

    mkGitMsgWrapper =
      name: pkg:
      let
        formatConfig = pkg.format;
        scriptsParams = [
          "--model=${formatConfig.modelType}"
          (lib.optionals (formatConfig.style != null) "--format=${formatConfig.style}")
          (lib.optionals (
            formatConfig.historyLength != null
          ) "--git-history=${lib.toHexString formatConfig.historyLength}")
        ];

      in
      pkgs.writeShellApplication {
        name = "git-msg-${name}";
        runtimeInputs = with pkgs; [
          coreutils
          git
          config.git-msg-bin
        ];
        text = ''
          echo "Welcome to Git Commit Generator Modules!"
          echo "Format: ${formatConfig.style}"
          echo "Git Commit Message Generator: ${config.git-msg-bin}"
          echo "Params: ${lib.concatStringsSep " " scriptsParams}"
          ${config.git-msg-bin}/bin/git-msg ${lib.concatStringsSep " " scriptsParams}
        '';
      };

    defaultPackage =
      let
        formatName = "conventional";
      in
      config.mkGitMsgWrapper formatName config.packagesSet.${formatName};
  };
}
