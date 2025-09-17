# Rust Packet Logger

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/SheezaAlam/rust-packet-logger/rust.yml?branch=main)](https://github.com/SheezaAlam/rust-packet-logger/actions)
[![Issues](https://img.shields.io/github/issues/SheezaAlam/rust-packet-logger)](https://github.com/SheezaAlam/rust-packet-logger/issues)

---

## Overview

**Rust Packet Logger** is a high-performance **network packet sniffer** built in Rust.
It captures live traffic from any network interface, parses Ethernet, IPv4, TCP, and UDP headers, and outputs structured, human-readable logs.

This project demonstrates **protocol-level analysis**, **system-level Rust programming**, and **network tool design**, bridging the gap between academic networking knowledge and real-world protocol development.

---

## Features

* List available network interfaces (`--list`)
* Capture packets from a selected interface (`--iface <name>`)
* Parse **Ethernet**, **IPv4**, **TCP**, and **UDP** headers
* Colored, human-readable logging with timestamps
* CLI filters:

  * `--tcp` / `--udp`
  * `--port <number>`
* Promiscuous mode support (`--promisc`)
* Robust error handling with `anyhow` and structured logs with `tracing`

---

## Installation

### Prerequisites

* Rust 1.80+
* Linux or macOS (Windows supported via Npcap)
* Admin/root privileges for packet capture

### Build

```bash
git clone https://github.com/SheezaAlam/rust-packet-logger.git
cd rust-packet-logger
cargo build --release
```

---

## Usage

### List Interfaces

```bash
cargo run -- --list
```

Example output:

```
eth0   mac:00:0c:29:ab:cd:ef  ips:[192.168.1.10/24]
lo     mac:none              ips:[127.0.0.1/8]
```

### Capture Packets

```bash
cargo run -- --iface eth0 --promisc
```

Example output:

```
[2025-09-13 22:10:05] TCP 192.168.1.2:50500 -> 93.184.216.34:443
[2025-09-13 22:10:06] UDP 192.168.1.2:58000 -> 8.8.8.8:53
```

### Apply Filters

Capture **TCP only**:

```bash
cargo run -- --iface eth0 --tcp
```

Capture **UDP on port 53**:

```bash
cargo run -- --iface eth0 --udp --port 53
```

---

## Architecture

```
+------------------------+
|  Command-line (clap)   | --list, --iface, --tcp/udp, --port, --promisc
+------------------------+
             |
             v
+------------------------+
|   Packet Capture       | (pnet::datalink::channel)
|   capture.rs           |
+------------------------+
             |
             v
+------------------------+
|   Packet Parser        | parser.rs
|  - Ethernet header     |
|  - IPv4 header         |
|  - TCP/UDP headers     |
+------------------------+
             |
             v
+------------------------+
|   Logger & Output      | utils.rs
|  - chrono timestamps   |
|  - colored output      |
|  - tracing logs        |
+------------------------+
```

---

## Learning Outcomes

This project demonstrates:

* **Raw packet capture** with `pnet`
* **Protocol parsing** at datalink, network, and transport layers
* **CLI tool design** with `clap`
* **Structured logging** using `tracing`, `chrono`, and `colored`
* Real-world networking tool development suitable for **protocol analysis, monitoring, and security applications**

---

## Project Structure

```
src/
 ├─ main.rs        # Entry point
 ├─ cli.rs         # Command-line parsing
 ├─ capture.rs     # Packet capture logic
 ├─ parser.rs      # Header parsing and formatting
 ├─ logger.rs      # Logging & output helpers
 └─ tests.rs       # Unit tests for packet parsing
```

---

## Recommended Next Steps

To elevate this project for **Level 3+ protocol developer showcase**:

1. Implement **IPv6, ICMP, and TCP option parsing**
2. Add **unit and integration tests** for packet parsing correctness
3. Include **performance benchmarks** for high-throughput scenarios
4. Add **multi-interface capture** and **real-time filtering**

---

## License

MIT License – see [LICENSE](./LICENSE) for details.

