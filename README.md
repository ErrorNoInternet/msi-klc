# msi-klc
A tool written in Rust that allows you to control your MSI laptop keyboard light. Tested on the MSI GE60 2PE laptop running Arch Linux.

## Installation
### Compiling
- Requirements
	- Rust (cargo)
	- hidapi
```sh
git clone https://github.com/errorNoInternet/msi-klc
cd msi-klc
cargo build --release
sudo cp target/release/msi-klc /usr/local/bin
```

