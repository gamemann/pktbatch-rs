# WIP & Uncomplete ([Use For Now](https://github.com/Packet-Batch/pktbatch-c))

[Packet Batch](https://github.com/Packet-Batch) is a collection of high-performance tools used for generating network packets. These tools are commonly used for penetration testing, benchmarking, and network monitoring.

This repository serves as the Rust implementation of Packet Batch. While Packet Batch was originally written in C ([`pktbatch-c`](https://github.com/Packet-Batch/pktbatch-c)), this Rust version aims to provide a safer and more modern codebase while maintaining high performance.

That said, this will now be the main repository for Packet Batch. While `pktbatch-c` will still be maintained, all new features and improvements will be developed in this Rust implementation. The C version will only receive critical bug fixes.

## Building
To build the project, you need to have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/). Once you have Rust installed, you can build the project using Cargo, Rust's package manager and build system.

```bash
# Install Rust using rustup (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project in release mode
cargo build --release
```

This will compile the project and produce an executable in the `target/release` directory. You can run the executable from there or move it to a location in your system's PATH for easier access.

## Usage
After building the project, you can run the executable to generate network packets. The exact usage will depend on the command-line arguments you provide along with the main program's configuration file.

Here is a list of the main command-line arguments you can use.

| Argument | Description |
| --- | --- |
| `-c, --config <FILE>` | Path to the configuration file (required) |
| `-h, --help` | Print help information |
| `-V, --version` | Print version information |