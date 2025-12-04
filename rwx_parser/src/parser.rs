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
        let mut material = RwxMaterial {
            texture: None,
            opacity: None,
            ambient: None,
            diffuse: None,
            specular: None,
            emissive: None,
            shininess: None,
        };

        let mut transform_stack: Vec<RwxTransform> = vec![];

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }

            let tokens: Vec<&str> = trimmed.split_whitespace().collect();
            let cmd = tokens[0].to_ascii_lowercase();

            match cmd.as_str() {

                // VERTEX x y z [u v]
                "vertex" => {
                    if tokens.len() < 4 {
                        return Err(err("Vertex requires x y z"));
                    }

                    let x = parse_f32(tokens[1])?;
                    let y = parse_f32(tokens[2])?;
                    let z = parse_f32(tokens[3])?;

                    let (u, v) = if tokens.len() >= 6 {
                        (Some(parse_f32(tokens[4])?), Some(parse_f32(tokens[5])?))
                    } else {
                        (None, None)
                    };

                    vertices.push(RwxVertex { x, y, z, u, v });
                }

                // FACE i j k [...]
                "face" => {
                    if tokens.len() < 4 {
                        return Err(err("Face requires at least 3 indices"));
                    }

                    let indices = tokens[1..]
                        .iter()
                        .map(|t| parse_u32(t))
                        .collect::<Result<Vec<u32>, _>>()?;

                    faces.push(RwxFace { indices });
                }

                // SURFACE MATERIAL PROPERTIES
                "texture" => {
                    if tokens.len() >= 2 {
                        material.texture = Some(tokens[1].to_string());
                    }
                }

                "opacity" => {
                    if tokens.len() >= 2 {
                        material.opacity = Some(parse_f32(tokens[1])?);
                    }
                }

                "ambient" => {
                    if tokens.len() == 4 {
                        material.ambient = Some([
                            parse_f32(tokens[1])?,
                            parse_f32(tokens[2])?,
                            parse_f32(tokens[3])?,
                        ]);
                    }
                }

                "diffuse" => {
                    if tokens.len() == 4 {
                        material.diffuse = Some([
                            parse_f32(tokens[1])?,
                            parse_f32(tokens[2])?,
                            parse_f32(tokens[3])?,
                        ]);
                    }
                }

                "specular" => {
                    if tokens.len() == 4 {
                        material.specular = Some([
                            parse_f32(tokens[1])?,
                            parse_f32(tokens[2])?,
                            parse_f32(tokens[3])?,
                        ]);
                    }
                }

                "emissive" => {
                    if tokens.len() == 4 {
                        material.emissive = Some([
                            parse_f32(tokens[1])?,
                            parse_f32(tokens[2])?,
                            parse_f32(tokens[3])?,
                        ]);
                    }
                }

                "shininess" => {
                    if tokens.len() >= 2 {
                        material.shininess = Some(parse_f32(tokens[1])?);
                    }
                }

                // TRANSFORMS
                "translate" => {
                    if tokens.len() == 4 {
                        transform_stack.push(RwxTransform {
                            translate: Some([
                                parse_f32(tokens[1])?,
                                parse_f32(tokens[2])?,
                                parse_f32(tokens[3])?,
                            ]),
                            rotate: None,
                            scale: None,
                        });
                    }
                }

                "scale" => {
                    if tokens.len() == 4 {
                        transform_stack.push(RwxTransform {
                            scale: Some([
                                parse_f32(tokens[1])?,
                                parse_f32(tokens[2])?,
                                parse_f32(tokens[3])?,
                            ]),
                            translate: None,
                            rotate: None,
                        });
                    }
                }

                "rotate" => {
                    if tokens.len() == 5 {
                        transform_stack.push(RwxTransform {
                            rotate: Some([
                                parse_f32(tokens[1])?, // angle
                                parse_f32(tokens[2])?, // x axis
                                parse_f32(tokens[3])?, // y axis
                                parse_f32(tokens[4])?, // z axis
                            ]),
                            translate: None,
                            scale: None,
                        });
                    }
                }

                // OBJECT/CLUMP BEGIN-END NESTING
                "clumpbegin" | "objectbegin" => {
                    let child_name = if tokens.len() >= 2 {
                        tokens[1].to_string()
                    } else {
                        "child".into()
                    };

                    let child = Self::parse_object(&child_name, lines)?;
                    children.push(child);
                }

                "clumpend" | "objectend" => {
                    return Ok(RwxObject {
                        name: name.into(),
                        mesh: Some(RwxMesh { vertices, faces }),
                        material: Some(material),
                        transform: transform_stack.last().cloned(),
                        children,
                    });
                }

                _ => {
                    // Ignore unknown commands for now
                }
            }
        }

        Ok(RwxObject {
            name: name.into(),
            mesh: Some(RwxMesh { vertices, faces }),
            material: Some(material),
            transform: transform_stack.last().cloned(),
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