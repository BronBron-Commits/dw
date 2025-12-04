use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct RwxMesh {
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RwxObject {
    pub name: String,
    pub mesh: Option<RwxMesh>,
    pub children: Vec<RwxObject>,
}
