// rwx_parser/src/lexer.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f32),
    StringLiteral(String),
    Comma,
    BeginBlock,
    EndBlock,
    Eof,

    // RWX KEYWORDS
    ModelBeginKeyword,
    ModelEndKeyword,
    ProtoBeginKeyword,
    ProtoEndKeyword,
    ClumpBeginKeyword,
    ClumpEndKeyword,
    
    // NEW BLOCK KEYWORDS (for couch2c structure)
    TransformBeginKeyword,
    TransformEndKeyword,
    JointTransformBeginKeyword,
    JointTransformEndKeyword,

    // GEOMETRY & MATERIAL COMMANDS
    TransformKeyword,
    VertexKeyword,
    VertexExtKeyword, // Added to handle the VertexExt keyword found in couch2c.rwx
    UVKeyword,
    FaceKeyword,
    QuadKeyword,
    PolygonKeyword, // Added to handle the Polygon command
    TextureKeyword,
    ColorKeyword,
    SurfaceKeyword,
    OpacityKeyword,
    
    // OTHER PROTO/CLUMP COMMANDS
    TextureModesKeyword,
    LightSamplingKeyword,
    GeometrySamplingKeyword,
    MaterialModesKeyword,
    ProtoInstanceKeyword,
    IdentityKeyword,
    IdentityJointKeyword,
    AmbientKeyword, // Added for Ambient property
    DiffuseKeyword, // Added for Diffuse property
    SpecularKeyword, // Added for Specular property
    AddTextureModeKeyword, // Added for AddTextureMode
    
    // Catch-all for unknown commands (e.g., Hints, Rotate, Scale, etc.)
}

fn is_number_char_continuation(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c.to_ascii_lowercase() == 'e' || c == '+' || c == '-'
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace: Consume ALL contiguous whitespace
            c if c.is_whitespace() => {
                chars.next(); 
                while let Some(&c2) = chars.peek() {
                    if c2.is_whitespace() {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }

            // Handle Comments (#)
            '#' => {
                chars.next(); // consume '#'
                while let Some(c2) = chars.next() {
                    if c2 == '\n' || c2 == '\r' {
                        break;
                    }
                }
            }

            // Punctuation
            '{' => { tokens.push(Token::BeginBlock); chars.next(); }
            '}' => { tokens.push(Token::EndBlock); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }

            // String Literals
            '"' => {
                chars.next(); 
                let mut s = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2 == '"' {
                        break;
                    }
                    s.push(c2);
                    chars.next();
                }
                chars.next(); 
                tokens.push(Token::StringLiteral(s));
            }

            // Numbers (CRITICAL BLOCK)
            c @ '-' | c if c.is_ascii_digit() || c == '.' => {
                let mut s = String::new();
                
                s.push(ch);
                chars.next(); 

                while let Some(&c2) = chars.peek() {
                    if is_number_char_continuation(c2) {
                        s.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                if let Ok(n) = s.parse::<f32>() {
                    tokens.push(Token::Number(n));
                } else {
                    eprintln!("Lexing Error: Failed to parse '{}' as f32. Treating as Ident.", s);
                    tokens.push(Token::Ident(s));
                }
            }

            // Identifiers and Keywords
            c if c.is_ascii_alphabetic() => {
                let mut s = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2.is_alphanumeric() || c2 == '_' {
                        s.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                // Check against known keywords
                match s.to_uppercase().as_str() {
                    "MODELBEGIN" => tokens.push(Token::ModelBeginKeyword),
                    "MODELEND" => tokens.push(Token::ModelEndKeyword),
                    "PROTOBEGIN" => tokens.push(Token::ProtoBeginKeyword),
                    "PROTOEND" => tokens.push(Token::ProtoEndKeyword),
                    "CLUMPBENIN" => tokens.push(Token::ClumpBeginKeyword),
                    "CLUMPEND" => tokens.push(Token::ClumpEndKeyword),
                    
                    "TRANSFORMBEGIN" => tokens.push(Token::TransformBeginKeyword),
                    "TRANSFORMEND" => tokens.push(Token::TransformEndKeyword),
                    "JOINTTRANSFORMBEGIN" => tokens.push(Token::JointTransformBeginKeyword),
                    "JOINTTRANSFORMEND" => tokens.push(Token::JointTransformEndKeyword),
                    
                    "TRANSFORM" => tokens.push(Token::TransformKeyword),
                    "VERTEX" => tokens.push(Token::VertexKeyword),
                    "VERTEXT" => tokens.push(Token::VertexExtKeyword), // Handles VertexExt found in couch2c.rwx
                    "UV" => tokens.push(Token::UVKeyword),
                    "FACE" => tokens.push(Token::FaceKeyword),
                    "QUAD" => tokens.push(Token::QuadKeyword),
                    "POLYGON" => tokens.push(Token::PolygonKeyword),
                    "TEXTURE" => tokens.push(Token::TextureKeyword),
                    "COLOR" => tokens.push(Token::ColorKeyword),
                    "SURFACE" => tokens.push(Token::SurfaceKeyword),
                    "OPACITY" => tokens.push(Token::OpacityKeyword),
                    
                    "TEXTUREMODES" => tokens.push(Token::TextureModesKeyword),
                    "LIGHTSAMPLING" => tokens.push(Token::LightSamplingKeyword),
                    "GEOMETRYSAMPLING" => tokens.push(Token::GeometrySamplingKeyword),
                    "MATERIALMODES" => tokens.push(Token::MaterialModesKeyword),
                    "PROTOINSTANCE" => tokens.push(Token::ProtoInstanceKeyword),
                    "IDENTITY" => tokens.push(Token::IdentityKeyword),
                    "IDENTITYJOINT" => tokens.push(Token::IdentityJointKeyword),
                    "AMBIENT" => tokens.push(Token::AmbientKeyword),
                    "DIFFUSE" => tokens.push(Token::DiffuseKeyword),
                    "SPECULAR" => tokens.push(Token::SpecularKeyword),
                    "ADDTEXTUREMODE" => tokens.push(Token::AddTextureModeKeyword),

                    // Fallback
                    _ => tokens.push(Token::Ident(s)),
                }
            }

            // Catch-all 
            _ => {
                chars.next();
            }
        }
    }
    
    tokens.push(Token::Eof);
    tokens
}