use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::{
    ethernet::{EthernetPacket, EtherTypes},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};
use colored::*;

/// this function starts the sniffing process on the default NIC.
pub fn start_sniffer(interface_name: Option<String>) {
    let interface_name_match = match interface_name {
        Some(name) => datalink::interfaces()
            .into_iter()
            .find(|iface| iface.name == name),
        None => datalink::interfaces()
            .into_iter()
            .find(|iface| iface.is_up() && !iface.is_loopback()),
    };

    let interface = match interface_name_match {
        Some(iface) => iface,
        None => {
            println!("{} No active network interface found.", "[-]".red());
            return;
        }
    };

    println!("{} Listening on interface: {}...", "[*]".cyan(), interface.name);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            println!("{} Unsupported channel type", "[-]".red());
            return;
        }
        Err(e) => {
            println!("{} Failed to create channel: {}", "[-]".red(), e);
            return;
        }
    };

    println!("{} Sniffer started. Press CTRL+C to stop.", "[+]".green());
    println!("{:<25} {:<15} {:<10} {:<10} {}", "Source IP", "Dest IP", "Protocol", "Length", "Payload Hint");
    println!("{}", "-".repeat(90));

    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                
                //only care about IPv4 packets for this demo
                if ethernet_packet.get_ethertype() != EtherTypes::Ipv4 {
                    continue;
                }

                let ip_packet = Ipv4Packet::new(ethernet_packet.payload()).unwrap();
                
                let source = ip_packet.get_source();
                let destination = ip_packet.get_destination();
                let protocol = ip_packet.get_next_level_protocol();
                let payload_len = ip_packet.payload().len();

                let protocol_str = match protocol {
                    IpNextHeaderProtocols::Tcp => {
                        let tcp_packet = TcpPacket::new(ip_packet.payload()).unwrap();
                        format!("TCP ({})", tcp_packet.get_destination())
                    }
                    IpNextHeaderProtocols::Udp => {
                        let udp_packet = UdpPacket::new(ip_packet.payload()).unwrap();
                        format!("UDP ({})", udp_packet.get_destination())
                    }
                    _ => "OTHER".to_string(),
                };

                //table format
                println!(
                    "{:<25} {:<15} {:<10} {:<10} {}",
                    source.to_string().bright_cyan(),
                    destination.to_string().bright_white(),
                    protocol_str.bright_yellow(),
                    payload_len,
                    "Data...".dimmed()
                );
            }
            Err(e) => {
                println!("{} Error while reading: {}", "[-]".red(), e);
            }
        }
    }
}