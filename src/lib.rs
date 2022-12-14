#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clashing_extern_declarations)]
#![feature(c_variadic)]
#![feature(extern_types)]

// Don't expose unsafe raw FFI functions/types
// in the public-facing API (no one should be
// using them anyway)
mod cffi;
mod gl;
pub mod ngl;
pub mod tigr;
pub use gl::types::*;
use std::ffi::CString;

pub struct RGBColor(pub i32, pub i32, pub i32);

pub struct GLWindow {
    width: i32,
    height: i32,
    title: String,
    flag: i32,
    window_ptr: *mut tigr::Tigr,
}

impl GLWindow {
    pub fn new(width: i32, height: i32, title: &str, flag: i32) -> GLWindow {
        // To prevent memory errors we have to make a dedicated cstring
        // to pass to tigrWindow
        let title_str = CString::new(title).unwrap();
        let window_ptr: *mut tigr::Tigr = unsafe {
            tigr::tigrWindow(
                width as cffi::c_int,
                height as cffi::c_int,
                title_str.as_ptr(),
                flag as cffi::c_int,
            )
        };
        GLWindow {
            width: width,
            height: height,
            title: String::from(title),
            flag: flag,
            window_ptr: window_ptr,
        }
    }

    pub fn clear(&self, color: RGBColor) {
        unsafe {
            tigr::tigrClear(self.window_ptr, rgb(color.0, color.1, color.2));
        }
    }

    pub fn update(&self) {
        unsafe {
            tigr::tigrUpdate(self.window_ptr);
        }
    }

    pub fn is_running(&self) -> bool {
        unsafe { tigr::tigrClosed(self.window_ptr) as i32 == 0 }
    }

    pub fn print_bitmap(
        &self,
        font: *mut tigr::TigrFont,
        x: i32,
        y: i32,
        color: RGBColor,
        text: &str,
    ) {
        unsafe {
            tigr::tigrPrint(
                self.window_ptr,
                font,
                x as cffi::c_int,
                y as cffi::c_int,
                rgb(color.0, color.1, color.2),
                text.as_bytes().as_ptr() as *const cffi::c_char,
            );
        }
    }

    pub fn begin_gl(&self) {
        unsafe {
            tigr::tigrBeginOpenGL(self.window_ptr);
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            tigr::tigrFree(self.window_ptr);
        }
    }
}

pub fn rgb(r: i32, g: i32, b: i32) -> tigr::TPixel {
    unsafe {
        tigr::tigrRGB(
            r as cffi::c_int as cffi::c_uchar,
            g as cffi::c_int as cffi::c_uchar,
            b as cffi::c_int as cffi::c_uchar,
        )
    }
}
