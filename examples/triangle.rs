use nanogl::ngl::*;
use nanogl::GLWindow;

const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
// const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

pub fn main() {
    let window: GLWindow = GLWindow::new(320, 240, "Window 1", 0);
    println!("OpenGL Renderer: {}", glGetString(GL_RENDERER));
    println!("OpenGL Version: {}", glGetString(GL_VERSION));
    println!("GLSL Version: {}", glGetString(GL_SHADING_LANGUAGE_VERSION));
    while window.is_running() {
        window.begin_gl();

        let vertices: [f32; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

        // Create VAO
        let vao = 0;
        glGenVertexArrays(1, vao);
        glBindVertexArray(vao);

        // Create VBO
        let vbo = 0;
        glGenBuffers(1, vbo);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        glBufferData(GL_ARRAY_BUFFER, vertices.len(), &vertices, GL_STATIC_DRAW);

        // Create vertex shader
        let vertex_shader: GLuint = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertex_shader, 1, &VERT_SHADER_SRC);
        glCompileShader(vertex_shader);

        window.update();
    }
    window.cleanup();
}
