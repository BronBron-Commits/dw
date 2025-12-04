use serde::{Serialize, Deserialize};
use rwx_parser::{RwxObject, RwxVertex, RwxFace, RwxMesh};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwxTransform {
    pub translate: Option<[f32; 3]>,
    pub rotate: Option<[f32; 3]>,
    pub scale: Option<[f32; 3]>,
}

impl RwxTransform {
    pub fn identity() -> Self {
        Self {
            translate: None,
            rotate: None,
            scale: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwxNode {
    pub name: String,
    pub vertices: Vec<RwxVertex>,
    pub faces: Vec<RwxFace>,
    pub transform: RwxTransform,
    pub children: Vec<RwxNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwxScene {
    pub root: RwxNode,
}

impl RwxScene {
    pub fn from_object(obj: RwxObject) -> Self {
        // Extract mesh data safely
        let (vertices, faces) = match &obj.mesh {
            Some(RwxMesh { vertices, faces }) => (vertices.clone(), faces.clone()),
            None => (vec![], vec![]),
        };

        let root_node = RwxNode {
            name: obj.name.clone(),
            vertices,
            faces,
            transform: RwxTransform::identity(),
            children: vec![],
        };

        Self { root: root_node }
    }
}