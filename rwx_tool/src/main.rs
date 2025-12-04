use rwx_parser::lexer::lex;
use rwx_parser::normalizer::normalize;
use rwx_parser::parser::RwxParser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rwx_tool <file.rwx>");
        return;
    }

    let path = &args[1];

    // 1. Read file
    let text = std::fs::read_to_string(path).expect("Failed to read file");

    // 2. Lex
    let tokens = lex(&text);

    // 3. Normalize dialects
    let canon = normalize(&tokens);

    // 4. Parse canonical commands
    let obj = RwxParser::from_canon(&canon);

    // 5. Print stats
    let mesh = obj.mesh.unwrap();
    println!("Vertices: {}", mesh.vertices.len());
    println!("Faces: {}", mesh.faces.len());
}
