use clap::Parser;
use colored::*;
use json;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    keys: String,
}

fn main() {
    let args = Args::parse();
    // * Input parsing
    let input_str = fs::read_to_string(args.file).expect("unable to read file");
    let input_json = json::parse(&input_str).unwrap();

    let keys_str = fs::read_to_string(args.keys).expect("unable to read file");
    let keys_json = json::parse(&keys_str).unwrap();

    println!("|{: ^20}|{: ^20}|", "Target Key".bold(), "Match".bold(),);

    for (key, _) in keys_json.entries() {
        if input_json.has_key(&key) {
            println!("|{:>20}|{:>20}|", key.green(), "OK".green())
        } else {
            println!("|{:>20}|{:>20}|", key.red(), "MISSING".red().bold())
        }
    }
}
