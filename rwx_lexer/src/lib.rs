use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Token {
    VertexExt,
    Vertex,
    Quad,
    Polygon,
    UV,
    Number(f32),
    Ident(String),
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let mut parts = line.split_whitespace();

        while let Some(word) = parts.next() {
            match word {
                "VertexExt" => tokens.push(Token::VertexExt),
                "Vertex"    => tokens.push(Token::Vertex),
                "Quad"      => tokens.push(Token::Quad),
                "Polygon"   => tokens.push(Token::Polygon),
                "UV"        => tokens.push(Token::UV),

                // Numbers
                _ if word.parse::<f32>().is_ok() => {
                    tokens.push(Token::Number(word.parse::<f32>().unwrap()));
                }

                // Identifiers (material names, texture names)
                _ => tokens.push(Token::Ident(word.to_string())),
            }
        }
    }

    tokens
}
