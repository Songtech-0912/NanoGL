use nanogl;
use nanogl::tigr::tfont;

pub fn main() {
    let window: nanogl::GLWindow = nanogl::GLWindow::new(320, 240, "Window 1", 0);
    while window.is_running() {
        window.clear(nanogl::RGBColor(128, 144, 160));
        window.print_bitmap(tfont, 120, 110, nanogl::RGBColor(0, 0, 0), "Hello world #1");
        window.update();
    }
    window.cleanup();
}
