use crate::flake::UpdateStatus;
use notify_rust::Notification;

pub struct UpdateInfo {
    pub name: String,
    pub message: String,
}

pub fn send_notification(updates: Vec<UpdateInfo>) {
    if updates.is_empty() {
        println!("Everything is up to date. No notifications sent.");
        return;
    }
    let notification_body = updates
        .iter()
        .map(|u| u.message.as_str())
        .collect::<String>();

    let mut notification = Notification::new();
    notification
        .summary("❄️ Nix Flake Updates Available")
        .body(&notification_body)
        .appname("Flake Checker")
        .icon("software-update-available")
        .timeout(0);

    match notification.show() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to show notification: {}", e);
        }
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
