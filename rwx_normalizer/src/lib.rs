use hashbrown::HashMap;
use rwx_parser::{Vertex, Face};
use rwx_semantics::FlatMesh;

#[derive(Debug, Clone)]
pub struct NormalizedMesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

pub fn normalize(flat: &FlatMesh) -> NormalizedMesh {
    let mut out_vertices = Vec::new();
    let mut remap = HashMap::new();

    for (i, v) in flat.vertices.iter().enumerate() {
        let key = (
            (v.x * 10000.0) as i32,
            (v.y * 10000.0) as i32,
            (v.z * 10000.0) as i32,
            (v.u * 10000.0) as i32,
            (v.v * 10000.0) as i32,
        );

        if let Some(&new_index) = remap.get(&key) {
            remap.insert(key, new_index);
        } else {
            let new_index = out_vertices.len();
            out_vertices.push(v.clone());
            remap.insert(key, new_index);
        }
    }

    let mut out_faces = Vec::new();

    for face in &flat.faces {
        match face {
            Face::Quad(idx, mat) => {
                let a = remap_vertex(idx[0], &flat.vertices, &remap);
                let b = remap_vertex(idx[1], &flat.vertices, &remap);
                let c = remap_vertex(idx[2], &flat.vertices, &remap);
                let d = remap_vertex(idx[3], &flat.vertices, &remap);

                out_faces.push(Face::Poly(vec![a, b, c], mat.clone()));
                out_faces.push(Face::Poly(vec![a, c, d], mat.clone()));
            }

            Face::Poly(indices, mat) => {
                if indices.len() < 3 {
                    continue;
                }
                for t in 1..(indices.len() - 1) {
                    let a = remap_vertex(indices[0], &flat.vertices, &remap);
                    let b = remap_vertex(indices[t], &flat.vertices, &remap);
                    let c = remap_vertex(indices[t + 1], &flat.vertices, &remap);
                    out_faces.push(Face::Poly(vec![a, b, c], mat.clone()));
                }
            }
        }
    }

    NormalizedMesh {
        vertices: out_vertices,
        faces: out_faces,
    }
}

fn remap_vertex(original: u32, verts: &[Vertex], map: &HashMap<(i32,i32,i32,i32,i32), usize>) -> u32 {
    let v = &verts[original as usize];

    let key = (
        (v.x * 10000.0) as i32,
        (v.y * 10000.0) as i32,
        (v.z * 10000.0) as i32,
        (v.u * 10000.0) as i32,
        (v.v * 10000.0) as i32,
    );

    *map.get(&key).unwrap() as u32
}
