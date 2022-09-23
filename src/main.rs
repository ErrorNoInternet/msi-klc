#[repr(u8)]
#[derive(Copy, Clone)]
enum Region {
    Left = 1,
    Middle = 2,
    Right = 3,
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum Color {
    Off = 0,
    Red = 1,
    Orange = 2,
    Yellow = 3,
    Green = 4,
    Sky = 5,
    Blue = 6,
    Purple = 7,
    White = 8,
}

#[repr(u8)]
#[derive(Copy, Clone)]
enum Brightness {
    Dark = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(Copy, Clone)]
struct KeyboardLightData {
    region: Region,
    color: Color,
    brightness: Brightness,
}

impl KeyboardLightData {
    fn new(region: &Region, color: &Color, brightness: &Brightness) -> Self {
        KeyboardLightData {
            region: region.clone(),
            color: color.clone(),
            brightness: brightness.clone(),
        }
    }
}

impl Into<[u8; 8]> for KeyboardLightData {
    fn into(self) -> [u8; 8] {
        [
            1,
            2,
            66,
            self.region as u8,
            self.color as u8,
            self.brightness as u8,
            0,
            236,
        ]
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum Mode {
    Normal = 1,
    Gaming = 2,
}

#[derive(Copy, Clone)]
struct KeyboardModeData {
    mode: Mode,
}

impl KeyboardModeData {
    fn new(mode: &Mode) -> Self {
        KeyboardModeData { mode: mode.clone() }
    }
}

impl Into<[u8; 8]> for KeyboardModeData {
    fn into(self) -> [u8; 8] {
        [1, 2, 65, self.mode as u8, 0, 0, 0, 236]
    }
}

struct Keyboard {
    keyboard: hidapi::HidDevice,
    current_light_data: KeyboardLightData,
    current_mode_data: KeyboardModeData,
}

impl Keyboard {
    fn new() -> Result<Self, hidapi::HidError> {
        let api = hidapi::HidApi::new()?;
        let keyboard = api.open(0x1770, 0xff00)?;
        Ok(Keyboard {
            keyboard,
            current_light_data: KeyboardLightData::new(
                &Region::Left,
                &Color::White,
                &Brightness::Medium,
            ),
            current_mode_data: KeyboardModeData::new(&Mode::Normal),
        })
    }

    fn reset(&mut self) -> Result<(), hidapi::HidError> {
        self.set_color(&KeyboardLightData::new(
            &Region::Left,
            &Color::White,
            &Brightness::Medium,
        ))?;
        self.set_color(&KeyboardLightData::new(
            &Region::Middle,
            &Color::White,
            &Brightness::Medium,
        ))?;
        self.set_color(&KeyboardLightData::new(
            &Region::Right,
            &Color::White,
            &Brightness::Medium,
        ))?;
        self.set_mode(&KeyboardModeData::new(&Mode::Normal))
    }

    fn set_color(
        &mut self,
        keyboard_light_data: &KeyboardLightData,
    ) -> Result<(), hidapi::HidError> {
        let light_data: [u8; 8] = keyboard_light_data.to_owned().into();
        self.keyboard.send_feature_report(&light_data)?;
        self.current_light_data = keyboard_light_data.clone();

        Ok(())
    }

    fn set_mode(&mut self, keyboard_mode_data: &KeyboardModeData) -> Result<(), hidapi::HidError> {
        let mode_data: [u8; 8] = keyboard_mode_data.to_owned().into();
        self.keyboard.send_feature_report(&mode_data)?;
        self.current_mode_data = keyboard_mode_data.clone();

        Ok(())
    }
}

fn main() {
    let mut keyboard = match Keyboard::new() {
        Ok(keyboard) => keyboard,
        Err(_) => {
            println!("Unable to open keyboard. Are you sure you're running as root?");
            std::process::exit(1);
        }
    };

    let command = clap::Command::new("msi-klc")
        .subcommand_required(true)
        .subcommand(clap::Command::new("reset").about("Reset all the LEDs on the keyboard"))
        .subcommand(clap::Command::new("off").about("Turn off all the LEDs on the keyboard"))
        .subcommand(
            clap::Command::new("set")
                .about("Set the color/brightness/mode of the LEDs on the keyboard")
                .arg_required_else_help(true)
                .arg(clap::Arg::new("color"))
                .arg(clap::Arg::new("brightness"))
                .arg(clap::Arg::new("mode")),
        );
    match command.get_matches().subcommand() {
        Some(("reset", _)) => keyboard.reset().unwrap(),
        Some(("off", _)) => {
            keyboard
                .set_color(&KeyboardLightData::new(
                    &Region::Left,
                    &Color::Off,
                    &Brightness::Medium,
                ))
                .unwrap();
            keyboard
                .set_color(&KeyboardLightData::new(
                    &Region::Middle,
                    &Color::Off,
                    &Brightness::Medium,
                ))
                .unwrap();
            keyboard
                .set_color(&KeyboardLightData::new(
                    &Region::Right,
                    &Color::Off,
                    &Brightness::Medium,
                ))
                .unwrap();
        }
        Some(("set", matches)) => {
            let colors = match matches.get_one::<String>("color") {
                Some(colors) => colors.to_string(),
                None => "white".to_string(),
            };
            let brightness = match matches.get_one::<String>("brightness") {
                Some(brightness) => brightness.to_string(),
                None => "medium".to_string(),
            };
            for (index, color) in colors.split(",").enumerate() {
                let keyboard_color = match color.to_lowercase().to_string().as_str() {
                    "off" => &Color::Off,
                    "red" => &Color::Red,
                    "orange" => &Color::Orange,
                    "yellow" => &Color::Yellow,
                    "green" => &Color::Green,
                    "sky" => &Color::Sky,
                    "blue" => &Color::Blue,
                    "purple" => &Color::Purple,
                    "white" => &Color::White,
                    _ => &Color::White,
                };
                let keyboard_brightness = match brightness.to_lowercase().as_str() {
                    "dark" => &Brightness::Dark,
                    "low" => &Brightness::Low,
                    "medium" => &Brightness::Medium,
                    "high" => &Brightness::High,
                    _ => &Brightness::Medium,
                };

                if index == 0 {
                    keyboard
                        .set_color(&KeyboardLightData::new(
                            &Region::Left,
                            &keyboard_color,
                            &keyboard_brightness,
                        ))
                        .unwrap();
                } else if index == 1 {
                    keyboard
                        .set_color(&KeyboardLightData::new(
                            &Region::Middle,
                            &keyboard_color,
                            &keyboard_brightness,
                        ))
                        .unwrap();
                } else if index == 2 {
                    keyboard
                        .set_color(&KeyboardLightData::new(
                            &Region::Right,
                            &keyboard_color,
                            &keyboard_brightness,
                        ))
                        .unwrap();
                };
            }

            let mode = match matches.get_one::<String>("mode") {
                Some(mode) => mode.to_string(),
                None => "normal".to_string(),
            };
            let keyboard_mode = match mode.to_lowercase().as_str() {
                "normal" => &Mode::Normal,
                "gaming" => &Mode::Gaming,
                _ => &Mode::Normal,
            };
            keyboard
                .set_mode(&KeyboardModeData::new(&keyboard_mode))
                .unwrap();
        }
        _ => unreachable!(),
    }
}
