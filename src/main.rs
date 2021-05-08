use anyhow::Result;
use helium_crypto::Network;
use log::*;
use num_cpus;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{fs::File, process};
use structopt::StructOpt;

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

    let cpus: u32 = if let Some(num) = &options.cpus {
        *num
    } else {
        num_cpus::get() as u32
    };

    let reg_str = args::handle_subcommands(options.cmd);

    let num_keys_checked = Arc::new(Mutex::new(0u64));

    let counter = Arc::clone(&num_keys_checked);

    println!(
        "Network: {} | CPUS: {}",
        &options.network.to_lowercase(),
        cpus
    );
    println!("Starting key generation.");
    let start = Instant::now();
    let key = args::find_key(network, cpus, reg_str, counter)?;
    let duration = start.elapsed();
    let keys_checked = num_keys_checked.lock().unwrap();
    let key_rate = *keys_checked as f64 / duration.as_secs_f64();

    println!("Found key! Address: {} Name: {}", key.address, key.name);
    println!(
        "Checked {:?} keys in {:?}, averaging {:?} keys per second.",
        keys_checked,
        duration,
        key_rate.round()
    );

    let mut buffer = File::create(options.output)?;
    key.write(&mut buffer)?;

    Ok(())
}
