use msi_klc::*;
mod parser;

fn main() {
    let mut keyboard = match Keyboard::new() {
        Ok(keyboard) => keyboard,
        Err(_) => {
            println!("Unable to communicate with keyboard. Are you sure you're running as root?");
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
        )
    .subcommand(clap::Command::new("load").about("Load a msi-klc animation from a file").arg_required_else_help(true).arg(clap::Arg::new("file")));
    match command.get_matches().subcommand() {
        Some(("reset", _)) => keyboard.reset().unwrap(),
        Some(("off", _)) => keyboard.off().unwrap(),
        Some(("set", matches)) => {
            let color = match matches.get_one::<String>("color") {
                Some(color) => color.to_string(),
                None => "".to_string(),
            };
            let keyboard_color = parser::parse_color(&color);

            let region = match matches.get_one::<String>("region") {
                Some(region) => region.to_string(),
                None => "".to_string(),
            };
            let keyboard_region = parser::parse_region(&region);

            let brightness = match matches.get_one::<String>("brightness") {
                Some(brightness) => brightness.to_string(),
                None => "".to_string(),
            };
            let keyboard_brightness = parser::parse_brightness(&brightness);

            let mode = match matches.get_one::<String>("mode") {
                Some(mode) => mode.to_string(),
                None => "".to_string(),
            };
            let keyboard_mode = parser::parse_mode(&mode);

            if keyboard_mode == Mode::RGB {
                let rgb_colors: [u8; 3] = parser::parse_rgb_colors(&color);
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
        Some(("load", matches)) => {
            let file_name = match matches.get_one::<String>("file") {
                Some(file_name) => file_name.to_string(),
                None => "".to_string(),
            };
            let file_lines: Vec<String> = match std::fs::read_to_string(file_name) {
                Ok(file_data) => file_data
                    .split("\n")
                    .into_iter()
                    .map(|item| item.to_string())
                    .collect(),
                Err(error) => {
                    println!("Unable to read file: {}", error);
                    std::process::exit(1);
                }
            };
            let loop_forever = file_lines.contains(&"loop_forever".to_string());

            loop {
                for line in &file_lines {
                    if line.trim().len() <= 0 {
                        continue;
                    }

                    let mut region = String::new();
                    let mut color = String::new();
                    for segment in line.split(",") {
                        let segment = segment.trim().to_string();
                        let action: Vec<String> = segment
                            .split(":")
                            .into_iter()
                            .map(|item| item.to_string())
                            .collect();
                        match action[0].trim().to_lowercase().as_str() {
                            "reset" => keyboard.reset().unwrap(),
                            "off" => keyboard.off().unwrap(),
                            "region" => region = action[1].clone(),
                            "color" => color = action[1].clone(),
                            "sleep" => std::thread::sleep(std::time::Duration::from_millis(
                                action[1].parse().unwrap_or(0),
                            )),
                            _ => (),
                        }
                    }
                    if region == "" && color == "" {
                        continue;
                    }

                    let rgb_colors = parser::parse_rgb_colors(&color);
                    keyboard
                        .set_rgb_color(&KeyboardRGBLightData::new(
                            &parser::parse_region(&region),
                            &(rgb_colors[0], rgb_colors[1], rgb_colors[2]),
                        ))
                        .unwrap();
                }

                if !loop_forever {
                    break;
                }
            }
        }
        _ => unreachable!(),
    }
}
