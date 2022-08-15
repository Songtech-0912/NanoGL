use nanogl;
use nanogl::gl::*;

const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vertex.glsl");
const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.fragment.glsl");

pub fn main() {
    let window: nanogl::GLWindow = nanogl::GLWindow::new(320, 240, "Window 1", 0);
    while window.is_running() {
        window.clear(nanogl::RGBColor(128, 144, 160));
        // OpenGL drawing commands here
        let vertices: [f32; 6] = [0.5, 1.0, 0.0, 0.0, 1.0, 0.0];
        let vertex_shader = nanogl::create_vertex_shader();

        window.update();
    }
    window.cleanup();
}
