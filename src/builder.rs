use std::fs;
use std::path::Path;

use walkdir::WalkDir;
use fs_extra::file::{copy, CopyOptions};

use crate::js_processor::transpile_js;
use crate::manifest::Manifest;

pub fn build_project(input: &str, output: &str, manifest: &mut Manifest) {
    let input_path = Path::new(input);
    let output_path = Path::new(output);

    fs::create_dir_all(output_path).unwrap();

    for entry in WalkDir::new(input_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let rel_path = path.strip_prefix(input_path).unwrap();
            let out_path = output_path.join(rel_path);

            match ext {
                "js" => {
                    fs::create_dir_all(out_path.parent().unwrap()).unwrap();
                    let js = fs::read_to_string(path).unwrap();
                    let compiled = transpile_js(&js, path.display().to_string());

                    fs::write(&out_path, compiled.as_bytes()).unwrap();

                    manifest.add_entry(rel_path, &out_path.strip_prefix(output_path).unwrap());
                }
                "php" => {
                    fs::create_dir_all(out_path.parent().unwrap()).unwrap();
                    copy(path, &out_path, &CopyOptions::new()).unwrap();
                }
                _ => {}
            }
        }
    }
}
