use clap::Parser;
use json::{self};
use std::{fs, path::PathBuf};
mod key_validation;
mod json_utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None, author="Fernando Crozetta")]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long)]
    keys_to_check: String,
}

fn main() {
    let args = Args::parse();
    // * Input parsing
    let input_str = fs::read_to_string(&args.file).expect("unable to read json file");
    let input_json = json::parse(&input_str).unwrap();

    let keys_str = fs::read_to_string(args.keys_to_check).expect("unable to read keys file");
    let keys_json = json::parse(&keys_str).unwrap();

    // key_validation::validate(input_json, keys_json);
}
