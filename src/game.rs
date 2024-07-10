use std::process::Command;

const ARCHIVE_SIZE: u64 = 561_971_470;

pub fn is_archive_valid() -> bool {
    if !std::fs::metadata("gtasa.7z").is_ok() { return false; }
    
    let metadata = std::fs::metadata("gtasa.7z").expect("Failed to read metadata");

    if metadata.len() != ARCHIVE_SIZE {
        println!("Archive size is not the expected size.");
        println!("Archive Size: {}, Expected Size: {}", metadata.len(), ARCHIVE_SIZE);
    }
    
    return true;
}

pub fn download() {
    let output = Command::new("powershell")
        .args(["-c", "Invoke-WebRequest -Uri 'https://gta.flaviopereira.dev/sa/gtasa.7z' -OutFile 'gtasa.7z'"])
        .output()
        .expect("Failed to execute powershell");

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
}

pub fn download_sampcac(install_dir: String) {
    Command::new("powershell")
        .arg("-c")
        .arg(format!("Invoke-WebRequest -Uri 'https://gta.flaviopereira.dev/sa/samp/asi/sampcac_client.asi' -OutFile '{}\\sampcac_client.asi'", install_dir))
        .output()
        .expect("Failed to execute powershell");
}