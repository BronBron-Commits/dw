use serde::{Serialize, Deserialize};
use rwx_parser::{RwxObject, RwxVertex, RwxFace, RwxMesh, RwxTransform as ParserTransform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwxTransform {
    pub translate: Option<[f32; 3]>,
    pub rotate: Option<[f32; 4]>,
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

    pub fn from_parser(src: &ParserTransform) -> Self {
        RwxTransform {
            translate: src.translate,
            rotate: src.rotate,
            scale: src.scale,
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
        let root = Self::convert_node(&obj);
        RwxScene { root }
    }

    fn convert_node(obj: &RwxObject) -> RwxNode {
        // Extract mesh data if present
        let (vertices, faces) = match &obj.mesh {
            Some(RwxMesh { vertices, faces }) => (vertices.clone(), faces.clone()),
            None => (vec![], vec![]),
        };

        // Recursively convert child objects
        let children = obj
            .children
            .iter()
            .map(|child| Self::convert_node(child))
            .collect();

        // Convert transform
        let transform = match &obj.transform {
            Some(t) => RwxTransform::from_parser(t),
            None => RwxTransform::identity(),
        };

        RwxNode {
            name: obj.name.clone(),
            vertices,
            faces,
            transform,
            children,
        }
    }
}