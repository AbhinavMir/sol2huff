use clap::{ App, Arg };
use std::fs;
use std::path::PathBuf;
use ethers_solc::{ Project, ProjectPathsConfig };
mod helpers;
use helpers::{
    compile_solidity,
    find_all_json_files,
    find_all_sol_files,
    get_bytecode_and_path_from_json,
    bytecode_to_huff,
};
use std::process::Command;

fn mx() {
    let output = Command::new("solc")
        .arg("--version")
        .output()
        .expect("Failed to run solc command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("solc version: {}", stdout);
    } else {
        eprintln!(
            "Please install solc and add it to your PATH - more details on https://docs.soliditylang.org/en/v0.8.17/installing-solidity.html"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error running solc command: {}", stderr);
        std::process::exit(1);
    }

    let matches = App::new("Sol2Huff")
        .version("0.1.0")
        .author("Abhinav `August` Srivastava <atg271@gmail.com>")
        .about("Transpiles Solidity contracts to Huff")
        .arg(
            Arg::with_name("contracts")
                .short("c")
                .long("contracts")
                .value_name("DIR")
                .help("Sets the contracts directory")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("artifacts")
                .short("a")
                .long("artifacts")
                .value_name("DIR")
                .help("Sets the artifacts directory")
                .takes_value(true)
        )
        .arg(Arg::with_name("verbose").short("v").long("verbose").help("Prints verbose output"))
        .get_matches();

    let contracts_dir = matches.value_of("contracts").unwrap_or_else(|| "contracts");
    let artifacts_dir = matches.value_of("artifacts").unwrap_or_else(|| "artifacts");
    let verbose = matches.is_present("verbose");

    compile_solidity(contracts_dir);
    let json_files = find_all_json_files(artifacts_dir);
    let sol_files = find_all_sol_files(contracts_dir);

    for (file_name, _) in sol_files.iter() {
        println!("INFO:Transpiling {}...", file_name);
    }
}

fn main() {
    let project = Project::builder()
        .paths(ProjectPathsConfig::hardhat(env!("CARGO_MANIFEST_DIR")).unwrap())
        .build()
        .unwrap();
    let output = project.compile().unwrap();
    project.rerun_if_sources_changed();
}