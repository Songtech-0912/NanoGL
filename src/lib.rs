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

mod tigr;

use libc;

extern "C" {
    fn tigrWindow(
        w: libc::c_int,
        h: libc::c_int,
        title: *const libc::c_char,
        flags: libc::c_int,
    ) -> *mut Tigr;
    fn tigrFree(bmp: *mut Tigr);
    fn tigrClosed(bmp: *mut Tigr) -> libc::c_int;
    fn tigrUpdate(bmp: *mut Tigr);
    fn tigrClear(bmp: *mut Tigr, color: TPixel);
    fn tigrPrint(
        dest: *mut Tigr,
        font: *mut TigrFont,
        x: libc::c_int,
        y: libc::c_int,
        color: TPixel,
        text: *const libc::c_char,
        _: ...
    );
    static mut tfont: *mut TigrFont;
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
unsafe fn main_0(
    mut _argc: libc::c_int,
    mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut window: *mut Tigr = tigrWindow(
        320 as libc::c_int,
        240 as libc::c_int,
        b"Window1\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    while tigrClosed(window) == 0 {
        tigrClear(
            window,
            tigrRGB(
                0x80 as libc::c_int as libc::c_uchar,
                0x90 as libc::c_int as libc::c_uchar,
                0xa0 as libc::c_int as libc::c_uchar,
            ),
        );
        tigrPrint(
            window,
            tfont,
            120 as libc::c_int,
            110 as libc::c_int,
            tigrRGB(
                0xff as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
            ),
            b"Hello, world #1.\0" as *const u8 as *const libc::c_char,
        );
        tigrUpdate(window);
    }
    tigrFree(window);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
