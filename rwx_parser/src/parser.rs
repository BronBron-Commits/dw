use crate::ast::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RwxError {
    #[error("Invalid RWX syntax: {0}")]
    Syntax(String),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct RwxParser;

impl RwxParser {
    pub fn parse_str(input: &str) -> Result<RwxObject, RwxError> {
        let mut root = RwxObject {
            name: "root".into(),
            mesh: None,
            children: vec![],
        };

        let mut vertices = vec![];
        let mut triangles = vec![];

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }

            let parts: Vec<_> = line.split_whitespace().collect();
            let cmd = parts[0].to_lowercase();

            match cmd.as_str() {
                "vertex" if parts.len() == 4 => {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    vertices.push([x, y, z]);
                }

                "triangle" if parts.len() == 4 => {
                    let a: u32 = parts[1].parse().unwrap_or(0);
                    let b: u32 = parts[2].parse().unwrap_or(0);
                    let c: u32 = parts[3].parse().unwrap_or(0);
                    triangles.push([a, b, c]);
                }

                _ => {}
            }
        }

        root.mesh = Some(RwxMesh { vertices, triangles });
        Ok(root)
    }
}
