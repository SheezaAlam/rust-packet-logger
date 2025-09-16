# rust-packet-logger

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/SheezaAlam/rust-packet-logger/rust.yml?branch=main)](https://github.com/SheezaAlam/rust-packet-logger/actions)
[![Issues](https://img.shields.io/github/issues/SheezaAlam/rust-packet-logger)](https://github.com/SheezaAlam/rust-packet-logger/issues)

A lightweight **network packet sniffer** written in Rust.
It captures live traffic from a chosen network interface and decodes Ethernet, IPv4, TCP, and UDP headers.
Inspired by tools like `tcpdump` and `Wireshark`, this project demonstrates how to build real protocol analysis tools in Rust.

---

## Features

* List available network interfaces (`--list`)
* Capture packets from a chosen interface (`--iface <name>`)
* Support for **Ethernet**, **IPv4**, **TCP**, and **UDP**
* Human-readable logs with **timestamps** and **colored output**
* CLI filters:

  * `--tcp` or `--udp`
  * `--port <number>`
* Promiscuous mode support (`--promisc`)
* Error handling with `anyhow` and structured logs with `tracing`

---

## Installation

### Prerequisites

* Rust (1.80 or later)
* Linux or macOS (Windows supported with Npcap)
* Admin/root privileges to capture packets

### Clone and build

```bash
git clone https://github.com/SheezaAlam/rust-packet-logger.git
cd rust-packet-logger
cargo build --release
```

---

## Usage

### List interfaces

```bash
cargo run -- --list
```

Example output:

```
eth0   mac:00:0c:29:ab:cd:ef  ips:[192.168.1.10/24]
lo     mac:none              ips:[127.0.0.1/8]
```

### Capture all packets

```bash
cargo run -- --iface eth0 --promisc
```

Output:

```
[2025-09-13 22:10:05] TCP 192.168.1.2:50500 -> 93.184.216.34:443
[2025-09-13 22:10:06] UDP 192.168.1.2:58000 -> 8.8.8.8:53
```

### Apply filters

Only capture TCP:

```bash
cargo run -- --iface eth0 --tcp
```

Only UDP on port 53:

```bash
cargo run -- --iface eth0 --udp --port 53
```

---

## Project Structure

```
src/
 ├─ main.rs        # Entry point
 ├─ cli.rs         # Command-line parsing
 ├─ capture.rs     # Packet capture logic
 ├─ parser.rs      # Ethernet/IP/TCP/UDP decoders
 └─ utils.rs       # Logging, formatting helpers
```

---

## Learning Outcomes

This project demonstrates:

* How to use **pnet** for raw packet capture
* Building **async-ready CLI tools** with `clap`
* Using `chrono`, `colored`, and `tracing` for professional-grade logging
* Real-world protocol parsing at the datalink, network, and transport layers

This aligns with **Level 3 projects** in Rust hiring pipelines:
structured codebase, extensible design, and domain-specific problem solving.

---

## Roadmap

* [ ] Add IPv6 support
* [ ] Write to pcap file format for Wireshark import
* [ ] Add ICMP parser
* [ ] Optional: async runtime with Tokio

---

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.

