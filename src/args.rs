use anyhow::Result;
use helium_crypto::Network;
use regex::Regex;
use std::{
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};
use structopt::StructOpt;

use crate::key::Key;
use crate::words::{ADJECTIVES, ANIMALS, COLORS};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "vain-purple-tiger",
    about = "A Helium node vanity name generator."
)]
pub struct Opt {
    /// Silence all output
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,

    /// Timestamp (sec, ms, ns, none)
    #[structopt(short = "t", long = "timestamp")]
    pub ts: Option<stderrlog::Timestamp>,

    /// generate the key for either MainNet or TestNet
    #[structopt(short, long, default_value = "main")]
    pub network: String,

    /// how many threads to use
    #[structopt(short, long)]
    pub cpus: Option<u64>,

    /// output path to save the swarm key
    #[structopt(short, long, default_value = "swarm_key")]
    pub output: String,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "words")]
    /// generate a swarm key containing specific words (must supply at least one)
    Words {
        #[structopt(short = "j", long)]
        adjective: Option<String>,

        #[structopt(short, long)]
        color: Option<String>,

        #[structopt(short = "a", long)]
        animal: Option<String>,
    },

    #[structopt(name = "letter")]
    /// generate an alliterative swarm key using the provided letter
    Letter { letter: String },

    #[structopt(name = "regex")]
    /// generate a swarm key name from the provided regex
    Regex { regex: String },

    #[structopt(name = "lists")]
    Lists,
}

pub fn find_key(
    network: Network,
    cpus: u64,
    reg_str: Regex,
    num_keys_checked: Arc<Mutex<u64>>,
) -> Result<Key> {
    let (tx, rx): (Sender<Key>, Receiver<Key>) = channel();
    let is_key_found = Arc::new(AtomicBool::new(false));

    for _ in 0..cpus {
        let tx = tx.clone();
        let reg_str = reg_str.clone();
        let is_key_found = is_key_found.clone();
        let counter = Arc::clone(&num_keys_checked);

        thread::spawn(
            move || match check_key(network, reg_str, is_key_found, counter) {
                Some(keypair) => tx.send(keypair).expect("failed to send on channel"),
                None => (),
            },
        );
    }

    Ok(rx.recv()?)
}

fn check_key(
    network: Network,
    reg_str: Regex,
    is_key_found: Arc<AtomicBool>,
    counter: Arc<Mutex<u64>>,
) -> Option<Key> {
    let mut keypair = Key::generate(network);

    let result = loop {
        // let counter = Arc::clone(&counter);

        if reg_str.is_match(&keypair.name) {
            is_key_found.store(true, Ordering::Relaxed);
            break Some(keypair);
        } else if is_key_found.load(Ordering::Relaxed) {
            break None;
        }

        *counter.lock().unwrap() += 1;

        keypair = Key::generate(network);
    };

    result
}

pub fn handle_subcommands(cmd: Command) -> Regex {
    match cmd {
        Command::Words {
            adjective,
            color,
            animal,
        } => handle_words(adjective, color, animal),
        Command::Letter { letter } => handle_letter(letter),
        Command::Regex { regex } => handle_regex(regex),
        Command::Lists => {
            println!("Adjectives: {:?}\n", ADJECTIVES);
            println!("Colors: {:?}\n", COLORS);
            println!("Animals: {:?}\n", ANIMALS);
            process::exit(0);
        }
    }
}

fn handle_words(adjective: Option<String>, color: Option<String>, animal: Option<String>) -> Regex {
    if adjective.is_none() & color.is_none() & animal.is_none() {
        panic!("Must provide at least one of the words options!");
    }

    let adjective = if let Some(word) = adjective {
        if ADJECTIVES.contains(&(&word[..])) {
            word
        } else {
            println!("Not a valid adjective!");
            process::exit(1);
        }
    } else {
        String::from(r"\w+")
    };

    let color = if let Some(word) = color {
        if COLORS.contains(&(&word[..])) {
            word
        } else {
            println!("Not a valid color!");
            process::exit(1);
        }
    } else {
        String::from(r"\w+")
    };

    let animal = if let Some(word) = animal {
        if ANIMALS.contains(&(&word[..])) {
            word
        } else {
            println!("Not a valid animal!");
            process::exit(1);
        }
    } else {
        String::from(r"\w+")
    };

    Regex::new(&format!("{}-{}-{}", adjective, color, animal)).expect("failed to create regex!")
}

fn handle_letter(letter: String) -> Regex {
    Regex::new(&format!(r"^{}\w+-{}\w+-{}\w+$", letter, letter, letter))
        .expect("failed to create regex!")
}

fn handle_regex(regex: String) -> Regex {
    println!(
        "Searching for key of format: {}.\n Invalid regexes will never complete.",
        regex
    );
    Regex::new(&format!("{}", regex)).expect("failed to create regex!")
}
