/// The 3 keyboard regions. `All` represents all regions (each region will be set automatically).
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Region {
    Left = 1,
    Middle = 2,
    Right = 3,
    All = 255,
}

/// 8 predefined colors (and `Off`).
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
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

/// Saturation/brightness of the lights on the keyboard. Not needed when using RGB colors.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Brightness {
    Dark = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

/// Regular keyboard light data. Contains the region, predefined color, and brightness.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeyboardLightData {
    pub region: Region,
    pub color: Color,
    pub brightness: Brightness,
}

impl KeyboardLightData {
    pub fn new(region: &Region, color: &Color, brightness: &Brightness) -> Self {
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

/// RGB keyboard light data. Contains the region and RGB values.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeyboardRGBLightData {
    pub region: Region,
    pub color: (u8, u8, u8),
}

impl KeyboardRGBLightData {
    pub fn new(region: &Region, color: &(u8, u8, u8)) -> Self {
        KeyboardRGBLightData {
            region: region.clone(),
            color: color.clone(),
        }
    }
}

impl Into<[u8; 8]> for KeyboardRGBLightData {
    fn into(self) -> [u8; 8] {
        [
            1,
            2,
            64,
            self.region as u8,
            self.color.0,
            self.color.1,
            self.color.2,
            236,
        ]
    }
}

/// Keyboard mode (`Normal` or `Gaming`). `Wave` and `Breathe` are not implemented since you can do
/// the same thing with `set_rgb_color` and a loop.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    /// All lights on the keyboard.
    Normal = 1,
    /// Only the left side of the keyboard.
    Gaming = 2,
    /// Indicates that the keyboard mode will be RGB. Doesn't do anything.
    RGB = 255,
}

/// Keyboard mode data (only contains mode).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeyboardModeData {
    pub mode: Mode,
}

impl KeyboardModeData {
    pub fn new(mode: &Mode) -> Self {
        KeyboardModeData { mode: mode.clone() }
    }
}

impl Into<[u8; 8]> for KeyboardModeData {
    fn into(self) -> [u8; 8] {
        [1, 2, 65, self.mode as u8, 0, 0, 0, 236]
    }
}

/// A keyboard object (wrapper around a HidDevice).
pub struct Keyboard {
    keyboard: hidapi::HidDevice,
}

impl Keyboard {
    /// Creates a new keyboard object.
    pub fn new() -> Result<Self, hidapi::HidError> {
        let api = hidapi::HidApi::new()?;
        let keyboard = api.open(0x1770, 0xff00)?;
        Ok(Keyboard { keyboard })
    }

    /// Makes all the LEDs on the keyboard white.
    pub fn reset(&mut self) -> Result<(), hidapi::HidError> {
        self.set_color(&KeyboardLightData::new(
            &Region::Left,
            &Color::White,
            &Brightness::Dark,
        ))?;
        self.set_color(&KeyboardLightData::new(
            &Region::Middle,
            &Color::White,
            &Brightness::Dark,
        ))?;
        self.set_color(&KeyboardLightData::new(
            &Region::Right,
            &Color::White,
            &Brightness::Dark,
        ))?;
        self.set_mode(&KeyboardModeData::new(&Mode::Normal))
    }

    /// Turns off all the LEDs on the keyboard.
    pub fn off(&mut self) -> Result<(), hidapi::HidError> {
        self.set_color(&KeyboardLightData::new(
            &Region::Left,
            &Color::Off,
            &Brightness::Medium,
        ))?;
        self.set_color(&KeyboardLightData::new(
            &Region::Middle,
            &Color::Off,
            &Brightness::Medium,
        ))?;
        self.set_color(&KeyboardLightData::new(
            &Region::Right,
            &Color::Off,
            &Brightness::Medium,
        ))?;
        self.set_mode(&KeyboardModeData::new(&Mode::Normal))
    }

    /// Makes the keyboard display a predefined color.
    pub fn set_color(
        &mut self,
        keyboard_light_data: &KeyboardLightData,
    ) -> Result<(), hidapi::HidError> {
        if keyboard_light_data.region == Region::All {
            let mut modified_light_data = keyboard_light_data.clone();

            modified_light_data.region = Region::Left;
            let light_data: [u8; 8] = modified_light_data.to_owned().into();
            self.keyboard.send_feature_report(&light_data)?;

            modified_light_data.region = Region::Middle;
            let light_data: [u8; 8] = modified_light_data.to_owned().into();
            self.keyboard.send_feature_report(&light_data)?;

            modified_light_data.region = Region::Right;
            let light_data: [u8; 8] = modified_light_data.to_owned().into();
            self.keyboard.send_feature_report(&light_data)?;
        } else {
            let light_data: [u8; 8] = keyboard_light_data.to_owned().into();
            self.keyboard.send_feature_report(&light_data)?;
        }

        Ok(())
    }

    /// Makes the keyboard display a custom RGB color.
    pub fn set_rgb_color(
        &mut self,
        keyboard_light_data: &KeyboardRGBLightData,
    ) -> Result<(), hidapi::HidError> {
        if keyboard_light_data.region == Region::All {
            let mut modified_light_data = keyboard_light_data.clone();

            modified_light_data.region = Region::Left;
            let light_data: [u8; 8] = modified_light_data.to_owned().into();
            self.keyboard.send_feature_report(&light_data)?;

            modified_light_data.region = Region::Middle;
            let light_data: [u8; 8] = modified_light_data.to_owned().into();
            self.keyboard.send_feature_report(&light_data)?;

            modified_light_data.region = Region::Right;
            let light_data: [u8; 8] = modified_light_data.to_owned().into();
            self.keyboard.send_feature_report(&light_data)?;
        } else {
            let light_data: [u8; 8] = keyboard_light_data.to_owned().into();
            self.keyboard.send_feature_report(&light_data)?;
        }

        Ok(())
    }

    /// Changes the mode of the keyboard.
    pub fn set_mode(
        &mut self,
        keyboard_mode_data: &KeyboardModeData,
    ) -> Result<(), hidapi::HidError> {
        let mut keyboard_mode_data = keyboard_mode_data.clone();
        if keyboard_mode_data.mode == Mode::RGB {
            keyboard_mode_data.mode = Mode::Normal;
        }
        let mode_data: [u8; 8] = keyboard_mode_data.to_owned().into();
        self.keyboard.send_feature_report(&mode_data)
    }
}
