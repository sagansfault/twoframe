extern crate core;

use std::env::current_dir;
use std::fs::read_to_string;
use std::time::Duration;

use hidapi::HidApi;
use tokio::time::Instant;

use crate::gamepad::{Config, GamePad, GGSTButton};

mod gamepad;

fn main() {

    let conf_result = load_config();
    let conf: Config = match conf_result {
        Some(c) => c,
        None => {
            println!("\n\nCould not load config, using default binds.\n\n");
            Config::default()
        }
    };

    let hidapi = HidApi::new().expect("HidApi could not be made");
    let device_info = hidapi
        .device_list()
        .next()
        .expect("No devices are available!")
        .clone();
    println!(
        "Opening device:\n VID: {:04x}, PID: {:04x}\n",
        device_info.vendor_id(),
        device_info.product_id()
    );

    let device = device_info.open_device(&hidapi).expect("Could not open device");

    let mut input_bin: Vec<(Duration, GamePad)> = vec![];
    let mut instant = Instant::now();
    loop {
        let mut buf = [0u8; 64];
        let _ = device.read(&mut buf[..]);

        let gamepad = GamePad::new().get_state(&buf);
        if gamepad.share_button.pressed {
            print_input_bin(input_bin.clone(), &conf);
            input_bin.clear();
            instant = Instant::now();
            continue;
        } else {
            input_bin.push((instant.elapsed(), gamepad));
        }
    }
}

pub fn load_config() -> Option<Config> {
    let mut p = current_dir().map_or(None, |c| Some(c))?;
    p.push("twoframe_config.txt");
    let s = read_to_string(p).map_or(None, |c| Some(c))?;
    let conf: Config = serde_json::from_str(&s).map_or(None, |c| Some(c))?;
    return Some(conf);
}

pub fn print_input_bin(bin: Vec<(Duration, GamePad)>, conf: &Config) {
    if bin.is_empty() {
        return;
    }
    let mut frame_bins = vec![GamePad::new(); 10000].into_boxed_slice();
    for (dur, state) in bin {
        frame_bins[(dur.as_nanos() / 16_666_666) as usize] = state.clone();
    }

    let mut string: String = String::new();

    let mut prior_state: &GamePad = &GamePad::new();
    for state in frame_bins.iter() {
        let mut state_string: Vec<String> = vec![];
        for button_id in GamePad::get_buttons() {
            let prior = prior_state.get_button(button_id);
            let current = state.get_button(button_id);

            if let Some(val) = conf.get_ggst_bind(button_id) {
                if current.pressed {
                    if !prior.pressed {
                        // only care when H is released
                        if val != GGSTButton::H {
                            state_string.push(format!("{:?}", val));
                        }
                        continue;
                    }
                } else {
                    if prior.pressed {
                        // only care when H is released
                        if val == GGSTButton::H {
                            state_string.push(format!("]{:?}[", val));
                        }
                        continue;
                    }
                }
            }
        }
        if state_string.is_empty() {
            state_string.push(String::from("-"));
        }
        string.push_str(state_string.join(",").as_str());
        prior_state = state;
    }

    string = String::from(string.trim_start_matches("-").trim_end_matches("-"));
    println!("{}", string);
}

