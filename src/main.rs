use std::process;
use std::time::Duration;
use std::thread::sleep;
use std::fs::File;
use std::io::prelude::*;
use reqwest;
use log::{debug, error, info};

fn main() {
    env_logger::init();

    let sleep_sec = 1;
    let sleep_sec_dur = Duration::from_secs(sleep_sec);
    let mut last_active_count = 0;
    let mut location = String::new();

    info!("Starting main loop...");
    loop {
        let active_count = get_active_interfaces();

        if active_count != last_active_count {
            info!("Network interfaces changes detected: {} => {}.", last_active_count, active_count);
            location = update_location();
            save_location(&location).unwrap_or_else(|err| {
                error!("Failed to save location: '{}'.", err);
                process::exit(1);
            });

            last_active_count = active_count;
        }

        debug!("Current location: {}.", location);
        debug!("Going to sleep seconds: {}.", sleep_sec);
        sleep(sleep_sec_dur);
    }
}

fn get_active_interfaces() -> u32 {
    let mut active_int_count = 0;

    let interfaces = netdev::get_interfaces();
    debug!("Interfaces number: {}.", interfaces.len());

    for interface in interfaces {
        if interface.is_up() == true {
            active_int_count += 1;
            debug!("Active interface: {}.", interface.name)
        }
    }

    debug!("Active interfaces number: {}.", active_int_count);

    return active_int_count;
}

fn update_location() -> String {
    let location = String::from(get_location());
    info!("Got location update: {}.", location);
    return location
}

fn get_location () -> String {
    let resp = match reqwest::blocking::get("http://ipinfo.io/country") {
        Ok(resp) => resp.text().unwrap().replace("\n", ""),
        Err(err) => panic!("Error: {}", err)
    };

    return resp
}

fn save_location(location: &String) -> std::io::Result<()> {
    let mut file = File::create("/tmp/whereamifrom")?;
    file.write_all(&location.as_bytes())?;
    Ok(())
}
