use rwx_lexer::Token;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone)]
pub enum Face {
    Quad(Vec<u32>, String),
    Poly(Vec<u32>, String),
}

#[derive(Debug, Clone)]
pub struct Material { pub name: String }

#[derive(Debug, Clone)]
pub struct RwxModel {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub materials: Vec<Material>,
}

pub fn parse(tokens: &[Token]) -> RwxModel {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut materials = Vec::new();
    let mut cur_mat = "NULL".to_string();

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Ident(name) => {
                // texture switching
                if name == "Texture" {
                    if let Token::Ident(tex) = &tokens[i+1] {
                        cur_mat = tex.clone();
                        materials.push(Material { name: cur_mat.clone() });
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }

            Token::VertexExt => {
                let x = get_num(tokens, i+1);
                let y = get_num(tokens, i+2);
                let z = get_num(tokens, i+3);

                // expect UV token
                let u = get_num(tokens, i+5);
                let v = get_num(tokens, i+6);

                vertices.push(Vertex { x, y, z, u, v });
                i += 7;
            }

            Token::Quad => {
                let a = get_num(tokens, i+1) as u32;
                let b = get_num(tokens, i+2) as u32;
                let c = get_num(tokens, i+3) as u32;
                let d = get_num(tokens, i+4) as u32;
                faces.push(Face::Quad(vec![a,b,c,d], cur_mat.clone()));
                i += 5;
            }

            Token::Polygon => {
                let count = get_num(tokens, i+1) as usize;
                let mut idx = Vec::new();
                for k in 0..count {
                    idx.push(get_num(tokens, i+2+k) as u32);
                }
                faces.push(Face::Poly(idx, cur_mat.clone()));
                i += 2 + count;
            }

            _ => i += 1,
        }
    }

    RwxModel { vertices, faces, materials }
}

fn get_num(tokens: &[Token], idx: usize) -> f32 {
    if let Token::Number(n) = tokens[idx] { n } else { 0.0 }
}
