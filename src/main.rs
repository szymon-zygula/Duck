use duck::{camera::Camera, constants::*, mouse::MouseState, window::Window};
use glow::HasContext;
use glutin::{
    event::{Event, WindowEvent},
    platform::run_return::EventLoopExtRunReturn,
};
use std::time::Instant;

fn render_scene(gl: &glow::Context, camera: &Camera) {
    unsafe {
        gl.clear(glow::COLOR_BUFFER_BIT);
    }
}

fn update_io(window: &Window, mouse: &mut MouseState, camera: &mut Camera) {
    camera.update_from_mouse(mouse, window);
}

fn build_ui(ui: &mut imgui::Ui) {
    ui.window("Control")
        .build(|| ui.text("Duck environment control"));
}

fn main() {
    let (mut window, mut event_loop, gl) = Window::new(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut last_frame = Instant::now();
    let mut mouse = MouseState::new();
    let mut camera = Camera::new();

    unsafe {
        gl.clear_color(CLEAR_COLOR.r, CLEAR_COLOR.g, CLEAR_COLOR.b, CLEAR_COLOR.a);
    }

    event_loop.run_return(|event, _, control_flow| match event {
        Event::NewEvents(_) => {
            let now = Instant::now();
            let duration = now.duration_since(last_frame);
            window.update_delta_time(duration);
            last_frame = now;
        }
        Event::MainEventsCleared => window.request_redraw(),
        Event::RedrawRequested(_) => {
            update_io(&window, &mut mouse, &mut camera);

            render_scene(&gl, &camera);

            window.render(&gl, build_ui);
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = glutin::event_loop::ControlFlow::Exit,
        event => {
            if let Event::WindowEvent { event, .. } = &event {
                mouse.handle_window_event(event);

                if let WindowEvent::Resized(resolution) = event {
                    camera.resolution = *resolution;
                }
            }

            window.handle_event(event, &gl);
        }
    });
}
