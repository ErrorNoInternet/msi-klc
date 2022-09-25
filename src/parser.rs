use msi_klc::*;

pub fn parse_color(color: &String) -> Color {
    match color.to_lowercase().to_string().as_str() {
        "off" => Color::Off,
        "red" => Color::Red,
        "orange" => Color::Orange,
        "yellow" => Color::Yellow,
        "green" => Color::Green,
        "sky" => Color::Sky,
        "blue" => Color::Blue,
        "purple" => Color::Purple,
        "white" => Color::White,
        _ => Color::Off,
    }
}

pub fn parse_region(region: &String) -> Region {
    match region.to_lowercase().as_str() {
        "left" => Region::Left,
        "middle" => Region::Middle,
        "right" => Region::Right,
        "all" => Region::All,
        _ => Region::All,
    }
}

pub fn parse_brightness(brightness: &String) -> Brightness {
    match brightness.to_lowercase().as_str() {
        "dark" => Brightness::Dark,
        "low" => Brightness::Low,
        "medium" => Brightness::Medium,
        "high" => Brightness::High,
        _ => Brightness::Medium,
    }
}

pub fn parse_mode(mode: &String) -> Mode {
    match mode.to_lowercase().as_str() {
        "normal" => Mode::Normal,
        "gaming" => Mode::Gaming,
        "rgb" => Mode::RGB,
        _ => Mode::Normal,
    }
}
