#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::TcpStream;
use std::time::Duration;

fn main() {
    // 1. Test if the Tor local proxy is active before opening the window
    let proxy_address = "127.0.0.1:9050";
    let is_tor_running = TcpStream::connect_timeout(
        &proxy_address.parse().unwrap(), 
        Duration::from_secs(2)
    ).is_ok();

    // 2. Setup the application builder
    tauri::Builder::default()
        .setup(move |app| {
            if !is_tor_running {
                println!("CRITICAL ERROR: Tor daemon not detected on port 9050!");
                // You can add logic here to alert the user or kill the process safely
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
