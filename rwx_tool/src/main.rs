use std::env;
use std::fs;

use rwx_lexer::lex;
use rwx_parser::parse;

// import the function from the module
use crate::export_obj::export_obj;

mod export_obj;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: rwx_tool <input.rwx> <output_base> [--debug]");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];
    let debug = args.iter().any(|a| a == "--debug");

    let text = fs::read_to_string(input)
        .expect("Failed to read RWX file");

    let tokens = lex(&text);
    let model = parse(&tokens);

    if debug {
        println!("{:#?}", model);
    }

    // this now works
    if let Err(e) = export_obj(&model, output) {
        eprintln!("Error exporting OBJ: {}", e);
        std::process::exit(1);
    }

    println!("Exported OBJ + MTL to {output}.obj and {output}.mtl");
}
