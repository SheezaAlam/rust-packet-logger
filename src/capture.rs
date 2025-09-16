use pnet::datalink::{self, Channel};
use anyhow::Result;
use crate::{parser, Cli};


pub fn start_capture(iface_name: &str, cli: &Cli) -> Result<()> {
let interfaces = datalink::interfaces();
let interface = interfaces
.into_iter()
.find(|i| i.name == iface_name)
.ok_or_else(|| anyhow::anyhow!("Interface {} not found", iface_name))?;


let (_, mut rx) = match datalink::channel(&interface, Default::default())? {
Channel::Ethernet(tx, rx) => (tx, rx),
_ => return Err(anyhow::anyhow!("Unhandled channel type")),
};


while let Ok(packet) = rx.next() {
parser::handle_frame(packet, cli);
}


Ok(())
}