use nanogl::ngl::*;
use nanogl::GLWindow;

const VERT_SHADER_SRC: &str = include_str!("shaders/triangle.vert");
const FRAG_SHADER_SRC: &str = include_str!("shaders/triangle.frag");

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

        println!("Got to point 1!");
        glShaderSource(vertex_shader, 1, &VERT_SHADER_SRC, 0);
        println!("Got to point 1!");
        glCompileShader(vertex_shader);
        shader_status(vertex_shader);

        // Create fragement shader
        let fragment_shader: GLuint = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragment_shader, 1, &FRAG_SHADER_SRC, 0);
        glCompileShader(fragment_shader);
        shader_status(fragment_shader);

        // Create program
        let shader_program: GLuint = glCreateProgram();
        glAttachShader(shader_program, vertex_shader);
        glAttachShader(shader_program, fragment_shader);
        glBindFragDataLocation(shader_program, 0, "outColor");
        glLinkProgram(shader_program);
        glUseProgram(shader_program);

        let pos_attrib: GLint = glGetAttribLocation(shader_program, "position");
        glEnableVertexAttribArray(pos_attrib);
        glVertexAttribPointer(pos_attrib, 2, GL_FLOAT, false, 0);

        // Render
        glClearColor(0.25, 0.88, 0.82, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);

        // Draw triangle
        glDrawArrays(GL_TRIANGLES, 0, 3);

        // Show
        window.update();
    }
    window.cleanup();
}
