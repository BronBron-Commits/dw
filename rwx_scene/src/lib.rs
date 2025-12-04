use serde::{Serialize, Deserialize};
use rwx_parser::{
    RwxObject, RwxVertex, RwxFace, RwxMesh,
    RwxTransform as ParserTransform
};

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

    pub fn apply(&self, v: &RwxVertex) -> RwxVertex {
        let mut x = v.x;
        let mut y = v.y;
        let mut z = v.z;

        // Scale
        if let Some(s) = self.scale {
            x *= s[0];
            y *= s[1];
            z *= s[2];
        }

        // Rotate (angle-axis)
        if let Some(r) = self.rotate {
            let angle = r[0].to_radians();
            let ax = r[1];
            let ay = r[2];
            let az = r[3];

            let len = (ax*ax + ay*ay + az*az).sqrt();
            let (ax, ay, az) = if len != 0.0 {
                (ax/len, ay/len, az/len)
            } else {
                (0.0, 0.0, 0.0)
            };

            let (sin_a, cos_a) = angle.sin_cos();

            let rx = ax*(ax*x + ay*y + az*z)*(1.0 - cos_a)
                + x*cos_a
                + (-az*y + ay*z)*sin_a;

            let ry = ay*(ax*x + ay*y + az*z)*(1.0 - cos_a)
                + y*cos_a
                + (az*x - ax*z)*sin_a;

            let rz = az*(ax*x + ay*y + az*z)*(1.0 - cos_a)
                + z*cos_a
                + (-ay*x + ax*y)*sin_a;

            x = rx;
            y = ry;
            z = rz;
        }

        // Translate
        if let Some(t) = self.translate {
            x += t[0];
            y += t[1];
            z += t[2];
        }

        RwxVertex {
            x, y, z,
            u: v.u,
            v: v.v,
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
        let (vertices, faces) = match &obj.mesh {
            Some(RwxMesh { vertices, faces }) => (vertices.clone(), faces.clone()),
            None => (vec![], vec![]),
        };

        let transform = obj.transform
            .as_ref()
            .map(|t| RwxTransform::from_parser(t))
            .unwrap_or_else(RwxTransform::identity);

        let children = obj.children.iter()
            .map(|c| Self::convert_node(c))
            .collect();

        RwxNode {
            name: obj.name.clone(),
            vertices,
            faces,
            transform,
            children,
        }
    }

    // Flatten scene into world-space mesh list
    pub fn flatten(&self) -> Vec<RwxNode> {
        let mut out = vec![];
        Self::flatten_recursive(&self.root, &RwxTransform::identity(), &mut out);
        out
    }

    fn flatten_recursive(node: &RwxNode, parent_tf: &RwxTransform, out: &mut Vec<RwxNode>) {
        // Combine transforms: parent then current
        let world_tf = Self::combine(parent_tf, &node.transform);

        // Apply transform to vertices
        let world_vertices = node.vertices
            .iter()
            .map(|v| world_tf.apply(v))
            .collect();

        out.push(RwxNode {
            name: node.name.clone(),
            vertices: world_vertices,
            faces: node.faces.clone(),
            transform: world_tf.clone(),
            children: vec![],
        });

        for child in &node.children {
            Self::flatten_recursive(child, &world_tf, out);
        }
    }

    fn combine(a: &RwxTransform, b: &RwxTransform) -> RwxTransform {
        RwxTransform {
            translate: match (a.translate, b.translate) {
                (Some(at), Some(bt)) => Some([at[0] + bt[0], at[1] + bt[1], at[2] + bt[2]]),
                (Some(at), None) => Some(at),
                (None, Some(bt)) => Some(bt),
                _ => None,
            },
            rotate: b.rotate.or(a.rotate),
            scale: match (a.scale, b.scale) {
                (Some(as_), Some(bs_)) =>
                    Some([as_[0]*bs_[0], as_[1]*bs_[1], as_[2]*bs_[2]]),
                (Some(as_), None) => Some(as_),
                (None, Some(bs_)) => Some(bs_),
                _ => None,
            },
        }
    }
}