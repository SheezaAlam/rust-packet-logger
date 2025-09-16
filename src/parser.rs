use pnet::packet::{Packet};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use chrono::Utc;
use crate::{Cli, filter, logger};
//use colored::Colorize;




pub fn handle_frame(frame: &[u8], cli: &Cli) {
if let Some(eth) = EthernetPacket::new(frame) {
match eth.get_ethertype() {
EtherTypes::Ipv4 => {
if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
let src = ipv4.get_source();
let dst = ipv4.get_destination();
let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");


match ipv4.get_next_level_protocol() {
IpNextHeaderProtocols::Tcp => {
if cli.udp { return; }
if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
if !filter::port_match(cli, tcp.get_source(), tcp.get_destination()) {
return;
}
logger::log_tcp(&timestamp.to_string(), src, dst, tcp);
}
}
IpNextHeaderProtocols::Udp => {
if cli.tcp { return; }
if let Some(udp) = UdpPacket::new(ipv4.payload()) {
if !filter::port_match(cli, udp.get_source(), udp.get_destination()) {
return;
}
logger::log_udp(&timestamp.to_string(), src, dst, udp);
}
}
_ => {}
}
}
}
EtherTypes::Ipv6 => {}
_ => {}
}
}
}