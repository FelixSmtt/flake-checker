{
  self,
  ...
}:

{
  flake.homeManagerModules.default =
    {
      config,
      lib,
      pkgs,
      ...
    }:

    let
      configModule = import ./config.nix { inherit lib; };
      cleanNulls =
        value:
        if builtins.isAttrs value then
          builtins.listToAttrs (
            builtins.filter (x: x.value != null) (
              builtins.attrValues (
                builtins.mapAttrs (k: v: {
                  name = k;
                  value = cleanNulls v;
                }) value
              )
            )
          )
        else if builtins.isList value then
          map cleanNulls value
        else
          value;
      configFile = pkgs.writeText "flake-checker-config.json" (
        builtins.toJSON (cleanNulls config.services.flake-checker.config)
      );
    in
    {
      options.services.flake-checker = {
        enable = lib.mkEnableOption "flake-checker user service";
        package = lib.mkOption {
          type = lib.types.package;
          default = self.packages.${pkgs.system}.default;
          description = "The flake-checker package to use for the service.";
        };
        config = lib.mkOption {
          type = lib.types.submodule configModule;
          description = "flake-checker JSON config";
        };
        environmentFile = lib.mkOption {
          type = lib.types.nullOr lib.types.path;
          default = null;
          description = ''
            Path to a file in .env format (e.g. GITHUB_TOKEN=...) that will be passed to the service via systemd's EnvironmentFile.
            You can use a secrets file or a generated file.
          '';
        };
      };
      config = lib.mkIf config.services.flake-checker.enable {
        home.packages = [ config.services.flake-checker.package ];
        systemd.user.services.flake-checker = {
          Unit = {
            Description = "Flake Checker";
            After = [ "graphical-session.target" ];
          };
          Service = {
            Type = "oneshot";
            ExecStart = "${config.services.flake-checker.package}/bin/flake-checker -c ${configFile}";
            WorkingDirectory = "%h";
            Environment = "RUST_LOG=info";
            EnvironmentFile = lib.mkIf (
              config.services.flake-checker.environmentFile != null
            ) config.services.flake-checker.environmentFile;
          };
          Install = {
            WantedBy = [ "default.target" ];
          };
        };
      };
    };
}
