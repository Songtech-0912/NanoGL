use crate::gl;
use crate::gl::glGetShaderInfoLog;
// Export OpenGL constants and types
pub use gl::constants::*;
pub use gl::types::*;
use std::ffi::CStr;
use std::str;

// OpenGL utility functions
pub fn shader_status(shader: GLuint) {
    unsafe {
        let log_len = 4096;
        let status: GLint = 0;
        let shader_type = if GL_FRAGMENT_SHADER != 0 {
            "Fragment shader"
        } else {
            "Vertex shader"
        };
        gl::glGetShaderiv(shader, GL_COMPILE_STATUS, status as *mut i32);
        if status != 1 {
            let mut log: Vec<u8> = Vec::with_capacity(log_len);
            glGetShaderInfoLog(
                shader,
                log_len as i32,
                0 as *mut i32,
                log.as_mut_ptr().cast(),
            );
            log.set_len(log_len.try_into().unwrap());
            eprint!(
                "Shader compile error: {}: {}",
                &shader_type,
                String::from_utf8_lossy(&log)
            );
        }
    }
}

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

pub fn glShaderSource(shader: u32, count: i32, shader_src: &str, len: i32) {
    unsafe {
        gl::glShaderSource(
            shader,
            count,
            shader_src.as_bytes().as_ptr().cast(),
            len as *mut i32,
        )
    }
}

pub fn glCompileShader(shader: GLuint) {
    unsafe {
        gl::glCompileShader(shader);
    }
}

pub fn glCreateProgram() -> GLuint {
    unsafe { gl::glCreateProgram() }
}

pub fn glAttachShader(program: GLuint, shader: GLuint) {
    unsafe {
        gl::glAttachShader(program, shader);
    }
}

pub fn glBindFragDataLocation(program: GLuint, num: u32, name: &str) {
    unsafe {
        gl::glBindFragDataLocation(program, num, name.as_ptr() as *const i8);
    }
}

pub fn glLinkProgram(program: GLuint) {
    unsafe {
        gl::glLinkProgram(program);
    }
}

pub fn glUseProgram(program: GLuint) {
    unsafe {
        gl::glUseProgram(program);
    }
}

pub fn glGetAttribLocation(program: GLuint, attrib: &str) -> GLint {
    unsafe { gl::glGetAttribLocation(program, attrib.as_ptr() as *const i8) }
}

pub fn glEnableVertexAttribArray(attrib: GLint) {
    unsafe {
        gl::glEnableVertexAttribArray(attrib as u32);
    }
}

pub fn glVertexAttribPointer(
    attrib: GLint,
    size: i32,
    ptr_type: GLenum,
    normalized: bool,
    ptr: i32,
) {
    unsafe {
        gl::glVertexAttribPointer(
            attrib as u32,
            size,
            ptr_type,
            normalized as u8,
            ptr as *const GLvoid,
        )
    }
}

pub fn glDrawArrays(mode: GLenum, first: i32, count: i32) {
    unsafe {
        gl::glDrawArrays(mode, first, count);
    }
}
