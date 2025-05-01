use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub files: HashMap<String, String>,
}

impl Manifest {
    pub fn new() -> Self {
        Manifest {
            files: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, input_path: &Path, output_path: &Path) {
        let input_str = input_path.to_string_lossy().to_string();
        let output_str = output_path.to_string_lossy().to_string();
        self.files.insert(input_str, output_str);
    }

    pub fn save_to(&self, output_dir: &str) -> Result<(), String> {
        let manifest_path = PathBuf::from(output_dir).join("asset-manifest.json");
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(&manifest_path, json).map_err(|e| format!("Failed to write manifest: {}", e))?;
        Ok(())
    }
}
