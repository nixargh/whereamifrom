use std::process;
use std::time::Duration;
use std::thread::sleep;
use std::fs::File;
use std::io::prelude::*;
use log::{debug, error, info};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Url to get location information from
    #[arg(short, long, env, default_value = "http://ipinfo.io/country")]
    url: String,

    /// File path to save location to
    #[arg(short, long, env, default_value = "/tmp/whereamifrom")]
    file: String,

    /// Seconds to sleep between network interfaces checks
    #[arg(short, long, env, default_value_t = 1)]
    sleep: u64,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let sleep_sec_dur = Duration::from_secs(args.sleep);
    let mut last_active_count = 0;
    let mut location = String::new();

    info!("Starting main loop...");
    loop {
        let active_count = get_active_interfaces();

        if active_count != last_active_count {
            info!("Network interfaces changes detected: {} => {}.", last_active_count, active_count);

            location = get_location(&args.url);

            save_location(&location, &args.file).unwrap_or_else(|err| {
                error!("Failed to save location: '{}'.", err);
                process::exit(1);
            });

            last_active_count = active_count;
        }

        debug!("Current location: {}.", location);
        debug!("Going to sleep seconds: {}.", args.sleep);
        sleep(sleep_sec_dur);
    }
}

fn get_active_interfaces() -> u32 {
    let mut active_int_count = 0;

    let interfaces = netdev::get_interfaces();
    debug!("Interfaces number: {}.", interfaces.len());

    for interface in interfaces {
        if interface.is_up() {
            active_int_count += 1;
            debug!("Active interface: {}.", interface.name)
        }
    }

    debug!("Active interfaces number: {}.", active_int_count);
    active_int_count
}

fn get_location(url: &String) -> String {
    debug!("Requesting location update from: {}.", url);

    let location = do_request(url).unwrap_or_else(|err| {
        error!("Failed to update location: {}.", err);
        String::from("NaN")
    });

    info!("Got location update: {}.", location);
    location
}

fn do_request(url: &String) -> Result<String, String> {
    match reqwest::blocking::get(url) {
        Ok(resp) => Ok(resp.text().unwrap().replace("\n", "")),
        Err(err) => Err(err.to_string()),
    }
}

fn save_location(location: &String, file: &String) -> std::io::Result<()> {
    debug!("Writing location to file: '{}'.", file);

    let mut file_obj = File::create(file)?;
    file_obj.write_all(location.as_bytes())?;

    info!("Location saved to: '{}'.", file);
    Ok(())
}
