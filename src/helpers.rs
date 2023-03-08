// helpers.rs

use ethers_solc::{ Project, ProjectPathsConfig };
use murph::parser;
use std::fs;
use std::collections::HashMap;
use serde_json::Value;

pub fn compile_solidity(dir) {
    let project = Project::builder()
        .paths(ProjectPathsConfig::hardhat(env!("CARGO_MANIFEST_DIR")).unwrap())
        .build()
        .unwrap();
    let output = project.compile().unwrap();
    project.rerun_if_sources_changed();
    println!("Compiled contracts successfully!");
}

fn find_all_sol_files(dir: &str) -> HashMap<String, bool> {
    let mut sol_files: HashMap<String, bool> = HashMap::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if entry.file_type().unwrap().is_file() && file_name.ends_with(".sol") {
                        sol_files.insert(file_name.to_string(), false);
                    }
                }
            }
        }
    }
    sol_files
}

pub fn get_bytecode_and_path_from_json(
    contract_name: &str,
    contract_json: &str
) -> (String, String) {
    let contents = fs
        ::read_to_string(format!("artifacts/{}/{}", contract_name, contract_json))
        .expect("Failed to read file");
    let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    let bytecode = json["bytecode"]["object"]
        .as_str()
        .expect("Bytecode not found in JSON")
        .to_string();
    let path = json["ast"]["absolutePath"].as_str().expect("Path not found in JSON").to_string();
    (bytecode, path)
}

// pub fn read_config() -> String {
//     let contents = fs::read_to_string("sol2huff.config")
//         .expect("Failed to read file");
//     contents.to_string();
// }

pub fn bytecode_to_huff(bytecode: String, path: String, verbose: bool) {
    let huff = parser::parse(bytecode, false);
    let path = format!("{}.huff", path);
    write_to_file(&huff, &path, verbose);
}

fn write_to_file(content: &str, path: &str, verbose: bool) {
    let path = strip_sol_extension(path);
    let path = format!("{}.huff", path);
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(content.as_bytes()).expect("Failed to write to file");
    if verbose {
        println!("Transpiled Huff written to {} successfully", path);
    }
}

fn strip_sol_extension(file_name: &str) -> String {
    if file_name.ends_with(".sol") {
        return file_name[..file_name.len() - 4].to_string();
    }
    file_name.to_string()
}