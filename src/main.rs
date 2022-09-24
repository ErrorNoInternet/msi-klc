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
            let keyboard_brightness = match brightness.to_lowercase().as_str() {
                "dark" => &Brightness::Dark,
                "low" => &Brightness::Low,
                "medium" => &Brightness::Medium,
                "high" => &Brightness::High,
                "rgb" => &Brightness::RGB,
                _ => &Brightness::Medium,
            };
            for (index, color) in colors.split(",").enumerate() {
                if keyboard_brightness != &Brightness::RGB {
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
                } else {
                    let mut rgb_colors: [u8; 3] = [0, 0, 0];
                    for (color_index, rgb_color) in color.split(";").enumerate() {
                        if color_index < 3 {
                            rgb_colors[color_index] = rgb_color.parse().unwrap_or(0);
                        }
                    }

                    if index == 0 {
                        keyboard
                            .set_rgb_color(&KeyboardRGBLightData::new(
                                &Region::Left,
                                &(rgb_colors[0], rgb_colors[1], rgb_colors[2]),
                            ))
                            .unwrap();
                    } else if index == 1 {
                        keyboard
                            .set_rgb_color(&KeyboardRGBLightData::new(
                                &Region::Middle,
                                &(rgb_colors[0], rgb_colors[1], rgb_colors[2]),
                            ))
                            .unwrap();
                    } else if index == 2 {
                        keyboard
                            .set_rgb_color(&KeyboardRGBLightData::new(
                                &Region::Right,
                                &(rgb_colors[0], rgb_colors[1], rgb_colors[2]),
                            ))
                            .unwrap();
                    };
                }
            }

            if keyboard_brightness != &Brightness::RGB {
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
        }
        _ => unreachable!(),
    }
}
