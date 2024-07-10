use std::process::Command;

pub fn check_7zip_installed() -> bool {
    let output = Command::new("winget")
        .args(["list", "7zip"])
        .output()
        .expect("Failed to execute winget");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("7zip")
}

pub fn install_7zip() -> bool {
    let output = Command::new("winget")
        .args(["install", "--id=7zip.7zip", "-e"])
        .spawn()
        .expect("Failed to execute winget");

    let output = output.wait_with_output().expect("Failed to wait for output");
    let output_str = String::from_utf8_lossy(&output.stdout);
    !output_str.contains("Found an existing package already installed")
}

pub fn check_directx_installed() -> bool {
    let output = Command::new("reg")
        .args(["query", "HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\DirectX"])
        .output()
        .expect("Failed to execute reg");

    let output_str = std::str::from_utf8(&output.stdout).expect("Failed to convert output to string");
    output_str.contains("DirectX")
}

// winget install -e --id Microsoft.DirectX
pub fn install_directx() -> bool {
    let output = Command::new("winget")
        .args(["install", "--id=Microsoft.DirectX", "-e"])
        .spawn()
        .expect("Failed to execute winget");

    let output = output.wait_with_output().expect("Failed to wait for output");
    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("Found an existing package already installed")
}

pub fn add_exclusion(install_dir: String) {
    Command::new("powershell")
        .arg("-c")
        .arg(format!("Add-MpPreference -ExclusionPath '{}\\sampcac_client.asi'", std::env::current_dir().unwrap().join(&install_dir).display()))
        .spawn()
        .expect("Failed to execute powershell");
}