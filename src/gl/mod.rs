pub mod constants;
pub mod types;

use types::*;

// OpenGL function definitions for FFI
extern "C" {
    pub fn glGetString(name: GLenum) -> *const GLubyte;
    pub fn glClear(mask: GLbitfield);
    pub fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    pub fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
    pub fn glBindVertexArray(array: GLuint);
    pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
    pub fn glBindBuffer(target: GLenum, buffer: GLuint);
    pub fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum);
    pub fn glCreateShader(shader_type: GLenum);
    pub fn glShaderSource(shader: GLuint, count: GLsizei, str: *const GLchar, len: *const GLint);
    pub fn glCompileShader(shader: GLuint);
    pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
    pub fn glGetShaderInfoLog(
        shader: GLuint,
        max_len: GLsizei,
        len: *mut GLsizei,
        info_log: *mut GLchar,
    );
    pub fn glCreateProgram();
    pub fn glAttachShader(program: GLuint, shader: GLuint);
    pub fn glBindFragDataLocation(
        program: GLuint,
        color_num: GLuint,
        index: GLuint,
        name: *const GLchar,
    );
    pub fn glLinkProgram(program: GLuint);
    pub fn glUseProgram(program: GLuint);
    pub fn glGetAttribLocation(program: GLuint, name: *const GLchar);
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
}
