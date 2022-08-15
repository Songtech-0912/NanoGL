use nanogl;
use nanogl::tigr::tfont;

pub fn main() {
    let window: nanogl::GLWindow = nanogl::GLWindow::new(320, 240, "Window 1", 0);
    while window.is_running() {
        window.clear(nanogl::RGBColor(128, 144, 160));
        // Currently tfont is a mut static TigrFont, so it isn't thread-safe,
        // this will be fixed eventually
        unsafe {
            window.print_bitmap(
                tfont,
                120,
                110,
                nanogl::RGBColor(255, 255, 255),
                "Hello world!",
            );
        }
        window.update();
    }
    window.cleanup();
}
