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
    High = 0,
    Medium = 1,
    Low = 2,
    Faint = 3,
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
                &Brightness::Low,
            ),
            current_mode_data: KeyboardModeData::new(&Mode::Normal),
        })
    }

    fn refresh(&mut self) -> Result<(), hidapi::HidError> {
        self.set_mode(&KeyboardModeData::new(&self.current_mode_data.mode))?;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), hidapi::HidError> {
        self.set_color(&KeyboardLightData::new(
            &Region::Left,
            &Color::White,
            &Brightness::Low,
        ))?;
        self.set_color(&KeyboardLightData::new(
            &Region::Middle,
            &Color::White,
            &Brightness::Low,
        ))?;
        self.set_color(&KeyboardLightData::new(
            &Region::Right,
            &Color::White,
            &Brightness::Low,
        ))?;
        self.refresh()
    }

    fn set_color(
        &mut self,
        keyboard_light_data: &KeyboardLightData,
    ) -> Result<(), hidapi::HidError> {
        let light_data: [u8; 8] = keyboard_light_data.to_owned().into();
        self.keyboard.send_feature_report(&light_data)?;
        self.current_light_data = keyboard_light_data.clone();
        self.set_mode(&KeyboardModeData::new(&Mode::Normal))?;

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

    keyboard.reset().unwrap();
    keyboard.refresh().unwrap();
    keyboard
        .set_color(&KeyboardLightData::new(
            &Region::Left,
            &Color::Red,
            &Brightness::Low,
        ))
        .unwrap();
    keyboard.refresh().unwrap();
    keyboard
        .set_color(&KeyboardLightData::new(
            &Region::Middle,
            &Color::Green,
            &Brightness::Low,
        ))
        .unwrap();
    keyboard.refresh().unwrap();
    keyboard
        .set_color(&KeyboardLightData::new(
            &Region::Right,
            &Color::Blue,
            &Brightness::Low,
        ))
        .unwrap();
    keyboard.refresh().unwrap();
}
