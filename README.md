# **PORT SNIFFER** 🕵️‍♂️

A fast, multithreaded **TCP port scanner** written in Rust.  
It scans an IP address for open ports and shows the associated service names when possible.

## Features
- 🚀 Multithreaded scanning for speed.
- 🔍 Detects open TCP ports.
- 🏷 Displays common service names (HTTP, SSH, etc.).
- 🌐 Supports both IPv4 and IPv6.
- ⚙️ Customizable thread count.

## Usage
```bash
cargo run -- <ip>                 # Scan with default 4 threads
cargo run -- -j <threads> <ip>    # Scan with custom number of threads
cargo run -- -h                   # Show help
