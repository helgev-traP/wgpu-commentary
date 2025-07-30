pub mod app;
pub mod gpu;

fn main() {
    let mut app = app::App::new();

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    let _ = event_loop.run_app(&mut app);
}
