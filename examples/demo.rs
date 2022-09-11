use nanogl::ngl::*;
use nanogl::GLWindow;

pub fn main() {
    let window: GLWindow = GLWindow::new(320, 240, "Window 1", 0);
    println!("OpenGL Renderer: {}", glGetString(GL_RENDERER));
    println!("OpenGL Version: {}", glGetString(GL_VERSION));
    println!("GLSL Version: {}", glGetString(GL_SHADING_LANGUAGE_VERSION));
    while window.is_running() {
        window.begin_gl();

        // Very basic OpenGL code
        glClearColor(1.0, 1.0, 1.0, 1.0);
        glClear(GL_COLOR_BUFFER_BIT);

        window.update();
    }
    window.cleanup();
}
