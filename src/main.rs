use anyhow::Result;
use helium_crypto::Network;
use log::*;
use num_cpus;
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

    println!(
        "Network: {} CPUS: {}",
        &options.network.to_lowercase(),
        cpus
    );
    println!("Starting key generation.");
    let key = args::find_key(network, cpus, reg_str)?;
    println!("Found key! Address: {} Name: {}", key.address, key.name);

    let mut buffer = File::create(options.output)?;
    key.write(&mut buffer)?;

    Ok(())
}
