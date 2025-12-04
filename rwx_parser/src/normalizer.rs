use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Canon {
    Vertex(f32, f32, f32, f32, f32),
    Face(Vec<usize>),
    Material {
        texture: Option<String>,
        diffuse: Option<(f32, f32, f32)>,
        specular: Option<(f32, f32, f32)>
    },
    PushTransform,
    PopTransform,
}

pub fn normalize(tokens: &[Token]) -> Vec<Canon> {
    let mut out = Vec::new();
    let mut i = 0;

    let mut current_texture = None;
    let mut current_diffuse = None;
    let mut current_specular = None;

    while i < tokens.len() {
        match &tokens[i] {
            // ---------------------------------------------------------
            // VertexList format
            // ---------------------------------------------------------
            Token::Ident(s) if s == "VertexList" => {
                i += 1;
                let count = get_usize(tokens, &mut i);

                for _ in 0..count {
                    let x = get_f32(tokens, &mut i);
                    let y = get_f32(tokens, &mut i);
                    let z = get_f32(tokens, &mut i);

                    out.push(Canon::Vertex(x, y, z, 0.0, 0.0));
                }
            }

            // ---------------------------------------------------------
            // VertexExt format
            // ---------------------------------------------------------
            Token::Ident(s) if s == "VertexExt" => {
                i += 1;
                let x = get_f32(tokens, &mut i);
                let y = get_f32(tokens, &mut i);
                let z = get_f32(tokens, &mut i);

                // Expect: Ident("UV")
                i += 1;

                let u = get_f32(tokens, &mut i);
                let v = get_f32(tokens, &mut i);

                out.push(Canon::Vertex(x, y, z, u, v));
            }

            // ---------------------------------------------------------
            // Classic Vertex x y z [u v]
            // ---------------------------------------------------------
            Token::Ident(s) if s == "Vertex" => {
                i += 1;
                let x = get_f32(tokens, &mut i);
                let y = get_f32(tokens, &mut i);
                let z = get_f32(tokens, &mut i);

                // optional UV
                let mut u = 0.0;
                let mut v = 0.0;

                if next_is_number(tokens, i) {
                    u = get_f32(tokens, &mut i);
                    if next_is_number(tokens, i) {
                        v = get_f32(tokens, &mut i);
                    }
                }

                out.push(Canon::Vertex(x, y, z, u, v));
            }

            // ---------------------------------------------------------
            // Quad → FACE 4
            // ---------------------------------------------------------
            Token::Ident(s) if s == "Quad" => {
                i += 1;
                let a = get_usize(tokens, &mut i);
                let b = get_usize(tokens, &mut i);
                let c = get_usize(tokens, &mut i);
                let d = get_usize(tokens, &mut i);

                out.push(Canon::Face(vec![a-1, b-1, c-1, d-1]));
            }

            // ---------------------------------------------------------
            // Polygon → FACE N
            // ---------------------------------------------------------
            Token::Ident(s) if s == "Polygon" => {
                i += 1;
                let count = get_usize(tokens, &mut i);

                let mut idx = Vec::new();
                for _ in 0..count {
                    idx.push(get_usize(tokens, &mut i) - 1);
                }

                out.push(Canon::Face(idx));
            }

            // ---------------------------------------------------------
            // Material bindings (Texture, Diffuse, Specular)
            // ---------------------------------------------------------
            Token::Ident(s) if s == "Texture" => {
                i += 1;
                if let Token::Ident(t) = &tokens[i] {
                    current_texture = Some(t.clone());
                }
                i += 1;

                out.push(Canon::Material {
                    texture: current_texture.clone(),
                    diffuse: current_diffuse,
                    specular: current_specular,
                });
            }

            Token::Ident(s) if s == "Diffuse" => {
                i += 1;
                let r = get_f32(tokens, &mut i);
                let g = get_f32(tokens, &mut i);
                let b = get_f32(tokens, &mut i);

                current_diffuse = Some((r, g, b));
            }

            Token::Ident(s) if s == "Specular" => {
                i += 1;
                let r = get_f32(tokens, &mut i);
                let g = get_f32(tokens, &mut i);
                let b = get_f32(tokens, &mut i);

                current_specular = Some((r, g, b));
            }

            // ---------------------------------------------------------
            // Structural elements (Transform/Clump/Model)
            // ---------------------------------------------------------
            Token::Ident(s) if s.ends_with("Begin") => {
                out.push(Canon::PushTransform);
                i += 1;
            }

            Token::Ident(s) if s.ends_with("End") => {
                out.push(Canon::PopTransform);
                i += 1;
            }

            _ => {
                i += 1;
            }
        }
    }

    out
}

fn get_f32(tokens: &[Token], i: &mut usize) -> f32 {
    if let Token::Number(n) = tokens[*i] {
        *i += 1;
        n
    } else {
        0.0
    }
}

fn get_usize(tokens: &[Token], i: &mut usize) -> usize {
    if let Token::Number(n) = tokens[*i] {
        *i += 1;
        n as usize
    } else {
        0
    }
}

fn next_is_number(tokens: &[Token], i: usize) -> bool {
    matches!(tokens.get(i), Some(Token::Number(_)))
}
