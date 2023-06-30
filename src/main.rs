use anyhow::anyhow;
use anyhow::Result;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use std::process::Command;

fn main() -> Result<()> {
    let mut random = rand::thread_rng();
    // look for a rar file at arg[0]
    let src_dir = std::env::args()
        .nth(1)
        .unwrap_or("nonexistant_dir".to_string());
    // if src_dir doesn't exist, return
    if !std::path::Path::new(&src_dir).exists() {
        println!("Src path does not exist: '{:?}'", src_dir);
        return Ok(());
    }

    // check for any rar files in the folder.
    let files = std::fs::read_dir(&src_dir)?;
    let mut rar_files = Vec::new();
    for file in files {
        let file = file?;
        let path = file.path();
        if path.is_file() {
            let ext = path.extension();
            if ext.is_some() && ext.unwrap() == "rar" {
                rar_files.push(path);
            }
        }
    }

    if rar_files.len() == 0 {
        println!("No rar files found in directory. Skipping. {:?}", src_dir);
        return Ok(());
    }

    if !file_exists("/usr/bin/7z") {
        println!("7z not found, installing...");
        //pacman -S p7zip --noconfirm
        let output = cmd(".", "pacman", vec!["-S", "p7zip", "--noconfirm"])?;
        println!("Output: {}", output);
    }

    // generate a random directory, and extract the rar files to it.
    let temp_dir_name = Alphanumeric.sample_string(&mut random, 10);

    // get parent path of src_dir
    let parent_path = std::path::Path::new(&src_dir).parent().unwrap();

    let full_dir = format!("{}/{}", parent_path.to_str().unwrap(), temp_dir_name);
    // create the directory
    std::fs::create_dir(&full_dir)?;

    let output = cmd(src_dir.as_str(), "7z", vec!["-y", format!("-o{}", full_dir).as_str(), "x", "*.rar"])?;
    println!("Output: {}", output);

    // move all files from full_dir to src_dir
    let items = std::fs::read_dir(&full_dir)?;
    for item in items {
        let item = item?;
        let path = item.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            let file_name = file_name.to_str().unwrap();
            let new_path = format!("{}/{}", src_dir, file_name);
            println!("Moving {} to {}", file_name, new_path);
            let result = std::fs::rename(&path, &new_path);
            if result.is_err() {
                println!("Error moving file: {:?}", result);
                println!("Trying method 2: copy and delete");
                let result = std::fs::copy(&path, &new_path);
                if result.is_err() {
                    println!("Error copying file: {:?}", result);
                } else {
                    let result = std::fs::remove_file(&path);
                    if result.is_err() {
                        println!("Error deleting file: {:?}", result);
                    }
                }
            }
        } else {
            println!("Skipping directory: {:?}", path);
        }
    }

    // delete the temp dir
    std::fs::remove_dir_all(std::path::Path::new(&full_dir))?;

    println!("Done! {}", temp_dir_name);

    Ok(())
}

fn file_exists(path: &str) -> bool {
    return std::path::Path::new(path).exists();
}

fn cmd(working_dir: &str, program: &str, args: Vec<&str>) -> Result<String> {
    println!("Executing cmd: {} {:?}", program, args);
    let mut command = Command::new(program);
    command.args(args);
    command.current_dir(working_dir);
    let output = command.output().expect("Failed to execute process!");
    let status = command.status().expect("Failed to execute process!");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !status.success() {
        return Err(anyhow!(format!(
            "Error while running: (status: {:?}) {}, {}",
            status.code(),
            program,
            stderr
        )));
    }

    return Ok(stdout);
}
