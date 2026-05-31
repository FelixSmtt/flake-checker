use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct FlakeLock {
    pub nodes: HashMap<String, FlakeNode>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FlakeNode {
    Root {
        inputs: HashMap<String, String>,
    },
    Dependency {
        locked: LockedDetails,
        original: OriginalDetails,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct LockedDetails {
    #[serde(rename = "type")]
    pub repo_type: String,
    pub rev: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OriginalDetails {
    pub owner: Option<String>,
    pub repo: Option<String>,
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GithubCommit {
    pub sha: String,
}

pub enum UpdateStatus {
    UpToDate,
    UpdateAvailable {
        target_ref: String,
        local_rev: String,
        remote_rev: String,
    },
    NoOp,
}

impl FlakeLock {
    pub fn target_node_ids<'a>(
        &'a self,
        allowed_inputs: &[String],
    ) -> Result<Vec<&'a String>, String> {
        if let Some(FlakeNode::Root { inputs }) = self.nodes.get("root") {
            Ok(inputs
                .iter()
                .filter(|(input_name, _)| allowed_inputs.contains(input_name))
                .map(|(_, node_id)| node_id)
                .collect())
        } else {
            Err("Could not find or parse the 'root' node entry.".to_string())
        }
    }
}

pub async fn check_input_update(
    client: &reqwest::Client,
    token: Option<&str>,
    name: &str,
    locked: &LockedDetails,
    original: &OriginalDetails,
) -> Result<UpdateStatus, Box<dyn std::error::Error>> {
    if locked.repo_type != "github" {
        return Ok(UpdateStatus::NoOp);
    }

    let owner = original.owner.as_deref().unwrap_or("");
    let repo = original.repo.as_deref().unwrap_or("");
    let target_ref = original.git_ref.as_deref().unwrap_or("master");

    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        owner, repo, target_ref
    );

    let mut req = client.get(&url);
    if let Some(tok) = token {
        req = req.bearer_auth(tok);
    }

    let response = req.send().await?;
    if !response.status().is_success() {
        eprintln!(
            "Failed to fetch data for [{}]: Status {}",
            name,
            response.status()
        );
        return Ok(UpdateStatus::NoOp);
    }

    let latest_commit: GithubCommit = response.json().await?;

    if latest_commit.sha != locked.rev {
        return Ok(UpdateStatus::UpdateAvailable {
            target_ref: target_ref.to_string(),
            local_rev: locked.rev[..7].to_string(),
            remote_rev: latest_commit.sha[..7].to_string(),
        });
    }

    Ok(UpdateStatus::UpToDate)
}
