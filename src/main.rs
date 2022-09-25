use msi_klc::*;

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
        .arg_required_else_help(true)
        .subcommand(clap::Command::new("reset").about("Reset all the LEDs on your keyboard"))
        .subcommand(clap::Command::new("off").about("Turn off all the LEDs on your keyboard"))
        .subcommand(
            clap::Command::new("set")
                .about("Set the color/brightness/mode of the LEDs on your keyboard")
                .arg_required_else_help(true)
                .arg(
                    clap::Arg::new("color")
                        .short('c')
                        .long("color")
                        .action(clap::ArgAction::StoreValue)
                        .help("The color of the LEDs on your keyboard")
                        .long_help("Supports 8 predefined colors. 'off' will turn off the LEDs, and setting 'mode' to 'rgb' will allow you to use RGB colors ('#ff0000' = red, '#0000ff' = blue, etc).")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("region")
                        .short('r')
                        .long("region")
                        .help("The region of the LEDs you want to change")
                        .long_help("The 3 regions on your keyboard ('left', 'middle', 'right'). 'all' will apply your color to all 3 regions.")
                        .action(clap::ArgAction::StoreValue)
                )
                .arg(
                    clap::Arg::new("brightness")
                        .short('b')
                        .long("brightness")
                        .help("The brightness of the LEDs on your keyboard")
                        .long_help("Supports 4 brightness/saturation levels ('dark', 'low', 'medium', and 'high').")
                        .action(clap::ArgAction::StoreValue),
                )
                .arg(
                    clap::Arg::new("mode")
                        .short('m')
                        .long("mode")
                        .help("The mode of the LEDs on your keyboard")
                        .long_help("Supports 3 modes ('normal', 'gaming', and 'rgb').")
                        .action(clap::ArgAction::StoreValue),
                ),
        );
    match command.get_matches().subcommand() {
        Some(("reset", _)) => keyboard.reset().unwrap(),
        Some(("off", _)) => keyboard.off().unwrap(),
        Some(("set", matches)) => {
            let mut color = match matches.get_one::<String>("color") {
                Some(color) => color.to_string(),
                None => "off".to_string(),
            };
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
                _ => &Color::Off,
            };

            let region = match matches.get_one::<String>("region") {
                Some(region) => region.to_string(),
                None => "all".to_string(),
            };
            let keyboard_region = match region.to_lowercase().as_str() {
                "left" => &Region::Left,
                "middle" => &Region::Middle,
                "right" => &Region::Right,
                "all" => &Region::All,
                _ => &Region::All,
            };

            let brightness = match matches.get_one::<String>("brightness") {
                Some(brightness) => brightness.to_string(),
                None => "medium".to_string(),
            };
            let keyboard_brightness = match brightness.to_lowercase().as_str() {
                "dark" => &Brightness::Dark,
                "low" => &Brightness::Low,
                "medium" => &Brightness::Medium,
                "high" => &Brightness::High,
                _ => &Brightness::Medium,
            };

            let mode = match matches.get_one::<String>("mode") {
                Some(mode) => mode.to_string(),
                None => "normal".to_string(),
            };
            let keyboard_mode = match mode.to_lowercase().as_str() {
                "normal" => &Mode::Normal,
                "gaming" => &Mode::Gaming,
                "rgb" => &Mode::RGB,
                _ => &Mode::Normal,
            };

            if keyboard_mode == &Mode::RGB {
                let mut rgb_colors: [u8; 3] = [0, 0, 0];
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
                    for (color_index, rgb_color) in color.split(",").enumerate() {
                        if color_index < 3 {
                            rgb_colors[color_index] = rgb_color.parse().unwrap_or(0);
                        }
                    }
                }
                keyboard
                    .set_rgb_color(&KeyboardRGBLightData::new(
                        &keyboard_region,
                        &(rgb_colors[0], rgb_colors[1], rgb_colors[2]),
                    ))
                    .unwrap();
            } else {
                keyboard
                    .set_color(&KeyboardLightData::new(
                        &keyboard_region,
                        &keyboard_color,
                        &keyboard_brightness,
                    ))
                    .unwrap();
                keyboard
                    .set_mode(&KeyboardModeData::new(&keyboard_mode))
                    .unwrap();
            }
        }
        _ => unreachable!(),
    }
}
