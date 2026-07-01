{ lib, ... }:

with lib;

{
  options = {
    flake_lock_path = mkOption {
      type = types.str;
      description = "Path to the flake.lock file to check.";
    };
    allowed_inputs = mkOption {
      type = types.listOf types.str;
      default = [ "nixpkgs" ];
      description = "List of allowed input names to check for updates.";
    };
    notification_format = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = "Notification format string with placeholders.";
    };
  };
}
