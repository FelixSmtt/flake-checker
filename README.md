# flake-checker

**flake-checker** is a Rust-based tool for monitoring and notifying about upstream updates for Nix flakes. It checks your `flake.lock` for updates to specified inputs, notifies you via desktop notifications (with clickable actions), and can be integrated as a systemd service on NixOS or Home Manager.

---

## Features

- Checks for updates to specified Nix flake inputs (e.g., `nixpkgs`)
- Sends desktop notifications with customizable format and action button
- Easily configurable via a JSON config file
- Integrates as a systemd service (NixOS or Home Manager)

---

## Installation

### 1. NixOS

Add this flake as an input to your `flake.nix`:

```nix
{
  inputs.flake-checker = {
    url = "github:FelixSmtt/flake-checker";
    inputs.nixpkgs.follows = "nixpkgs";
  };
}
```

#### Home Manager (Recommended)

Add the Home Manager module:

```nix
{
  imports = [
    inputs.flake-checker.homeManagerModules.default
  ];

  services.flake-checker = {
    enable = true;
    config = {
      flake_lock_path = "/home/youruser/nixos/flake.lock";
      allowed_inputs = [ "nixpkgs" ];
    };
    environmentFile = "/run/user/1000/secrets/flake-checker.env"; # Optional, for the GITHUB_TOKEN secret
  };
}
```

#### NixOS Module

```nix
{
  imports = [
    inputs.flake-checker.nixosModules.default
  ];

  services.flake-checker = {
    enable = true;
    config = {
      flake_lock_path = "/etc/nixos/flake.lock";
      allowed_inputs = [ "nixpkgs" ];
    };
    environmentFile = "/run/secrets/flake-checker.env"; # Optional, for the GITHUB_TOKEN secret
  };
}
```

### 2. Building from Scratch

You need a recent Rust toolchain and Nix:

```sh
# Clone the repo
git clone https://github.com/FelixSmtt/flake-checker.git
cd flake-checker

# Run with Cargo
cargo run -c /path/to/config.json
```

---

## Configuration

The configuration is a JSON file (or Nix attrset if using the module) with the following options:

| Option                   | Type                | Description                                                                                       |
|--------------------------|---------------------|---------------------------------------------------------------------------------------------------|
| `flake_lock_path`        | string              | Path to the `flake.lock` file to check.                                                           |
| `allowed_inputs`         | list of strings     | List of flake input names to check for updates (e.g., `["nixpkgs"]`).                             |
| `notification_format`    | string (optional)   | Format string for notifications (see placeholders below).                                         |

### Placeholders for `notification_format`

You can use the following placeholders in your notification format string:

- `{name}`: The input name (e.g., `nixpkgs`)
- `{target_ref}`: The branch or ref being tracked (e.g., `master`)
- `{local_rev}`: The current revision (short SHA)
- `{remote_rev}`: The latest upstream revision (short SHA)

**Example (Default):**

```json
"notification_format": "• <b>{name}</b> ({target_ref})\n  {local_rev} → {remote_rev}\n"
```

### Example `.env` file for secrets

If you need to use a GitHub token for private repositories or higher API limits, create a file like:

```env
GITHUB_TOKEN=ghp_XXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

and set `environmentFile` to its path.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contributions

PRs and issues are welcome!
