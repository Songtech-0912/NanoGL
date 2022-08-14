#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // linux depedencies
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=GLU");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=X11");
}

#[cfg(target_os = "macos")]
fn main() {
    // macos dependencies
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=OpenGL");
    println!("cargo:rustc-link-lib=Cocoa");
}

#[cfg(target_os = "windows")]
fn main() {
    // windows dependencies
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=opengl32");
    println!("cargo:rustc-link-lib=gdi32");
}
