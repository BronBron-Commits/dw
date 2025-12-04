use crate::ast::{
    RwxObject, RwxMesh, RwxVertex, RwxFace,
    RwxMaterial, RwxTransform
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RwxError {
    #[error("Parse error: {0}")]
    Parse(String),
}

pub struct RwxParser;

impl RwxParser {
    pub fn parse(text: &str) -> Result<RwxObject, RwxError> {
        let mut lines = text.lines().peekable();
        Self::parse_object("root", &mut lines)
    }

    fn parse_object<'a, I>(name: &str, lines: &mut std::iter::Peekable<I>) -> Result<RwxObject, RwxError>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut vertices: Vec<RwxVertex> = vec![];
        let mut faces: Vec<RwxFace> = vec![];
        let mut children: Vec<RwxObject> = vec![];
        let mut transforms: Vec<RwxTransform> = vec![];

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }

            let tokens: Vec<&str> = trimmed.split_whitespace().collect();

            match tokens[0].to_ascii_lowercase().as_str() {

                // -----------------------------------------------------------
                // Vertex x y z
                // -----------------------------------------------------------
                "vertex" => {
                    if tokens.len() < 4 { return Err(err("Vertex requires 3 floats")); }
                    let x = parse_f32(tokens[1])?;
                    let y = parse_f32(tokens[2])?;
                    let z = parse_f32(tokens[3])?;
                    vertices.push(RwxVertex { x, y, z, u: None, v: None });
                }

                // -----------------------------------------------------------
                // Face i j k ...
                // -----------------------------------------------------------
                "face" => {
                    if tokens.len() < 4 { return Err(err("Face requires at least 3 indices")); }
                    let indices = tokens[1..]
                        .iter()
                        .map(|t| parse_u32(t))
                        .collect::<Result<Vec<_>, _>>()?;
                    faces.push(RwxFace { indices });
                }

                // -----------------------------------------------------------
                // ClumpBegin <name>
                // ClumpEnd
                // -----------------------------------------------------------
                "clumpbegin" => {
                    let child_name = if tokens.len() > 1 {
                        tokens[1].to_string()
                    } else {
                        "clump".to_string()
                    };

                    let child = Self::parse_object(&child_name, lines)?;
                    children.push(child);
                }

                "clumpend" => {
                    return Ok(RwxObject {
                        name: name.to_string(),
                        mesh: Some(RwxMesh { vertices, faces }),
                        material: None,
                        transform: transforms.last().cloned(),
                        children,
                    });
                }

                // -----------------------------------------------------------
                // TransformBegin / TransformEnd
                // -----------------------------------------------------------
                "transformbegin" => {
                    transforms.push(RwxTransform {
                        translate: None,
                        rotate: None,
                        scale: None,
                    });
                }

                "transformend" => {
                    // stack pop but no action needed yet
                    transforms.pop();
                }

                // -----------------------------------------------------------
                // ObjectBegin name
                // ObjectEnd
                // -----------------------------------------------------------
                "objectbegin" => {
                    let child_name = if tokens.len() > 1 {
                        tokens[1].to_string()
                    } else {
                        "object".to_string()
                    };
                    let child = Self::parse_object(&child_name, lines)?;
                    children.push(child);
                }

                "objectend" => {
                    return Ok(RwxObject {
                        name: name.to_string(),
                        mesh: Some(RwxMesh { vertices, faces }),
                        material: None,
                        transform: transforms.last().cloned(),
                        children,
                    });
                }

                // -----------------------------------------------------------
                // End of file level
                // -----------------------------------------------------------
                _ => {
                    // Unknown line → ignore for now, RWX has many optional commands.
                }
            }
        }

        Ok(RwxObject {
            name: name.to_string(),
            mesh: Some(RwxMesh { vertices, faces }),
            material: None,
            transform: transforms.last().cloned(),
            children,
        })
    }
}

// -----------------------------------------------------------
// Helpers
// -----------------------------------------------------------

fn parse_f32(s: &str) -> Result<f32, RwxError> {
    s.parse::<f32>().map_err(|_| err(&format!("Invalid float: {}", s)))
}

fn parse_u32(s: &str) -> Result<u32, RwxError> {
    s.parse::<u32>().map_err(|_| err(&format!("Invalid integer: {}", s)))
}

fn err(msg: &str) -> RwxError {
    RwxError::Parse(msg.to_string())
}