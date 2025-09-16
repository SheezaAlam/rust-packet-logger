mod interface;
mod capture;
mod parser;
mod filter;
mod logger;
#[cfg(test)]
mod tests;

use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust Packet Logger")]
pub struct Cli {
    #[arg(long, help = "List available interfaces")]
    pub list: bool,

    #[arg(long, help = "Interface name to capture on")]
    pub iface: Option<String>,

    #[arg(long, help = "Only capture TCP packets")]
    pub tcp: bool,

    #[arg(long, help = "Only capture UDP packets")]
    pub udp: bool,

    #[arg(long, help = "Only capture packets with this port")]
    pub port: Option<u16>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    if cli.list {
        interface::list_interfaces();
        return Ok(());
    }

    if let Some(iface) = &cli.iface {
        capture::start_capture(iface, &cli)?;
    } else {
        println!("No interface provided. Use --list to see available interfaces.");
    }

    Ok(())
}
