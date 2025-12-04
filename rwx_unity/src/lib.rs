use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnityPackage {
    pub name: String,
    pub obj: String,
    pub mtl: String,
    pub textures: Vec<String>,
}

pub fn package_unity(
    base_name: &str,
    obj_path: &str,
    mtl_path: &str,
    texture_dir: &str,
    out_dir: &str
) {
    fs::create_dir_all(out_dir).unwrap();

    let obj_target = format!("{}/{}.obj", out_dir, base_name);
    let mtl_target = format!("{}/{}.mtl", out_dir, base_name);

    fs::copy(obj_path, &obj_target).unwrap();
    fs::copy(mtl_path, &mtl_target).unwrap();

    let mut tex_list = Vec::new();

    if Path::new(texture_dir).exists() {
        for entry in fs::read_dir(texture_dir).unwrap() {
            let e = entry.unwrap();
            let p = e.path();
            if let Some(ext) = p.extension() {
                if ext == "png" || ext == "jpg" {
                    let fname = p.file_name().unwrap().to_string_lossy().to_string();
                    let target = format!("{}/{}", out_dir, fname);
                    fs::copy(&p, &target).unwrap();
                    tex_list.push(fname);
                }
            }
        }
    }

    let pack = UnityPackage {
        name: base_name.to_string(),
        obj: format!("{}.obj", base_name),
        mtl: format!("{}.mtl", base_name),
        textures: tex_list,
    };

    let json_path = format!("{}/{}.unitypack.json", out_dir, base_name);
    let mut jf = File::create(json_path).unwrap();
    jf.write_all(serde_json::to_string_pretty(&pack).unwrap().as_bytes())
        .unwrap();
}
