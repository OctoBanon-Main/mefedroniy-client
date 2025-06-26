use anyhow::Result;
use reqwest::header::USER_AGENT;
use semver::Version;
use serde::Deserialize;
use colored::*;

use crate::updater::platform_check::target_suffix;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize)]
pub struct Release { pub tag_name: String, pub assets: Vec<Asset> }
#[derive(Deserialize)]
pub struct Asset { pub name: String, pub browser_download_url: String }

fn find_asset_url(release: &Release) -> Option<String> {
    let suffix = target_suffix();
    release.assets.iter().find(|a| a.name.ends_with(&suffix)).map(|a| a.browser_download_url.clone())
}

pub async fn check_for_updates() -> Result<Option<Release>> {
    let release: Release = reqwest::Client::new()
        .get("https://api.github.com/repos/OctoBanon-Main/mefedroniy-client/releases/latest")
        .header(USER_AGENT, "mefedroniy-client-updater")
        .send().await?.json().await?;

    let latest = Version::parse(release.tag_name.trim_start_matches('v'))?;
    let current = Version::parse(CURRENT_VERSION)?;
    if latest <= current { return Ok(None); }

    let url_opt = find_asset_url(&release);
    let link_text = "[Link]";

    let raw_lines = [
        "Update available!",
        &format!("Current: {}", current),
        &format!("Latest:  {}", latest),
        &url_opt.as_ref().map(|_| format!("Download: {}", link_text)).unwrap_or_else(|| "No suitable asset found for this platform.".into()),
    ];

    let width = raw_lines.iter().map(|s| s.len()).max().unwrap_or(0) + 4;
    let border = "═".repeat(width);
    println!("╔{}╗", border.magenta());

    let ansi_link = |text: &str| format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", url_opt.as_ref().unwrap(), text);

    for (i, line) in raw_lines.iter().enumerate() {
        let centered = center(line, width);
        let styled = match (i, url_opt.as_ref()) {
            (0, _) => centered.bold().green().to_string(),
            (3, Some(_)) => centered.replace(link_text, &ansi_link(link_text)),
            _ => centered.white().to_string(),
        };
        println!("║{}║", styled);
    }

    println!("╚{}╝", border.magenta());
    Ok(Some(release))
}

fn center(text: &str, width: usize) -> String {
    let pad = width.saturating_sub(text.len());
    let left = pad / 2; let right = pad - left;
    " ".repeat(left) + text + &" ".repeat(right)
}
