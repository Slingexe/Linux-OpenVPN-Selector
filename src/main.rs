use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct VpnFile {
    name: String,
    path: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    vpn_files: Vec<VpnFile>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !is_running_as_sudo() {
        println!("This program requires sudo privileges. Please run it with 'sudo'.");
        return Ok(());
    }

    // Determine the path to the directory where the executable is located
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get the executable directory");
    let config_path = exe_dir.join("config.json");

    // Check if the config file exists, and create it with default values if not
    if !config_path.exists() {
        println!("Configuration file not found. Creating a default config file...");

        let default_config = Config {
            vpn_files: vec![
                VpnFile {
                    name: String::from("Example VPN 1"),
                    path: String::from("/path/to/example_vpn1.ovpn"),
                },
                VpnFile {
                    name: String::from("Example VPN 2"),
                    path: String::from("/path/to/example_vpn2.ovpn"),
                },
            ],
        };

        let default_config_content = serde_json::to_string_pretty(&default_config)?;
        fs::write(&config_path, default_config_content)?;
        println!(
            "Default configuration file created at '{}'. Please update it as needed.",
            config_path.display()
        );
    }

    // Read the configuration file
    let config_content = fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&config_content)?;

    // Display options to the user
    println!("Select a VPN file to run:");
    for (i, vpn) in config.vpn_files.iter().enumerate() {
        println!("{}: {} ({})", i + 1, vpn.name, vpn.path);
    }

    // Get user input
    print!("Enter your choice (number): ");
    io::stdout().flush()?; // Ensure the prompt is printed
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(0);

    if choice == 0 || choice > config.vpn_files.len() {
        println!("Invalid choice.");
        return Ok(());
    }

    let selected_vpn = &config.vpn_files[choice - 1];
    println!("You selected: {} ({})", selected_vpn.name, selected_vpn.path);

    // Run the selected VPN file in the foreground and display the output
    let status = Command::new("openvpn")
        .arg(&selected_vpn.path)
        .status()
        .expect("failed to execute openvpn");

    if !status.success() {
        eprintln!("openvpn command exited with status: {}", status);
    }

    Ok(())
}

fn is_running_as_sudo() -> bool {
    // Check the effective user ID (euid); if it's 0, the program is running as root
    match Command::new("id").arg("-u").output() {
        Ok(output) => {
            if let Ok(user_id) = String::from_utf8(output.stdout) {
                user_id.trim() == "0"
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
