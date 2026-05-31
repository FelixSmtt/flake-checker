use crate::flake::UpdateStatus;
use notify_rust::Notification;
use std::env;
use std::path::Path;
use std::process::Command;

pub struct UpdateInfo {
    pub name: String,
    pub message: String,
}

pub fn send_notification(
    updates: Vec<UpdateInfo>,
    flake_dir: &str,
    action_label: Option<&str>,
    action_command: Option<&str>,
) {
    if updates.is_empty() {
        println!("Everything is up to date. No notifications sent.");
        return;
    }
    let notification_body = updates
        .iter()
        .map(|u| u.message.as_str())
        .collect::<String>();
    let flake_dir = Path::new(flake_dir).to_path_buf();

    let mut notification = Notification::new();
    let label = action_label.unwrap_or("Open Terminal");
    notification
        .summary("❄️ Nix Flake Updates Available")
        .body(&notification_body)
        .appname("Flake Checker")
        .icon("software-update-available")
        .timeout(0)
        .action("open_terminal", label);

    match notification.show() {
        Ok(handle) => {
            wait_for_action_with_timeout(handle, &flake_dir, action_command, 30);
        }
        Err(e) => {
            eprintln!("Failed to show notification: {}", e);
        }
    }
}

fn wait_for_action_with_timeout(
    handle: notify_rust::NotificationHandle,
    flake_dir: &std::path::Path,
    action_command: Option<&str>,
    timeout_secs: u64,
) {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();
    let flake_dir = flake_dir.to_path_buf();

    thread::spawn(move || {
        handle.wait_for_action(|action| {
            let _ = tx.send(action.to_string());
        });
    });

    if let Ok(action) = rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        if action == "open_terminal" {
            if let Some(cmd) = action_command {
                run_custom_terminal_command(cmd, &flake_dir);
            } else {
                open_terminal_with_prefill(&flake_dir);
            }
        }
    } else {
        eprintln!("Notification timed out after {} seconds.", timeout_secs);
        // Optionally: print a message or let the notification expire naturally
    }
}

fn run_custom_terminal_command(cmd: &str, dir: &Path) {
    use std::process::Command;
    let status = Command::new(cmd).current_dir(dir).spawn();
    if let Err(e) = status {
        eprintln!("Failed to launch custom terminal command: {}", e);
    }
}

fn open_terminal_with_prefill(dir: &Path) {
    let terminal = env::var("TERMINAL").unwrap_or_else(|_| "x-terminal-emulator".to_string());
    let prefill_cmd = "nix flake update";

    let status = if terminal.contains("gnome-terminal") {
        Command::new(&terminal)
            .arg("--working-directory")
            .arg(dir)
            .arg("--")
            .arg("bash")
            .arg("-c")
            .arg(format!("read -e -p '$ ' -i '{}'", prefill_cmd))
            .spawn()
    } else if terminal.contains("alacritty") {
        Command::new(&terminal)
            .arg("--working-directory")
            .arg(dir)
            .arg("-e")
            .arg("bash")
            .arg("-c")
            .arg(format!("read -e -p '$ ' -i '{}'", prefill_cmd))
            .spawn()
    } else if terminal.contains("konsole") {
        Command::new(&terminal)
            .arg("--workdir")
            .arg(dir)
            .arg("-e")
            .arg("bash")
            .arg("-c")
            .arg(format!("read -e -p '$ ' -i '{}'", prefill_cmd))
            .spawn()
    } else {
        Command::new(&terminal).current_dir(dir).spawn()
    };

    if let Err(e) = status {
        eprintln!("Failed to launch terminal: {}", e);
    }
}

pub fn format_update(node_id: &str, status: &UpdateStatus, template: &str) -> Option<String> {
    if let UpdateStatus::UpdateAvailable {
        target_ref,
        local_rev,
        remote_rev,
    } = status
    {
        Some(
            template
                .replace("{name}", node_id)
                .replace("{target_ref}", target_ref)
                .replace("{local_rev}", local_rev)
                .replace("{remote_rev}", remote_rev),
        )
    } else {
        None
    }
}
