use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use std::net::Ipv4Addr;
use colored::Colorize;


pub fn log_tcp(timestamp: &str, src: Ipv4Addr, dst: Ipv4Addr, tcp: TcpPacket) {
println!(
"[{}] {} {}:{} -> {}:{}",
timestamp,
"TCP".blue(),
src,
tcp.get_source(),
dst,
tcp.get_destination()
);
}


pub fn log_udp(timestamp: &str, src: Ipv4Addr, dst: Ipv4Addr, udp: UdpPacket) {
println!(
"[{}] {} {}:{} -> {}:{}",
timestamp,
"UDP".yellow(),
src,
udp.get_source(),
dst,
udp.get_destination()
);
}