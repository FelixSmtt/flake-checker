{ lib, ... }:

with lib;

{
  options = {
    flake_lock_path = mkOption {
      type = types.str;
      description = "Path to the flake.lock file to check.";
    };
    flake_dir = mkOption {
      type = types.str;
      description = "Path to the flake dir to open in terminal action.";
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
    terminal_action_label = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = "Label for the notification action button.";
    };
    terminal_action_command = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = "Command to run when the notification action is triggered.";
    };
  };
}
