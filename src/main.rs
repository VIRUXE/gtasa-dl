use std::process::{exit, Command};
use std::str;

fn main() {
    if !check_7zip_installed() {
        println!("Installing 7Zip...");
        if install_7zip() {
            println!("7Zip is installed.");
        } else {
            println!("Failed to Install 7Zip. Aborting...");

            let _ = std::io::stdin().read_line(&mut String::new());
            exit(1);
        }
    }

    if !check_directx_installed() {
        println!("Installing DirectX...");
        if install_directx() {
            println!("DirectX is installed.");
        } else {
            println!("Failed to Install DirectX. Aborting...");

            let _ = std::io::stdin().read_line(&mut String::new());
            exit(1);
        }
    }

    let files = std::fs::read_dir(".").expect("Failed to read directory");
    if files.count() == 1 {
        println!("Downloading GTA San Andreas...");
        download_gta_sa();

        if !is_game_archive_present() {
            println!("Downloaded file is corrupted. Aborting...");
            std::fs::remove_file("gtasa.7z").expect("Failed to remove corrupted file");

            let _ = std::io::stdin().read_line(&mut String::new());
            exit(1);
        }

        println!("Extracting with 7Zip...");
        let output = Command::new("7z")
            .args(["x", "gtasa.7z"])
            .output()
            .expect("Failed to execute 7z");

        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);

        println!("\nInstallation completed!\nWould you like to open SA-MP? Y/N (default: Y)");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() == "n" { exit(0); }

        Command::new("samp").spawn().expect("Failed to start SA-MP");
    } else {
        println!("Directory is not empty. Skipping installation.");
    }
}

fn check_7zip_installed() -> bool {
    let output = Command::new("winget")
        .args(["list", "7zip"])
        .output()
        .expect("Failed to execute winget");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("7zip")
}

fn install_7zip() -> bool {
    let output = Command::new("winget")
        .args(["install", "--id=7zip.7zip", "-e"])
        .output()
        .expect("Failed to execute winget");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("Found an existing package already installed")
}

fn check_directx_installed() -> bool {
    let output = Command::new("reg")
        .args(["query", "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\DirectX"])
        .output()
        .expect("Failed to execute reg");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    output_str.contains("DirectX")
}

// winget install -e --id Microsoft.DirectX
fn install_directx() -> bool {
    let output = Command::new("winget")
        .args(["install", "--id=Microsoft.DirectX", "-e"])
        .output()
        .expect("Failed to execute winget");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("Found an existing package already installed")
}

fn check_gta_sa_installed() -> bool {
    let output = Command::new("reg")
        .args(["query", "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{7A4F5F6C-9F03-4E63-8D60-3E610A3B2CDB}"])
        .output()
        .expect("Failed to execute reg");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    output_str.contains("Grand Theft Auto: San Andreas")
}

fn is_game_archive_present() -> bool {
    let metadata = std::fs::metadata("gtasa.7z").expect("Failed to get metadata");
    metadata.len() == 567_543_883 // 541 MB - Which is my repack size
}

fn download_gta_sa() {
    let output = Command::new("powershell")
        .args(["-c", "Invoke-WebRequest -Uri 'https://gta.flaviopereira.dev/sa/gtasa.7z' -OutFile 'gtasa.7z'"])
        .output()
        .expect("Failed to execute powershell");

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
}