use rwx_parser::{RwxModel, Node, Vertex, Face};

#[derive(Debug, Clone)]
pub struct FlatMesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

pub fn flatten(model: &RwxModel) -> FlatMesh {
    let mut out_vertices = Vec::new();
    let mut out_faces = Vec::new();

    flatten_nodes(&model.nodes, &mut out_vertices, &mut out_faces);

    FlatMesh {
        vertices: out_vertices,
        faces: out_faces,
    }
}

fn flatten_nodes(nodes: &[Node], verts: &mut Vec<Vertex>, faces: &mut Vec<Face>) {
    for n in nodes {
        match n {
            Node::Vertex(v) => {
                verts.push(v.clone());
            }

            Node::Face(f) => {
                faces.push(f.clone());
            }

            Node::Block(b) => {
                flatten_nodes(b, verts, faces);
            }

            Node::Transform(_) => {
                continue;
            }

            Node::Material(_) => {
                continue;
            }
        }
    }
}
