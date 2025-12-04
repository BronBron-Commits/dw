use rwx_parser::Clump; // Note: Updated crate name from rwx_export to rwx_parser
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn write_mtl(clump: &Clump, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;

    // NOTE: This assumes RwxMaterial is in the rwx_parser crate and accessible.
    for (name, mat) in &clump.materials {
        // Material Name
        writeln!(&mut file, "newmtl {}", name)?;

        // --- Standard MTL Properties (R, G, B are 0.0 to 1.0) ---

        // Ambient Color (Ka)
        // Calculated by applying the RWX Ambient intensity multiplier to the color.
        let ka_r = (mat.color.0 as f32 / 255.0) * mat.ambient;
        let ka_g = (mat.color.1 as f32 / 255.0) * mat.ambient;
        let ka_b = (mat.color.2 as f32 / 255.0) * mat.ambient;
        writeln!(&mut file, "Ka {:.6} {:.6} {:.6}", ka_r, ka_g, ka_b)?;


        // Diffuse Color (Kd) - Main visible color
        let kd_r = (mat.color.0 as f32 / 255.0) * mat.diffuse;
        let kd_g = (mat.color.1 as f32 / 255.0) * mat.diffuse;
        let kd_b = (mat.color.2 as f32 / 255.0) * mat.diffuse;
        writeln!(&mut file, "Kd {:.6} {:.6} {:.6}", kd_r, kd_g, kd_b)?;

        // Specular Color (Ks)
        let ks_r = (mat.color.0 as f32 / 255.0) * mat.specular;
        let ks_g = (mat.color.1 as f32 / 255.0) * mat.specular;
        let ks_b = (mat.color.2 as f32 / 255.0) * mat.specular;
        writeln!(&mut file, "Ks {:.6} {:.6} {:.6}", ks_r, ks_g, ks_b)?;
        
        // Dissolve (d) - Opacity
        if mat.opacity < 1.0 {
            writeln!(&mut file, "d {:.6}", mat.opacity)?;
        }

        // Texture Map (map_Kd)
        if mat.texture != "default" {
            // Assume .png for texture file, common for many RWX models
            writeln!(&mut file, "map_Kd {}.png", mat.texture)?; 
        }

        writeln!(&mut file)?; // Blank line for separation
    }

    Ok(())
}

fn write_obj(clump: &Clump, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;

    // 1. Header and MTL Library reference
    writeln!(&mut file, "# OBJ generated from RWX")?;
    writeln!(&mut file, "mtllib output.mtl")?;
    writeln!(&mut file, "o root")?;

    // 2. Vertices (v)
    for v in &clump.vertices {
        writeln!(&mut file, "v {:.6} {:.6} {:.6}", v.position.0, v.position.1, v.position.2)?;
    }

    // 3. Texture Coordinates (vt) - Exporting UVs is crucial for texture mapping!
    for v in &clump.vertices {
        // RWX UVs are typically 0-1, which is standard for OBJ
        writeln!(&mut file, "vt {:.6} {:.6}", v.uv.0, v.uv.1)?;
    }
    
    // 4. Faces (f)
    let mut current_material = String::new();
    
    for face in &clump.faces {
        // Check if material has changed, and if so, write the usemtl command
        if face.material_name != current_material {
            writeln!(&mut file, "usemtl {}", face.material_name)?;
            current_material = face.material_name.clone();
        }

        // Write the face primitive: f V/VT/VN
        let face_indices: Vec<String> = face.vertices.iter()
            // Format: V/VT (Vertex Index / UV Index). RWX uses 1-based indexing for both.
            // Since we write V and VT in the same order, the indices match.
            .map(|&idx| format!("{}/{}", idx, idx)) 
            .collect();

        writeln!(&mut file, "f {}", face_indices.join(" "))?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_rwx_file>", args[0]);
        return Ok(());
    }

    let rwx_path = Path::new(&args[1]);
    let content = match std::fs::read_to_string(rwx_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read RWX file: {}", e);
            std::process::exit(101);
        }
    };

    let clump = match rwx_parser::parse_rwx(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse RWX file: {}", e);
            std::process::exit(102);
        }
    };

    println!("Vertices: {}", clump.vertices.len());
    println!("Faces: {}", clump.faces.len());
    // Print the number of unique materials found
    println!("Unique Materials: {}", clump.materials.len()); 

    let output_dir = Path::new("./exports");
    std::fs::create_dir_all(output_dir)?;

    let obj_path = output_dir.join("output.obj");
    let mtl_path = output_dir.join("output.mtl");

    // Write the MTL file first (must be written before OBJ reads its reference)
    write_mtl(&clump, &mtl_path)?;
    println!("Exported Materials to: {}", mtl_path.display());

    // Write the OBJ file
    write_obj(&clump, &obj_path)?;
    println!("Exported Geometry to: {}", obj_path.display());

    Ok(())
}