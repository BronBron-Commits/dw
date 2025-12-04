use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RwxVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: Option<f32>,
    pub v: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RwxFace {
    pub indices: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RwxMesh {
    pub vertices: Vec<RwxVertex>,
    pub faces: Vec<RwxFace>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RwxMaterial {
    pub texture: Option<String>,
    pub opacity: Option<f32>,
    pub ambient: Option<[f32; 3]>,
    pub diffuse: Option<[f32; 3]>,
    pub specular: Option<[f32; 3]>,
    pub emissive: Option<[f32; 3]>,
    pub shininess: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RwxTransform {
    pub translate: Option<[f32; 3]>,
    pub rotate: Option<[f32; 4]>,   // angle + axis
    pub scale: Option<[f32; 3]>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RwxObject {
    pub name: String,
    pub mesh: Option<RwxMesh>,
    pub material: Option<RwxMaterial>,
    pub transform: Option<RwxTransform>,
    pub children: Vec<RwxObject>,
}