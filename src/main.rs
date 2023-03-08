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

fn main() {


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

    let contracts_dir = matches.value_of("contracts").unwrap_or_else(|| "$PWD/contracts");
    let artifacts_dir = matches.value_of("artifacts").unwrap_or_else(|| "$PWD/artifacts");
    let verbose = matches.is_present("verbose");

    for file in fs::read_dir(contracts_dir).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".sol") {
            compile_solidity(file_name);
        }
    }
    
    let json_files = find_all_json_files(artifacts_dir);
    let sol_files = find_all_sol_files(contracts_dir);

    for (file_name, _) in sol_files.iter() {
        println!("INFO:Transpiling {}...", file_name);

    }
}

fn maixn() {
    let project = Project::builder()
        .paths(ProjectPathsConfig::hardhat(env!("CARGO_MANIFEST_DIR")).unwrap())
        .build()
        .unwrap();
    let output = project.compile().unwrap();
    project.rerun_if_sources_changed();
}