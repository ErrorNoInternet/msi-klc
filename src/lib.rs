#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Region {
    Left = 1,
    Middle = 2,
    Right = 3,
    All = 255,
}

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

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Brightness {
    Dark = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}

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

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Normal = 1,
    Gaming = 2,
    RGB = 255,
}

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

pub struct Keyboard {
    keyboard: hidapi::HidDevice,
}

impl Keyboard {
    pub fn new() -> Result<Self, hidapi::HidError> {
        let api = hidapi::HidApi::new()?;
        let keyboard = api.open(0x1770, 0xff00)?;
        Ok(Keyboard { keyboard })
    }

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

    pub fn set_mode(
        &mut self,
        keyboard_mode_data: &KeyboardModeData,
    ) -> Result<(), hidapi::HidError> {
        let mode_data: [u8; 8] = keyboard_mode_data.to_owned().into();
        self.keyboard.send_feature_report(&mode_data)?;

        Ok(())
    }
}
