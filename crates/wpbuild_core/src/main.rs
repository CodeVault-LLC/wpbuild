mod builder;
mod manifest;
mod processors;
mod config;
mod bundler;

use std::{env::current_dir, path::PathBuf};
use std::time::Instant;

use manifest::Manifest;
use config::config::{Config, load_config};

fn main() {
    let start_time = Instant::now();

    let current_dir: PathBuf = current_dir().expect("Failed to get current directory");

    let config: Config = match load_config(current_dir.as_path()) {
        Ok(cfg) => cfg,
        Err(error) => {
            println!("Failed to load configuration. Error: {}", error);
            println!("❌ Script failed. It took {:?}.", start_time.elapsed());
            return;
        }
    };

    let mut manifest: Manifest = Manifest::new();

    // Use build_project_with_format with the specified module format
    if let Err(error) = builder::build_project(
        config.entry.as_str(),
        config.output.as_str(),
        &mut manifest,
    ) {
        println!("Failed to build project. Error: {}", error);
        println!("❌ Script failed. It took {:?}.", start_time.elapsed());
        return;
    }

    if let Err(error) = manifest.save_to(config.output.as_str()) {
        println!("Failed to write manifest. Error: {}", error);
        println!("❌ Script failed. It took {:?}.", start_time.elapsed());
        return;
    }

    println!("✅ Build complete. Manifest written. It took {:?}.", start_time.elapsed());
}
