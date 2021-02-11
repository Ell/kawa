mod gui;
mod renderer;
mod window;

fn main() {
    unsafe {
        let window = window::Window::create("kawa").unwrap();

        window.run_event_loop();
    }
}
