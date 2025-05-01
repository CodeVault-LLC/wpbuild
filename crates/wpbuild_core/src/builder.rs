use std::fs;
use std::path::Path;

use crate::bundler::bundle_entry;
use crate::manifest::Manifest;

pub fn build_project(
    input: &str, 
    output: &str, 
    manifest: &mut Manifest,
) -> Result<(), String> {
    let input_file = Path::new(input);
    let output_path = Path::new(output);

    if let Err(e) = fs::create_dir_all(output_path) {
        return Err(format!("Failed to create output directory: {}", e));
    }

    if !input_file.exists() {
        return Err(format!("Input file does not exist: {}", input));
    }

    if !input_file.is_file() {
        return Err(format!("Input file is not a file: {}", input));
    }

    let input_path = input_file.canonicalize().map_err(|e| {
        format!("Failed to canonicalize input file path: {}", e)
    })?;
    let input_path = input_path
        .to_str()
        .ok_or("Failed to convert input path to string")?;

    // Use the new bundle_entry_with_format function with the specified module format
    let compiled_js = bundle_entry(&input_file.display().to_string()).map_err(|e| {
        format!("Failed to bundle JS/TS files: {}", e)
    })?;
 
    let compiled_js_path = output_path.join("combined.js");

    if let Err(e) = fs::write(&compiled_js_path, compiled_js) {
        return Err(format!("Failed to write compiled JS/TS file: {}", e));
    }
    manifest.add_entry(
        Path::new(input_path),
        &compiled_js_path,
    );

    Ok(())
}
