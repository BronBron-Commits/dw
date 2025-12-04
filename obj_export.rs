use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};

// Dummy structs - replace with your actual RWX parser structs
pub struct Material {
    pub name: String,
    pub ka: [f32; 3],
    pub kd: [f32; 3],
    pub ks: [f32; 3],
    pub texture: Option<String>,
}

pub struct Face {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
}

pub struct Mesh {
    pub name: String,
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<Face>,
    pub materials: Vec<Material>,
    pub face_material_indices: Vec<usize>, // one per face
}

pub struct Model {
    pub meshes: Vec<Mesh>,
}

pub fn export_obj(model: &Model, obj_path: &Path) -> std::io::Result<()> {
    let obj_file = BufWriter::new(File::create(obj_path)?);
    let mtl_filename = obj_path.file_stem().unwrap().to_string_lossy().to_string() + ".mtl";
    let mtl_path = obj_path.with_file_name(&mtl_filename);
    let mut obj_file = obj_file;

    // OBJ header
    writeln!(obj_file, "# OBJ generated from RWX")?;
    writeln!(obj_file, "mtllib {}", mtl_filename)?;

    // Track global vertex offset
    let mut vertex_offset = 0;

    // Write vertices
    for mesh in &model.meshes {
        writeln!(obj_file, "o {}", mesh.name)?;
        for vertex in &mesh.vertices {
            writeln!(obj_file, "v {} {} {}", vertex[0], vertex[1], vertex[2])?;
        }

        // Write faces, grouping by material
        for (i, face) in mesh.faces.iter().enumerate() {
            let material_index = mesh.face_material_indices[i];
            let material_name = &mesh.materials[material_index].name;
            writeln!(obj_file, "usemtl {}", material_name)?;
            writeln!(
                obj_file,
                "f {} {} {}",
                face.v1 + 1 + vertex_offset,
                face.v2 + 1 + vertex_offset,
                face.v3 + 1 + vertex_offset
            )?;
        }

        vertex_offset += mesh.vertices.len();
    }

    // Write MTL file
    let mut mtl_file = BufWriter::new(File::create(&mtl_path)?);
    writeln!(mtl_file, "# MTL generated from RWX")?;
    let mut written_materials = std::collections::HashSet::new();
    for mesh in &model.meshes {
        for material in &mesh.materials {
            if written_materials.contains(&material.name) {
                continue; // avoid duplicate
            }
            written_materials.insert(material.name.clone());

            writeln!(mtl_file, "newmtl {}", material.name)?;
            writeln!(mtl_file, "Ka {} {} {}", material.ka[0], material.ka[1], material.ka[2])?;
            writeln!(mtl_file, "Kd {} {} {}", material.kd[0], material.kd[1], material.kd[2])?;
            writeln!(mtl_file, "Ks {} {} {}", material.ks[0], material.ks[1], material.ks[2])?;
            if let Some(ref tex) = material.texture {
                writeln!(mtl_file, "map_Kd {}", tex)?;
            }
            writeln!(mtl_file)?;
        }
    }

    Ok(())
}
