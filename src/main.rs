mod network;

use colored::*;
use std::io;

fn main() {
    println!("
╔════════════════════════════════════════╗
║    CUSTOM PACKET SNIFFER v1.0        ║
║       Network Forensics Tool           ║
╚════════════════════════════════════════╝
    ".magenta());

    println!("{} Select Interface (Leave empty for default):", "[?]".yellow());
    println!("Note: On Android (Termux), usually use 'wlan0' or 'tun0'.");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let interface = if input.is_empty() {
        None
    } else {
        Some(input.to_string())
    };

    // start sniff from network module
    network::start_sniffer(interface);
}
