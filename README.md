<div align="center">

# voidsniff 

<img src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&size=22&duration=2000&pause=500&color=BD93F9&center=true&vCenter=true&width=500&lines=Custom+Packet+Sniffer+v1.0;Network+Forensics+Tool;Powered+by+Rust+2025" alt="Typing SVG" />

<p align="center">
  <img src="https://img.shields.io/badge/Version-0.1.0-BD93F9?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Rust-2025-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/Security-Forensics-red?style=for-the-badge" />
</p>

---

### 📡 Overview

**VoidSniff** is a specialized network forensics tool developed in **Rust**. It performs low-level packet capture and real-time analysis of network traffic. By utilizing the `pnet` framework, VoidSniff provides a granular view of IPv4 packets, allowing for the monitoring of TCP and UDP transmissions across specific network interfaces.

</div>

---

### 🚀 Core Features

* **Real-Time Packet Sniffing**: Captures and decodes live Ethernet frames directly from the data link layer.
* **Protocol Analysis**: Automatically identifies and categorizes **IPv4** traffic, including detailed port information for **TCP** and **UDP**.
* **Interface Flexibility**: Allows users to manually select a network interface (e.g., `wlan0`, `eth0`) or auto-detect an active, non-loopback interface.
* **Formatted Terminal Output**: Uses color-coded tables to display Source/Destination IPs, Protocols, and Payload lengths for easy auditing.

---

### 🛠️ Tech Stack & Logic

* **Language**: Rust (Edition 2025)
* **Networking Stack**: `pnet` (Cross-platform data link access)
* **Packet Handling**:
    * **Ethernet Layer**: Decodes Ethernet packets and filters for `EtherTypes::Ipv4`.
    * **IP Layer**: Extracts source and destination addresses from `Ipv4Packet`.
    * **Transport Layer**: Identifies specific protocols and destination ports.

---

### ⚙️ Installation & Usage

#### 1. Prerequisites
Ensure you have the Rust toolchain installed. Since this accesses raw network channels, you may need `root` or `admin` privileges.

#### 2. Build the Project
cargo build --release

3. Execution
sudo ./target/release/voidsniff

4. Usage Flow
 * Launch the tool.
 * Input your preferred interface (e.g., tun0 for VPN or wlan0 for Wi-Fi).
 * Press Enter to start the sniffer.
 * View live traffic logs in the terminal.
 * Press CTRL+C to stop.
📊 Capture Table
| Field | Description |
|---|---|
| Source IP | The origin address of the packet. |
| Dest IP | The target recipient address. |
| Protocol | Identified protocol (TCP/UDP/OTHER) and target port. |
| Length | Total size of the payload in bytes. |

---

### ⚖️ License
This project is dual-licensed under:
 * MIT License (LICENSE-MIT)
 * Apache License, Version 2.0 (LICENSE-APACHE)
### ⚠️ Disclaimer
> [!CAUTION]
> VoidSniff is intended for authorized network auditing and forensics only. Sniffing traffic on networks where you do not have explicit permission is illegal. Use with caution.
> 

<div align="center">
Developed by [norct](https://github.com/Unjou) 👾
</div>

