use nanogl::ngl::*;
use nanogl::GLWindow;

// const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
// const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

pub fn main() {
    let window: GLWindow = GLWindow::new(320, 240, "Window 1", 0);
    println!("OpenGL Renderer: {}", glGetString(GL_RENDERER));
    println!("OpenGL Version: {}", glGetString(GL_VERSION));
    println!("GLSL Version: {}", glGetString(GL_SHADING_LANGUAGE_VERSION));
    while window.is_running() {
        window.begin_gl();

        let _vertices: [f32; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

        // Create VAO
        let vao = 0;
        glGenVertexArrays(1, vao);
        glBindVertexArray(vao);

        // Create VBO

        window.update();
    }
    window.cleanup();
}
