use crate::Cli;




pub fn port_match(cli: &Cli, src_port: u16, dst_port: u16) -> bool {
if let Some(port) = cli.port {
if src_port != port && dst_port != port {
return false;
}
}
true
}
