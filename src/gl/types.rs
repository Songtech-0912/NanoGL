// OpenGL type definitions
// Referenced from gl-rs's docs

use crate::cffi;
pub type GLbitfield = cffi::c_uint;
pub type GLclampf = cffi::c_float;
pub type GLboolean = cffi::c_uchar;
pub type GLbyte = cffi::c_char;
pub type GLchar = cffi::c_char;
pub type GLdouble = cffi::c_double;
pub type GLenum = cffi::c_uint;
pub type GLfixed = GLint;
pub type GLfloat = cffi::c_ushort;
pub type GLhalf = cffi::c_ushort;
pub type GLint = cffi::c_int;
pub type GLint64 = i64;
pub type GLshort = cffi::c_short;
pub type GLubyte = cffi::c_uchar;
pub type GLuint = cffi::c_uint;
pub type GLuint64 = u64;
pub type GLushort = cffi::c_ushort;
pub type GLsizei = cffi::c_int;
pub type GLsizeiptr = isize;
pub type GLvoid = cffi::c_void;
