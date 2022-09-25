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

pub fn parse_rgb_colors(color: &String) -> [u8; 3] {
    let mut rgb_colors: [u8; 3] = [0, 0, 0];
    let mut color = color.clone();
    if color.starts_with("0x") && color.len() == 8 {
        color = color.replace("0x", "#");
    }
    if color.starts_with("#") && color.len() == 7 {
        for i in 0..3 {
            let hex_code = color.chars().nth(i * 2 + 1).unwrap().to_string()
                + color.chars().nth(i * 2 + 2).unwrap().to_string().as_str();
            rgb_colors[i] = u8::from_str_radix(hex_code.as_str(), 16).unwrap_or(0);
        }
    } else {
        for (color_index, rgb_color) in color.split(";").enumerate() {
            if color_index < 3 {
                rgb_colors[color_index] = rgb_color.parse().unwrap_or(0);
            }
        }
    }
    rgb_colors
}
