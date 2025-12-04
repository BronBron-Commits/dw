use std::fs::File;
use std::io::{Write, Result};
use rwx_scene::{RwxScene, RwxNode};

pub struct ObjExporter;

impl ObjExporter {
    pub fn export_scene_to_obj(scene: &RwxScene, path: &str) -> Result<()> {
        let flattened = scene.flatten();
        let mut file = File::create(path)?;

        let mut vertex_offset = 1u32;

        for (i, node) in flattened.iter().enumerate() {
            Self::write_object(&mut file, node, i, &mut vertex_offset)?;
        }

        Ok(())
    }

    fn write_object(
        file: &mut File,
        node: &RwxNode,
        index: usize,
        vertex_offset: &mut u32,
    ) -> Result<()> {
        writeln!(file, "o {}_{}", node.name, index)?;

        // Write vertices
        for v in &node.vertices {
            writeln!(file, "v {} {} {}", v.x, v.y, v.z)?;
        }

        // Write UVs if they exist
        for v in &node.vertices {
            if let (Some(u), Some(vv)) = (v.u, v.v) {
                writeln!(file, "vt {} {}", u, vv)?;
            }
        }

        // Write faces
        for f in &node.faces {
            if f.indices.len() >= 3 {
                write!(file, "f")?;

                for idx in &f.indices {
                    write!(file, " {}", idx + *vertex_offset)?;
                }

                writeln!(file)?;
            }
        }

        *vertex_offset += node.vertices.len() as u32;

        Ok(())
    }
}