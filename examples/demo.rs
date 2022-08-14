use nanogl::tigr::*;

unsafe fn main_0(
    mut _argc: libc::c_int,
    mut _argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let window: *mut Tigr = tigrWindow(
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
