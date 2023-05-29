use duck::{constants::*, duck_app::DuckApp, window::Window};
use glow::HasContext;
use glutin::{
    event::{Event, WindowEvent},
    platform::run_return::EventLoopExtRunReturn,
};
use std::time::{Duration, Instant};

fn main() {
    let (mut window, mut event_loop, gl) = Window::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut last_frame = Instant::now();
    let mut delta = Duration::new(0, 0);

    let mut duck_app = DuckApp::init(&gl);

    unsafe {
        gl.clear_color(CLEAR_COLOR.r, CLEAR_COLOR.g, CLEAR_COLOR.b, CLEAR_COLOR.a);
    }

    event_loop.run_return(|event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = Instant::now();
            delta = now.duration_since(last_frame);
            window.update_delta_time(delta);
            last_frame = now;
        }
        Event::MainEventsCleared => window.request_redraw(),
        Event::RedrawRequested(_) => {
            duck_app.update(delta);

            duck_app.render();

            window.render(&gl, |ui| duck_app.control_ui(ui));
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = glutin::event_loop::ControlFlow::Exit,
        event => {
            duck_app.handle_event(&event);
            window.handle_event(event, &gl);
        }
    });
}
