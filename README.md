# msi-klc
A tool that allows you to control the backlight of your MSI SteelSeries laptop keyboard. Tested on the MSI GE60 2PE laptop running Arch Linux.
Supports 3 regions, 8 predefined colors, and RGB/hex colors. Wave/breathe animations can be created yourself using shell scripts.

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

## Usage
Make sure to run with root privileges (sudo)
```sh
# make the entire keyboard blue
msi-klc set --color blue

# make the left side of the keyboard red
msi-klc set --color red --region left

# turn off all the LEDs on the keyboard
msi-klc off

# ...and then them back on
msi-klc reset

# make the keyboard cyan
msi-klc set --color "#0fffaf" --mode rgb

# make only the left side cyan
msi-klc off && msi-klc set --color "#0fffaf" --mode rgb --region left
```

