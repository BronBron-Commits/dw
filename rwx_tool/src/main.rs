// rwx_tool/src/main.rs

// FIX A: Correctly import the Clump variant from the RwxObject enum.
use rwx_parser::ast::{RwxObject, RwxMesh, RwxPrototype}; 
use rwx_parser::parse_rwx_to_object; // Import the renamed function

use std::fs;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <rwx_file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Read the file content
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            process::exit(1);
        }
    };

    // FIX B: Call the function with the new, correct name.
    let rwx_objects = match parse_rwx_to_object(&content) {
        Ok(objects) => objects,
        Err(e) => {
            eprintln!("Parsing Error: {}", e);
            process::exit(1);
        }
    };

    // Output the resulting structure (using serde_json for easy viewing)
    match serde_json::to_string_pretty(&rwx_objects) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing output: {}", e),
    }
}