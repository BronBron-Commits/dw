// rwx_parser/src/ast.rs

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RwxVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RwxFace {
    pub indices: Vec<usize>,      // vertex indices (1-based in RWX, 0-based here)
    pub texture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RwxMesh {
    pub vertices: Vec<RwxVertex>,
    pub faces: Vec<RwxFace>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RwxPrototype { // Maps to ProtoBegin/ProtoEnd
    pub name: String,
    pub mesh: RwxMesh, // Simplified for this example
    // Add Transform, Surface, etc. fields here later
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RwxClump { // Maps to ClumpBegin/ClumpEnd
    pub children: Vec<RwxObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RwxObject {
    Prototype(RwxPrototype),
    Clump(RwxClump),
    // Other top-level RWX objects (e.g., ModelBegin)
}