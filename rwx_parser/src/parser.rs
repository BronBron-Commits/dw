use crate::normalizer::Canon;
use crate::ast::*;

pub struct RwxParser;

impl RwxParser {
    pub fn from_canon(canon: &[Canon]) -> RwxObject {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for item in canon {
            match item {
                Canon::Vertex(x, y, z, u, v) => {
                    vertices.push(RwxVertex {
                        x: *x,
                        y: *y,
                        z: *z,
                        u: *u,
                        v: *v
                    });
                }

                Canon::Face(idx) => {
                    faces.push(RwxFace {
                        indices: idx.clone(),
                        texture: None,
                    });
                }

                _ => {}
            }
        }

        let mesh = RwxMesh {
            vertices,
            faces,
        };

        RwxObject {
            name: "root".into(),
            mesh: Some(mesh),
            children: Vec::new(),
        }
    }
}
