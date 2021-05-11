use anyhow::Result;
use chrono::Duration;
use helium_crypto::Network;
use log::*;
use num_cpus;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{fs::File, process};
use structopt::StructOpt;
use timer;

mod args;
mod key;
mod words;

fn main() -> Result<()> {
    let options = args::Opt::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(options.quiet)
        .verbosity(options.verbose)
        .timestamp(options.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()?;

    let network = match &options.network.to_lowercase()[..] {
        "main" | "mainnet" => Network::MainNet,
        "test" | "testnet" => Network::TestNet,
        _ => {
            error!("Invalid network value!");
            println!("Invalid network value! Use 'testnet' or 'mainnet'.");
            process::exit(1);
        }
    };

    let cpus: u64 = if let Some(num) = &options.cpus {
        *num
    } else {
        num_cpus::get() as u64
    };

    let timer_period = if let Some(num) = &options.timer_period {
        *num
    } else {
        30
    };

    let reg_str = args::handle_subcommands(options.cmd);

    let num_keys_checked = Arc::new(Mutex::new(0u64));
    let counter = Arc::clone(&num_keys_checked);
    let timer = timer::Timer::new();
    let mut previous_keys_checked = 0;

    println!(
        "Network: {} | CPUS: {}",
        &options.network.to_lowercase(),
        cpus
    );
    println!("Starting key generation.");
    let start = Instant::now();
    let counter_clone = Arc::clone(&num_keys_checked);
    let _guard = timer.schedule_repeating(Duration::seconds(timer_period as i64), move || {
        let keys_checked = counter_clone.lock().unwrap();
        let period_keys_checked = *keys_checked - previous_keys_checked;
        let key_rate = period_keys_checked / timer_period;
        println!(
            "Checked {:?} keys in {:?} seconds, averaging {:?} keys per second.",
            period_keys_checked, timer_period, key_rate
        );
        previous_keys_checked = *keys_checked;
    });

    let key = args::find_key(network, cpus, reg_str, counter)?;
    let duration = start.elapsed();
    let keys_checked = num_keys_checked.lock().unwrap();
    let key_rate = *keys_checked as f64 / duration.as_secs_f64();

    println!(
        "*************\nFound key!\nAddress: {}\nName: {}",
        key.address, key.name
    );
    println!(
        "Checked {:?} keys in {:?} seconds, averaging {:?} keys per second, {:?} keys per core per second.",
        keys_checked,
        duration.as_secs(),
        key_rate as u64,
        (key_rate as u64 / cpus)
    );

    let mut buffer = File::create(options.output)?;
    key.write(&mut buffer)?;

    Ok(())
}
