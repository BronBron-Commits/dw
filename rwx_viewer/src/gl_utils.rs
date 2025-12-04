use std::fs;
use std::ptr;
use std::ffi::{CString};
use gl::types::*;

pub unsafe fn load_shader(src: &str, kind: GLenum) -> GLuint {
    let shader = gl::CreateShader(kind);
    let csrc = CString::new(src.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &csrc.as_ptr(), ptr::null());
    gl::CompileShader(shader);
    shader
}

pub unsafe fn load_program(vpath: &str, fpath: &str) -> GLuint {
    let vsrc = fs::read_to_string(vpath).unwrap();
    let fsrc = fs::read_to_string(fpath).unwrap();

    let vs = load_shader(&vsrc, gl::VERTEX_SHADER);
    let fs = load_shader(&fsrc, gl::FRAGMENT_SHADER);

    let prog = gl::CreateProgram();
    gl::AttachShader(prog, vs);
    gl::AttachShader(prog, fs);
    gl::LinkProgram(prog);

    gl::DeleteShader(vs);
    gl::DeleteShader(fs);

    prog
}
