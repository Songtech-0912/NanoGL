#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub mod tigr;
use libc;
extern "C" {
    pub fn tigrWindow(
            w: libc::c_int,
            h: libc::c_int,
            title: *const libc::c_char,
            flags: libc::c_int,
        ) -> *mut Tigr;
    pub fn tigrFree(bmp: *mut Tigr);
    pub fn tigrClosed(bmp: *mut Tigr) -> libc::c_int;
    pub fn tigrUpdate(bmp: *mut Tigr);
    pub fn tigrClear(bmp: *mut Tigr, color: TPixel);
    pub fn tigrPrint(
            dest: *mut Tigr,
            font: *mut TigrFont,
            x: libc::c_int,
            y: libc::c_int,
            color: TPixel,
            text: *const libc::c_char,
            _: ...
        );
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPixel {
    pub r: libc::c_uchar,
    pub g: libc::c_uchar,
    pub b: libc::c_uchar,
    pub a: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tigr {
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub cx: libc::c_int,
    pub cy: libc::c_int,
    pub cw: libc::c_int,
    pub ch: libc::c_int,
    pub pix: *mut TPixel,
    pub handle: *mut libc::c_void,
    pub blitMode: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TigrGlyph {
    pub code: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TigrFont {
    pub bitmap: *mut Tigr,
    pub numGlyphs: libc::c_int,
    pub glyphs: *mut TigrGlyph,
}
#[inline]
unsafe extern "C" fn tigrRGB(
    mut r: libc::c_uchar,
    mut g: libc::c_uchar,
    mut b: libc::c_uchar,
) -> TPixel {
    let mut p: TPixel = TPixel { r: 0, g: 0, b: 0, a: 0 };
    p.r = r;
    p.g = g;
    p.b = b;
    p.a = 0xff as libc::c_int as libc::c_uchar;
    return p;
}
