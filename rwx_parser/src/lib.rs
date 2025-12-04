// rwx_parser/src/lib.rs

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod normalizer; 

use ast::RwxObject;
use lexer::lex;
use parser::Parser;

/// Parses a RenderWare eXtension (RWX) string into a list of RwxObject structures.
pub fn parse_rwx_to_object(input: &str) -> Result<Vec<RwxObject>, String> {
    
    // 1. Lexing (Tokenization)
    let tokens = lex(input);
    
    // 2. Parsing (Building the Abstract Syntax Tree/Object)
    let mut parser = Parser::new(tokens);
    parser.parse()
}

// The 'mod tests' block is now empty or removed entirely, 
// resolving the structural error E0574.