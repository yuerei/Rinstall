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

use std::fs;
use std::fs::File;
use std::fs::{create_dir_all, read_dir, rename};
use std::io::BufReader;
use zip::ZipArchive;
use std::path::Path;

#[tauri::command]
async fn unzip_file(zip_path: String, temp_path: String, output_path: String) -> Result<(), String> {
    let file = File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(BufReader::new(file)).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = Path::new(&temp_path).join(file.name());

        if file.name().ends_with('/') {
            create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    println!("Extraction done to temp_path");

    // Now, move from temp_path to output_path
    fix_plugins_structure(temp_path, output_path)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn fix_plugins_structure(temp_path: String, output_path: String) -> Result<(), String> {
    let temp_path = Path::new(&temp_path);
    let output_path = Path::new(&output_path);

    println!("Fixing extracted files...");

    // Check if temp_path/plugins exists
    let nested_plugins_path = temp_path.join("plugins");

    let source_path = if nested_plugins_path.exists() {
        println!("Found nested 'plugins/' folder, using its contents.");
        nested_plugins_path
    } else {
        println!("No nested 'plugins/' folder, using extracted contents directly.");
        temp_path.to_path_buf()
    };

    for entry in read_dir(&source_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let from_path = entry.path();
        let file_name = from_path.file_name().ok_or("Invalid filename")?.to_owned();
        let to_path = output_path.join(file_name);

        println!("Moving {:?} -> {:?}", from_path, to_path);

        // Make sure parent exists
        if let Some(parent) = to_path.parent() {
            create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        rename(&from_path, &to_path).map_err(|e| e.to_string())?;
    }

    println!("Move completed!");

    Ok(())
}
