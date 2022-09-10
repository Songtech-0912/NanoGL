use nanogl;
use nanogl::gl::*;
use nanogl::GLWindow;

const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

pub fn main() {
    let window: GLWindow = GLWindow::new(320, 240, "Window 1", 0);
    while window.is_running() {
        window.clear(nanogl::RGBColor(128, 144, 160));
        // OpenGL drawing commands here
        window.update();
    }
    window.cleanup();
}
