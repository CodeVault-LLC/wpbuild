use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;
use anyhow::{Result, bail, Context};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub entry: String,
    pub output: String,

    #[serde(default)]
    pub externals: Vec<String>,

    #[serde(default)]
    pub php: Option<PhpConfig>,

    #[serde(default)]
    pub assets: Option<AssetConfig>,
}

impl Config { }

#[derive(Debug, Deserialize, Clone)]
pub struct PhpConfig {
    pub copy: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AssetConfig {
    pub emit_asset_php: bool,
}

pub fn load_config(project_dir: &Path) -> Result<Config> {
    let config_paths = [
        project_dir.join("wpbuild.config.js"),
        project_dir.join("wpbuild.config.json"),
        project_dir.join("package.json"),
    ];

    // 1. Try wpbuild.config.js
    if config_paths[0].exists() {
        return load_js_config(&config_paths[0]);
    }

    // 2. Try wpbuild.config.json
    if config_paths[1].exists() {
        return load_json_config(&config_paths[1]);
    }

    // 3. Try package.json
    if config_paths[2].exists() {
        return load_package_json_config(&config_paths[2]);
    }

    let error_message = format!(
        "No configuration file found in {}. Expected one of: wpbuild.config.js, wpbuild.config.json, or package.json",
        project_dir.display()
    );

    let error_message = format!("\x1b[31m{}\x1b[0m", error_message); // Red color for error message

    bail!(error_message)
}

fn load_json_config(path: &Path) -> Result<Config> {
    let content: String = fs::read_to_string(path).context("Failed to read wpbuild.config.json")?;
    let json: Value = serde_json::from_str(&content).context("Failed to parse wpbuild.config.json")?;
    let config: Config = serde_json::from_value(json).context("Failed to parse wpbuild.config.json")?;
    
    if config.entry.is_empty() || config.output.is_empty() {
        bail!("`entry` and `output` fields are required in wpbuild.config.json");
    }
    
    Ok(config)
}

fn load_package_json_config(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path).context("Failed to read package.json")?;
    let json: Value = serde_json::from_str(&content).context("Failed to parse package.json")?;

    if let Some(cfg) = json.get("wpbuild") {
        let config: Config = serde_json::from_value(cfg.clone())
            .context("Failed to parse wpbuild config in package.json")?;
        Ok(config)
    } else {
        bail!("package.json found, but missing `wpbuild` field");
    }
}

fn load_js_config(path: &Path) -> Result<Config> {
    let output = Command::new("node")
        .arg("-e")
        .arg(format!("console.log(JSON.stringify(require('{}')))", path.display()))
        .output()
        .context("Failed to execute Node.js to parse wpbuild.config.js")?;

    if !output.status.success() {
        bail!(
            "Node.js config execution failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let json_output = String::from_utf8(output.stdout)?;
    let config: Config = serde_json::from_str(&json_output)
        .context("Failed to parse wpbuild.config.js output")?;
    Ok(config)
}
