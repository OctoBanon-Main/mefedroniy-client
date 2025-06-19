use anyhow::Result;
use reqwest::header::USER_AGENT;
use semver::Version;
use serde::Deserialize;

use crate::updater::platform_check::target_suffix;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Assets>,
}

#[derive(Deserialize)]
pub struct Assets {
    pub name: String,
    pub browser_download_url: String,
}

fn find_asset_url(release: &Release) -> Option<String> {
    let suffix = target_suffix();

    release.assets.iter()
        .find(|asset| asset.name.ends_with(suffix))
        .map(|asset| asset.browser_download_url.clone())
}

pub async fn check_for_updates() -> Result<Option<Release>> {
    let release: Release = reqwest::Client::new()
        .get("https://api.github.com/repos/OctoBanon-Main/mefedroniy-client/releases/latest")
        .header(USER_AGENT, "mefedroniy-client-updater")
        .send()
        .await?
        .json()
        .await?;

    let latest_version = Version::parse(release.tag_name.trim_start_matches("v"))?;
    let current_version = Version::parse(CURRENT_VERSION)?;

    (latest_version > current_version)
        .then(|| {
            find_asset_url(&release).map_or_else(
                || println!("No suitable asset found for the current platform."),
                |url| {
                    println!("Update available!");
                    println!("Download link: {}", url);
                },
            );
    });



    Ok((latest_version > current_version).then_some(release))
}