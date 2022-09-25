# msi-klc
[![crates.io](https://img.shields.io/crates/v/msi-klc.svg)](https://crates.io/crates/msi-klc)
[![docs.rs](https://img.shields.io/docsrs/msi-klc/latest)](https://docs.rs/msi-klc/latest/msi_klc)

A tool/library that allows you to control the backlight of your MSI SteelSeries laptop keyboard.\
Supports 3 regions, 8 predefined colors, RGB/hex colors, and custom animations.

### Tested Devices
#### Linux
- [x] MSI GE60 2PE

## Installation
### Compiling
- Requirements
    - Rust (cargo)
    - hidapi
```sh
git clone https://github.com/ErrorNoInternet/msi-klc
cd msi-klc
cargo build --release
sudo cp target/release/msi-klc /usr/local/bin
```

## Usage
Make sure to run with root privileges (sudo)
### CLI
```sh
# make the entire keyboard blue
msi-klc set --color blue

# make the left side of the keyboard red
msi-klc set --color red --region left

# turn off all the LEDs on the keyboard
msi-klc off

# ...and turn them back on
msi-klc reset

# make the keyboard cyan
msi-klc set --color "#0fffaf" --mode rgb

# make only the left side cyan
msi-klc off && msi-klc set --color "#0fffaf" --mode rgb --region left

# load an animation
msi-klc load animations/breathe.txt
```
### Library
```rust
use msi_klc::*;

fn main() {
    let mut keyboard = Keyboard::new().unwrap();
    
    // make the keyboard blue
    keyboard
        .set_color(&KeyboardLightData::new(
            &Region::All,
            &Color::Blue,
            &Brightness::Medium,
        )).unwrap();
    keyboard
        .set_mode(&KeyboardModeData::new(&Mode::Normal))
        .unwrap();

    // set a custom RGB color on the right side of the keyboard
    keyboard
        .set_rgb_color(&KeyboardRGBLightData::new(
            &Region::Right,
            &(255, 80, 80),
        )).unwrap();
}
```

<sub>If you would like to modify or use this repository (including its code) in your own project, please be sure to credit!</sub>
