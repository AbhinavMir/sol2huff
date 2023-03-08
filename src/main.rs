use clap::{App, Arg};
use std::fs;
use std::path::PathBuf;

mod helpers;
use helpers::{compile_solidity, find_all_json_files, get_bytecode_and_path_from_json, bytecode_to_huff};

fn main() {
    let matches = App::new("Sol2Huff")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("Transpiles Solidity contracts to Huff")
        .arg(
            Arg::with_name("contracts")
                .short("c")
                .long("contracts")
                .value_name("DIR")
                .help("Sets the contracts directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("artifacts")
                .short("a")
                .long("artifacts")
                .value_name("DIR")
                .help("Sets the artifacts directory")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Prints verbose output"),
        )
        .get_matches();

    let contracts_dir = matches
        .value_of("contracts")
        .unwrap_or("contracts");
    let artifacts_dir = matches
        .value_of("artifacts")
        .unwrap_or("artifacts");
    let verbose = matches.is_present("verbose");

    compile_solidity(contracts_dir);
    let sol_files = find_all_json_files(artifacts_dir);
    for (file_name, _) in sol_files.iter() {
        println!("INFO:Transpiling {}...", file_name);
    }
}