#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![unzip_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[tauri::command]
async fn unzip_file(zip_path: String, temp_path: String, output_path: String) -> Result<(), String> {
    println!("Starting extraction...");

    let file = File::open(&zip_path).map_err(|e| format!("Failed to open zip: {}", e))?;
    let mut archive = ZipArchive::new(BufReader::new(file)).map_err(|e| format!("Failed to read zip archive: {}", e))?;

    // Extract into temp_path
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to access file in archive: {}", e))?;
        let outpath = Path::new(&temp_path).join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    println!("Extraction completed. Checking for plugins folder...");
    fix_plugins_structure(&temp_path, &output_path)?;

    println!("Cleaning up temp folder...");
    if Path::new(&temp_path).exists() {
        fs::remove_dir_all(&temp_path).map_err(|e| e.to_string())?;
    }
    if Path::new(&zip_path).exists() {
        fs::remove_file(&zip_path).map_err(|e| e.to_string())?;
    }    

    println!("Done!");
    Ok(())
}

fn fix_plugins_structure(temp_path: &str, output_path: &str) -> Result<(), String> {
    let temp_dir = Path::new(temp_path);
    let output_dir = Path::new(output_path);

    let nested_plugins_path = temp_dir.join("plugins");

    let source_dir: PathBuf = if nested_plugins_path.exists() && nested_plugins_path.is_dir() {
        println!("Found nested 'plugins/' folder inside extracted content.");
        nested_plugins_path
    } else {
        println!("No nested 'plugins/' folder found. Using root extracted folder.");
        temp_dir.to_path_buf()
    };

    if !source_dir.exists() {
        return Err("Source directory does not exist.".into());
    }

    fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;

    for entry in fs::read_dir(&source_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let from_path = entry.path();
        let file_name = from_path.file_name().ok_or("Invalid file name")?.to_owned();
        let to_path = output_dir.join(file_name);

        if from_path.is_dir() {
            // Recursively merge folders
            copy_dir_recursively(&from_path, &to_path)?;
        } else {
            // Overwrite files
            fs::copy(&from_path, &to_path).map_err(|e| e.to_string())?;
        }
    }

    println!("Plugins moved successfully.");
    Ok(())
}

fn copy_dir_recursively(from: &Path, to: &Path) -> Result<(), String> {
    if !to.exists() {
        fs::create_dir_all(to).map_err(|e| e.to_string())?;
    }

    for entry in fs::read_dir(from).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let from_path = entry.path();
        let to_path = to.join(entry.file_name());

        if from_path.is_dir() {
            copy_dir_recursively(&from_path, &to_path)?;
        } else {
            fs::copy(&from_path, &to_path).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
