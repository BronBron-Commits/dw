// rwx_parser/src/parser.rs

use crate::lexer::Token;
use crate::ast::{RwxObject, RwxMesh, RwxVertex, RwxFace, RwxPrototype};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }
    
    // --- Utility Functions ---

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    
    fn expect(&mut self, expected_token: Token) -> Result<(), String> {
        match self.next_token() {
            Some(actual) if *actual == expected_token => Ok(()),
            Some(actual) => Err(format!("Expected token {:?}, found {:?}", expected_token, actual)),
            None => Err(format!("Expected token {:?}, found EOF", expected_token)),
        }
    }
    
    fn next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        if token.is_some() {
            self.position += 1;
        }
        token
    }
    
    fn expect_number(&mut self) -> Result<f32, String> {
        match self.next_token() {
            Some(Token::Number(n)) => Ok(*n),
            Some(t) => Err(format!("Expected Number, found {:?}", t)),
            None => Err("Expected Number, found EOF".to_string()),
        }
    }
    
    // Helper to consume a fixed number of arguments
    fn consume_arguments(&mut self, count: usize, command_name: &str) -> Result<(), String> {
        for i in 0..count {
            if self.expect_number().is_err() {
                return Err(format!("Failed to parse argument {} of {} for command {}.", i + 1, count, command_name));
            }
        }
        Ok(())
    }

    // --- Core Parsing Functions ---

    // Parses a single RwxVertex: Vertex x y z UV u v (handles both Vertex and VertexExt)
    fn parse_vertex(&mut self) -> Result<RwxVertex, String> {
        // Consume the keyword (Vertex or VertexExt)
        match self.next_token() {
            Some(Token::VertexKeyword) | Some(Token::VertexExtKeyword) => {}
            Some(t) => return Err(format!("Expected Vertex or VertexExt keyword, found {:?}", t)),
            None => return Err("Expected Vertex or VertexExt keyword, found EOF.".to_string()),
        };
        
        // 1. Position Coordinates (X, Y, Z)
        let x = self.expect_number()?;
        let y = self.expect_number()?;
        let z = self.expect_number()?;
        
        // 2. Expect and consume the UV keyword
        self.expect(Token::UVKeyword)?; 

        // 3. Texture Coordinates (U, V)
        let u = self.expect_number()?;
        let v = self.expect_number()?;
        
        Ok(RwxVertex { x, y, z, u, v })
    }

    // Dispatches and consumes a command inside a structural block (Prototype or Clump)
    fn handle_structural_command(&mut self, vertices: &mut Vec<RwxVertex>, faces: &mut Vec<RwxFace>) -> Result<bool, String> {
        // Peek at the token to decide what to do
        let token_ref = match self.peek() {
            Some(t) => t,
            None => return Err("Unexpected EOF inside structural block.".to_string()),
        };

        // If it's a structural boundary token, return true to stop the loop
        if matches!(token_ref, Token::ProtoEndKeyword | Token::ClumpEndKeyword) {
            self.next_token(); // Consume the end token
            return Ok(true);
        }

        // --- Command Dispatch ---
        // Note: We use the `token_ref` for pattern matching, but inside each branch 
        // we use `self.next_token()` to consume the token.
        match token_ref {
            // Data Commands
            &Token::VertexKeyword | &Token::VertexExtKeyword => {
                // parse_vertex handles consuming its own keyword
                vertices.push(self.parse_vertex()?);
            }
            &Token::QuadKeyword => {
                self.next_token(); // Consume QUAD
                // Consume 4 indices
                let i1 = self.expect_number()? as usize;
                let i2 = self.expect_number()? as usize;
                let i3 = self.expect_number()? as usize;
                let i4 = self.expect_number()? as usize;
                faces.push(RwxFace { indices: vec![i1, i2, i3, i4], texture: None });
            }
            &Token::PolygonKeyword => {
                self.next_token(); // Consume POLYGON
                let count = self.expect_number()? as usize; // Read index count
                let mut indices = Vec::with_capacity(count);
                for _ in 0..count {
                    indices.push(self.expect_number()? as usize);
                }
                faces.push(RwxFace { indices, texture: None });
            }

            // Command Consumption (Robustly consume arguments)
            &Token::TransformKeyword => {
                self.next_token(); // Consume TRANSFORM
                self.consume_arguments(16, "Transform")?;
            }
            &Token::ColorKeyword | &Token::AmbientKeyword | &Token::DiffuseKeyword | &Token::SpecularKeyword => {
                self.next_token(); // Consume command keyword
                self.consume_arguments(3, "Color/Property")?;
            }
            &Token::SurfaceKeyword => {
                self.next_token(); // Consume SURFACE
                self.consume_arguments(3, "Surface")?;
            }
            &Token::OpacityKeyword => {
                self.next_token(); // Consume OPACITY
                self.consume_arguments(1, "Opacity")?;
            }
            &Token::TextureKeyword => {
                self.next_token(); // Consume TEXTURE
                match self.next_token() {
                    Some(Token::Ident(_)) | Some(Token::StringLiteral(_)) => {}
                    _ => return Err("Expected texture name after TEXTURE.".to_string()),
                }
            }
            &Token::TextureModesKeyword | &Token::LightSamplingKeyword | &Token::GeometrySamplingKeyword | &Token::MaterialModesKeyword | &Token::IdentityKeyword | &Token::IdentityJointKeyword | &Token::AddTextureModeKeyword => {
                self.next_token(); // Consume the keyword
                // Skip subsequent Identifiers or Numbers until a new command is hit
                while let Some(t) = self.peek() {
                    if matches!(t, Token::Ident(_) | Token::Number(_)) {
                         self.next_token();
                    } else {
                        break;
                    }
                }
            }
            
            // --- Robust Fallback (Fixes E0502) ---
            _ => {
                // Must consume the token first, then clone the consumed value for printing
                // This resolves the conflict between the mutable borrow (next_token) 
                // and the immutable borrow (token_ref/token)
                let consumed_token = self.next_token().unwrap().clone();
                eprintln!("Warning: Skipping unrecognized command token: {:?}", consumed_token);
            }
        }
        Ok(false)
    }

    // Parses the content between ProtoBegin and ProtoEnd
    fn parse_prototype(&mut self) -> Result<RwxPrototype, String> {
        self.expect(Token::ProtoBeginKeyword)?;
        
        let name = match self.next_token() {
            Some(Token::Ident(s)) => s.clone(),
            _ => return Err("Expected prototype name (Ident) after ProtoBegin.".to_string()),
        };
        
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        
        // This loop handles the entire content block for the Prototype
        loop {
            if self.peek().is_none() {
                return Err("Unexpected EOF while parsing Prototype content.".to_string());
            }
            
            // Dispatch to the handler. If it returns true, we break the loop.
            if self.handle_structural_command(&mut vertices, &mut faces)? {
                break;
            }
        }
        
        Ok(RwxPrototype { 
            name, 
            mesh: RwxMesh { vertices, faces },
        })
    }
    
    // Main entry point for the parser (handles ModelBegin/Clump/ModelEnd)
    pub fn parse(&mut self) -> Result<Vec<RwxObject>, String> {
        let mut objects: Vec<RwxObject> = Vec::new();
        
        self.expect(Token::ModelBeginKeyword)?;

        loop { // Use a loop that breaks explicitly
            let t = match self.peek() {
                Some(token) => token.clone(), // Clone the token to own the value, ending the immutable borrow
                None => break, // Break the loop if we hit EOF
            };
            
            match t {
                Token::ModelEndKeyword | Token::Eof => {
                    if t == Token::ModelEndKeyword {
                        self.expect(Token::ModelEndKeyword)?;
                    }
                    break;
                }
                
                // Expected Data Block (Prototypes)
                Token::ProtoBeginKeyword => {
                    objects.push(RwxObject::Prototype(self.parse_prototype()?));
                }
                
                // Block Skipping Logic (Handles nested blocks like TransformBegin/JointTransformBegin)
                Token::TransformBeginKeyword | Token::JointTransformBeginKeyword => {
                    // Determine the end token *before* consuming the start token
                    let end_token = match t {
                        Token::TransformBeginKeyword => Token::TransformEndKeyword,
                        Token::JointTransformBeginKeyword => Token::JointTransformEndKeyword,
                        _ => unreachable!(), // Should not happen
                    };
                    
                    self.next_token(); // CONSUME the BEGIN keyword (Mutable action)
                    
                    // Skip everything inside until the END token is found
                    while self.peek().map_or(false, |p| *p != end_token) {
                         self.next_token();
                    }
                    self.expect(end_token)?; // Consume the END keyword
                }
                
                // Clump Logic (Geometry is inside here for couch2c.rwx)
                Token::ClumpBeginKeyword => {
                    self.next_token(); // CONSUME ClumpBeginKeyword
                    
                    let mut vertices = Vec::new();
                    let mut faces = Vec::new();

                    // Loop through all commands inside the Clump
                    while !self.peek().map_or(false, |p| *p == Token::ClumpEndKeyword) {
                        // handle_structural_command will process commands or consume the end token
                        if self.handle_structural_command(&mut vertices, &mut faces)? {
                            break; 
                        }
                    }
                    self.expect(Token::ClumpEndKeyword)?; // Consume ClumpEnd
                    
                    // If geometry was found inside the Clump, extract it as a root object
                    if !vertices.is_empty() {
                         objects.push(RwxObject::Prototype(RwxPrototype { 
                             name: "Clump_Geometry".to_string(), 
                             mesh: RwxMesh { vertices, faces },
                         }));
                    }
                }
                
                // Consume single tokens at the top level (e.g., IDENTITY)
                _ => { 
                    self.next_token();
                }
            }
        }

        Ok(objects)
    }
}