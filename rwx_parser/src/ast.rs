use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwxVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwxFace {
    pub indices: Vec<usize>,      // vertex indices
    pub texture: Option<String>,  // texture name if present
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwxMesh {
    pub vertices: Vec<RwxVertex>,
    pub faces: Vec<RwxFace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RwxObject {
    pub name: String,
    pub mesh: Option<RwxMesh>,
    pub children: Vec<RwxObject>,
}
