mod config;
mod flake;
mod notifier;

use clap::Parser;
use config::Config;
use flake::{FlakeLock, FlakeNode, check_input_update};
use notifier::{UpdateInfo, format_update, send_notification};
use std::env;
use std::fs;
use std::path::Path;

/// Flake Checker
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to config file (overrides default)
    #[arg(short, long, value_name = "file")]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_path = args.config.unwrap_or_else(|| "config.json".to_string());
    let config = Config::load_from_file(&config_path)?;
    let lock_path = Path::new(&config.flake_lock_path);

    if !lock_path.exists() {
        eprintln!("Error: Target lock file does not exist at {:?}", lock_path);
        std::process::exit(1);
    }

    let lock_content = fs::read_to_string(lock_path)?;
    let lock_data: FlakeLock = serde_json::from_str(&lock_content)?;
    let github_token = env::var("GITHUB_TOKEN").ok();

    let client = reqwest::Client::builder()
        .user_agent("NixOS-Flake-Update-Checker-Daemon")
        .build()?;

    let target_node_ids = lock_data.target_node_ids(&config.allowed_inputs)?;

    println!(
        "Checking upstream updates for target nodes: {:?}...",
        target_node_ids
    );

    let mut updates = Vec::new();

    for node_id in target_node_ids {
        if let Some(FlakeNode::Dependency {
            locked, original, ..
        }) = lock_data.nodes.get(node_id)
        {
            match check_input_update(&client, github_token.as_deref(), node_id, locked, original)
                .await
            {
                Ok(status) => {
                    if let Some(entry) =
                        format_update(node_id, &status, &config.notification_format)
                    {
                        updates.push(UpdateInfo {
                            name: node_id.clone(),
                            message: entry,
                        });
                    }
                }
                Err(e) => eprintln!("Error probing node {}: {}", node_id, e),
            }
        }
    }

    if updates.is_empty() {
        println!("Everything is up to date. No updates found.");
    } else {
        println!("{} update(s) available:", updates.len());
        for update in &updates {
            let plain = update.message.replace("<b>", "").replace("</b>", "");
            println!("{}: {}", update.name, plain.trim());
        }
    }

    send_notification(
        updates,
        &config.flake_dir,
        config.terminal_action_label.as_deref(),
        config.terminal_action_command.as_deref(),
    );
    println!("Check complete.");
    Ok(())
}
