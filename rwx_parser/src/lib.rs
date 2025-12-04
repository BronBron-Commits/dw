use std::collections::HashMap;
use float_cmp::approx_eq;

// --- Data Structures ---

#[derive(Debug, Default, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub uv: (f32, f32),
}

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: Vec<usize>, // Vertex indices (1-based from RWX parsing)
    pub material_name: String, // Unique name for the material properties assigned to this face
}

#[derive(Debug, Clone, PartialEq)]
pub struct RwxMaterial {
    pub color: (u8, u8, u8), // RGB Color (0-255) from Color command
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub opacity: f32,
    pub texture: String, // Texture name from Texture command (e.g., "couch2a")
}

#[derive(Debug, Default)]
pub struct Clump {
    pub vertices: Vec<Vertex>,
    // HashMap key is the unique material name (e.g., "mat_1")
    pub materials: HashMap<String, RwxMaterial>, 
    pub faces: Vec<Face>,
}

// --- Parsing Logic ---

pub fn parse_rwx(content: &str) -> Result<Clump, String> {
    let mut clump = Clump::default();
    let lines = content.lines().collect::<Vec<_>>();
    let mut i = 0;

    // Default Material state
    let mut current_color = (0, 0, 0);
    let mut current_ambient = 0.5;
    let mut current_diffuse = 0.5;
    let mut current_specular = 0.5;
    let mut current_opacity = 1.0;
    let mut current_texture = "default".to_string(); 

    let mut material_counter = 1;

    while i < lines.len() {
        let line = lines[i].trim();
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() || line.starts_with("//") {
            i += 1;
            continue;
        }

        match parts[0] {
            "ClumpBegin" => {}

            // **FIXED VERTEX PARSING LOGIC**
            // Match on BOTH VertexExt and Vertex for maximum compatibility
            "VertexExt" | "Vertex" => { 
                // A vertex must have at least X, Y, Z (3 values + keyword = 4 total parts)
                if parts.len() >= 4 {
                    // Always attempt to parse the position (X, Y, Z)
                    if let (Ok(x), Ok(y), Ok(z)) = (
                        parts[1].parse::<f32>(), parts[2].parse::<f32>(), parts[3].parse::<f32>(),
                    ) {
                        let mut u = 0.0;
                        let mut v = 0.0;
                        
                        // Only check for UVs (U, V) if the command is VertexExt AND the parts are available
                        if parts[0] == "VertexExt" && parts.len() >= 6 {
                            if let (Ok(parsed_u), Ok(parsed_v)) = (
                                parts[4].parse::<f32>(), parts[5].parse::<f32>(),
                            ) {
                                u = parsed_u;
                                v = parsed_v;
                            }
                        }

                        // Push the new Vertex (with UVs or default 0.0, 0.0)
                        clump.vertices.push(Vertex {
                            position: (x, y, z),
                            uv: (u, v),
                        });
                    }
                }
            }
            
            // --- Material Commands ---
            "Color" => {
                if parts.len() >= 4 {
                    let r = (parts[1].parse::<f32>().unwrap_or(0.0) * 255.0).round() as u8;
                    let g = (parts[2].parse::<f32>().unwrap_or(0.0) * 255.0).round() as u8;
                    let b = (parts[3].parse::<f32>().unwrap_or(0.0) * 255.0).round() as u8;
                    
                    current_color = (r, g, b);
                }
            }
            "Ambient" => {
                if parts.len() >= 2 {
                    current_ambient = parts[1].parse::<f32>().unwrap_or(0.5);
                }
            }
            "Diffuse" => {
                if parts.len() >= 2 {
                    current_diffuse = parts[1].parse::<f32>().unwrap_or(0.5);
                }
            }
            "Specular" => {
                if parts.len() >= 2 {
                    current_specular = parts[1].parse::<f32>().unwrap_or(0.5);
                }
            }
            "Opacity" => {
                if parts.len() >= 2 {
                    current_opacity = parts[1].parse::<f32>().unwrap_or(1.0);
                }
            }
            "Texture" => {
                if parts.len() >= 2 {
                    current_texture = parts[1].to_string();
                }
            }
            // --- Face Primitives ---
            "Quad" | "Polygon" => {
                let vertex_indices: Result<Vec<usize>, _> = parts.iter().skip(1).map(|&s| s.parse::<usize>()).collect();
                
                if let Ok(vertices) = vertex_indices {
                    // 1. Create the current material definition
                    let mat_def = RwxMaterial {
                        color: current_color,
                        ambient: current_ambient,
                        diffuse: current_diffuse,
                        specular: current_specular,
                        opacity: current_opacity,
                        texture: current_texture.clone(),
                    };

                    // Custom loop to find unique material
                    let mut found_name = None;
                    for (name, existing_mat) in &clump.materials {
                        let props_match = existing_mat.color == mat_def.color
                            && existing_mat.texture == mat_def.texture
                            // Use approx_eq! for reliable float comparison
                            && approx_eq!(f32, existing_mat.ambient, mat_def.ambient, epsilon = 0.0001)
                            && approx_eq!(f32, existing_mat.diffuse, mat_def.diffuse, epsilon = 0.0001)
                            && approx_eq!(f32, existing_mat.specular, mat_def.specular, epsilon = 0.0001)
                            && approx_eq!(f32, existing_mat.opacity, mat_def.opacity, epsilon = 0.0001);

                        if props_match {
                            found_name = Some(name.clone());
                            break;
                        }
                    }
                    
                    let material_name = match found_name {
                        Some(name) => name,
                        None => {
                            // If new, generate a unique name and insert it
                            let new_name = format!("mat_{}", material_counter);
                            clump.materials.insert(new_name.clone(), mat_def);
                            material_counter += 1;
                            new_name
                        }
                    };

                    // 4. Create the face and assign the material name
                    clump.faces.push(Face {
                        vertices,
                        material_name,
                    });
                }
            }
            _ => {}
        }
        i += 1;
    }

    Ok(clump)
}