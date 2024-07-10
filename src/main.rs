use std::io::{self, Read};
use std::process::{exit, Command};

use check_elevation::is_elevated;

mod os;
mod game;

fn main() {
    // The program needs to be ran as Administrator.
    // Mainly to add an exclusion for "sampcac_client.asi" in Windows Defender.
    if !is_elevated().expect("Failed to get elevation status.") {
        println!("Not running as Administrator. Please run the program as Administrator.\nThis is needed in order to add an exclusion for SAMPCAC.");
        wait_for_keypress();
        exit(1);
    }
    
    if !os::check_7zip_installed() {
        println!("Installing 7Zip...");
        if os::install_7zip() {
            println!("7Zip is installed.");
        } else {
            println!("Failed to Install 7Zip. Aborting...");

            wait_for_keypress();
            exit(1);
        }
    }

    if !os::check_directx_installed() { // Otherwise the game doesn't run (you need this on fresh windows installs)
        println!("Installing DirectX...");
        if os::install_directx() {
            println!("DirectX was installed.");
        } else {
            println!("Failed to Install DirectX. Aborting...");
            wait_for_keypress();
            exit(1);
        }
    }

    // Download the archive if not present in the current directory
    loop {
        if !game::is_archive_valid() {
            println!("Downloading GTA San Andreas...");
            game::download();

            // Verify the archive's integrity before proceeding
            if !game::is_archive_valid() {
                println!("Downloaded file is corrupted. Would you like to download again? Y/N (default: N)");

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                if input.trim().to_lowercase() != "y" {
                    println!("Aborting...");
                    wait_for_keypress();
                    exit(1);
                } else {
                    continue; 
                }
            }
        } else { break; }
    }
        
    let mut install_dir = String::from("GTA San Andreas").clone();
    loop {
        // Ask the user what they would like to call the extracted folder.
        // We'll default to "GTA San Andreas" if they don't provide anything.
        println!("What would you like to name the installation folder? (Press enter for the default: GTA San Andreas)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();

        // If the user provided a name, use it as the installation folder name.
        // Otherwise, keep the default name.
        if !input.is_empty() { install_dir = input; }

        // We need to make sure that if it's created, it's empty.
        if std::fs::metadata(&install_dir).is_ok() && std::fs::read_dir(&install_dir).unwrap().next().is_some() {
            println!("Directory is not empty. Would you like to choose another name? Y/N (default: N)");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim().to_lowercase() == "y" { continue; }
        }

        break;
    }
    
    println!("Extracting Game Archive to \"{}\"...", install_dir);
    let output = Command::new("7z")
        .args([
            "x",
            &("-o".to_string() + &install_dir),
            "gtasa.7z"
        ])
        .output()
        .expect("Failed to execute 7z");

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);

    // Find out of the directory was created (meaning it would've been a success)
    if !std::fs::metadata(&install_dir).is_ok() {
        println!("Failed to extract the archive. Aborting...");
        wait_for_keypress();
        exit(1);
    }

    // Add an exclusion for "sampcac_client.asi" in Windows Defender, as it's a false positive.
    println!("Adding exclusion for SAMPCAC...");
    os::add_exclusion(install_dir.clone());

    println!("Downloading SAMPCAC...");
    game::download_sampcac(install_dir);

    println!("\nInstallation completed!\nYou can now play GTA San Andreas by running the game executable in the installation folder.");

    wait_for_keypress();
    exit(0);
}

fn wait_for_keypress() {
    println!("Press enter to exit...");
    let _ = io::stdin().bytes().next();
}