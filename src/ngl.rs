use crate::gl;
pub use gl::constants::*;
pub use gl::types::*;
use std::ffi::CStr;
use std::str;

// Wrappers around OpenGL functions
// (WIP) plan is to wrap all the raw FFI functions to
// have proper error handling and be completely safe
pub fn glGetString(gl_str: GLenum) -> &'static str {
    unsafe {
        let res_ptr = gl::glGetString(gl_str);
        let res_c_str = CStr::from_ptr(res_ptr as *const i8);
        res_c_str.to_str().unwrap()
    }
}
pub fn glClearColor(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::glClearColor(r as GLclampf, g as GLclampf, b as GLclampf, a as GLclampf);
    }
}

pub fn glClear(mask: GLbitfield) {
    unsafe {
        gl::glClear(mask);
    }
}

pub fn glGenVertexArrays(n: i32, arrays: u32) {
    unsafe {
        gl::glGenVertexArrays(n, arrays as *mut u32);
    }
}

pub fn glBindVertexArray(array: u32) {
    unsafe {
        gl::glBindVertexArray(array);
    }
}

pub fn glGenBuffers(n: i32, arrays: u32) {
    unsafe {
        gl::glGenBuffers(n, arrays as *mut u32);
    }
}

pub fn glBindBuffer(target: u32, buffer: u32) {
    unsafe {
        gl::glBindBuffer(target, buffer);
    }
}

pub fn glBufferData(target: u32, size: usize, data: &[f32], usage: u32) {
    unsafe { gl::glBufferData(target, size as isize, data.as_ptr().cast(), usage) }
}

pub fn glCreateShader(shader_type: GLenum) -> GLuint {
    unsafe { gl::glCreateShader(shader_type) }
}

pub fn glShaderSource(shader: u32, count: i32, shader_src: &str) {
    unsafe {
        gl::glShaderSource(
            shader,
            count,
            shader_src.as_bytes().as_ptr().cast(),
            shader_src.len() as *const i32,
        )
    }
}

pub fn glCompileShader(shader: GLuint) {
    unsafe { gl::glCompileShader(shader) }
}
