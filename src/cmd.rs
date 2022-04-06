use anyhow::{anyhow, Result};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{self, Cursor},
    path::Path,
};
use tar::Archive;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatestReleaseResponse {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<Asset>,
    pub tarball_url: String,
    pub zipball_url: String,
    // pub body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

static APP_USER_AGENT: &str = "KCPTUN_UI v1";

async fn check_bin_and_download() -> Result<String> {
    let os = match env::consts::OS {
        "windows" => "windows",
        "linux" => "linux",
        "macos" => "darwin",
        _ => "",
    };
    let arch = match env::consts::ARCH {
        "x86" => "386",
        "x86_64" => "amd64",
        _ => "",
    };

    if os.chars().count() == 0 || arch.chars().count() == 0 {
        return Err(anyhow!("Cannot determine `OS`/`ARCH`"));
    }

    let bin_name = format!("client_{}_amd64.exe", os);
    let bin_path = std::env::current_dir().unwrap().join("bin").join(bin_name);
    println!("{}", bin_path.to_string_lossy());
    if bin_path.exists() {
        return Ok(String::from(bin_path.to_string_lossy()));
    }

    let tag = format!("{}-{}", os, arch);
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let json = client
        .get("https://api.github.com/repos/xtaci/kcptun/releases/latest")
        .send()
        .await?
        .json::<LatestReleaseResponse>()
        .await?;

    let mut asset = &Asset::default();
    for a in json.assets.iter() {
        if a.name.contains(&tag) {
            asset = &a;
            break;
        }
    }
    let download_url = &asset.browser_download_url;
    if download_url.chars().count() == 0 {
        return Err(anyhow!("cannot find bin for {} from latest release", tag));
    }

    let resp = client.get(download_url).send().await?;
    let mut gz_file = Cursor::new(resp.bytes().await?);

    let tar_path = Path::new(".").join("bin").join(&asset.name);
    fs::create_dir_all(tar_path.parent().unwrap())?;
    let mut download_tar = File::create(&tar_path)?;
    io::copy(&mut gz_file, &mut download_tar)?;

    let tar_gz = File::open(tar_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack("./bin")?;
    Ok(String::from(bin_path.to_string_lossy()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bin() {
        match check_bin_and_download().await {
            Ok(p) => println!("== check ok ==, {}", p),
            Err(e) => println!("{:?}", e),
        }
    }
}
