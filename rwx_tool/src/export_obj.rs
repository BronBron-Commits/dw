use std::fs::File;
use std::io::Write;

use rwx_parser::{RwxModel, Face};

pub fn export_obj(model: &RwxModel, out_path: &str) -> std::io::Result<()> {
    let obj_path = format!("{out_path}.obj");
    let mtl_path = format!("{out_path}.mtl");

    let mut obj = File::create(&obj_path)?;
    let mut mtl = File::create(&mtl_path)?;

    // reference the MTL file
    writeln!(obj, "mtllib {}.mtl", out_path)?;

    // write every material
    for mat in &model.materials {
        writeln!(mtl, "newmtl {}", mat.name)?;
        writeln!(mtl, "Kd 0.8 0.8 0.8")?;
        writeln!(mtl, "Ks 0.0 0.0 0.0")?;
        writeln!(mtl, "d 1.0")?;
        writeln!(mtl)?;
    }

    // write vertices
    for v in &model.vertices {
        writeln!(obj, "v {} {} {}", v.x, v.y, v.z)?;
    }

    // write UVs
    for v in &model.vertices {
        writeln!(obj, "vt {} {}", v.u, 1.0 - v.v)?;
    }

    let mut last_material = String::new();

    // faces
    for face in &model.faces {
        match face {
            Face::Quad(idx, mat) => {
                if last_material != *mat {
                    writeln!(obj, "usemtl {}", mat)?;
                    last_material = mat.clone();
                }

                // OBJ uses 1-indexed indexing
                writeln!(
                    obj,
                    "f {0}/{0} {1}/{1} {2}/{2} {3}/{3}",
                    idx[0], idx[1], idx[2], idx[3]
                )?;
            }

            Face::Poly(idx_list, mat) => {
                if last_material != *mat {
                    writeln!(obj, "usemtl {}", mat)?;
                    last_material = mat.clone();
                }

                write!(obj, "f")?;
                for ix in idx_list {
                    write!(obj, " {0}/{0}", ix)?;
                }
                writeln!(obj)?;
            }
        }
    }

    Ok(())
}
