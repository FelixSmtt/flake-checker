{
  self,
  ...
}:

{
  flake.nixosModules.default =
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

      wrapper =
        pkgs.runCommand "flake-checker-wrapped"
          {
            nativeBuildInputs = [ pkgs.makeWrapper ];
          }
          ''
            mkdir -p $out/bin
            makeWrapper ${config.services.flake-checker.package}/bin/flake-checker $out/bin/flake-checker \
              --add-flags "-c ${configFile}"
          '';
    in
    {
      options.services.flake-checker = {
        enable = lib.mkEnableOption "flake-checker systemd service";
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
        environment.systemPackages = [ wrapper ];
        systemd.services.flake-checker = {
          description = "Flake Checker";
          wantedBy = [ "multi-user.target" ];
          after = [ "network.target" ];
          serviceConfig = {
            Type = "oneshot";
            ExecStart = "${config.services.flake-checker.package}/bin/flake-checker -c ${configFile}";
            WorkingDirectory = "/";
            Environment = "RUST_LOG=info";
            EnvironmentFile = lib.mkIf (
              config.services.flake-checker.environmentFile != null
            ) config.services.flake-checker.environmentFile;
          };
          install = {
            WantedBy = [ "multi-user.target" ];
          };
        };
      };
    };
}
