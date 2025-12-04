#[derive(Debug, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
}

#[derive(Debug, Clone)]
pub enum Face {
    Quad(Vec<u32>, String),
    Poly(Vec<u32>, String),
}

#[derive(Debug, Clone)]
pub struct MaterialRef {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Node {
    Vertex(Vertex),
    Face(Face),
    Material(MaterialRef),
    Transform(Vec<f32>),
    Block(Vec<Node>),
}

#[derive(Debug, Clone)]
pub struct RwxModel {
    pub nodes: Vec<Node>,
}
