use crate::{
    camera::Camera,
    constants::*,
    keyboard::KeyboardState,
    math::affine::transforms,
    mouse::MouseState,
    render::{gl_drawable::GlDrawable, gl_mesh::GlMesh, mesh::Mesh, shader_manager::ShaderManager},
    shaders,
};
use glow::HasContext;
use glutin::{
    dpi::PhysicalPosition,
    event::{Event, VirtualKeyCode, WindowEvent},
};
use nalgebra::{Matrix4, Vector3};
use std::time::Duration;

pub struct DuckApp<'gl> {
    gl: &'gl glow::Context,
    shader_manager: ShaderManager<'gl>,

    camera: Camera,

    last_mouse_position: Option<PhysicalPosition<f64>>,
    mouse: MouseState,
    keyboard: KeyboardState,

    duck: GlMesh<'gl>,
    duck_mtx: Matrix4<f32>,
}

impl<'gl> DuckApp<'gl> {
    const CAMERA_ROTATION_SPEED: f32 = 0.5;
    const CAMERA_MOVEMENT_SPEED: f32 = 1.0;

    pub fn init(gl: &'gl glow::Context) -> Self {
        let duck = Mesh::from_file(std::path::Path::new(DUCK_MODEL_PATH));
        let duck = GlMesh::new(gl, duck);

        Self::init_gl(gl);

        Self {
            gl,
            shader_manager: shaders::create_shader_manager(gl),

            camera: Camera::new(),

            last_mouse_position: None,
            mouse: MouseState::new(),
            keyboard: KeyboardState::new(),

            duck,
            duck_mtx: transforms::uniform_scale(0.01),
        }
    }

    fn init_gl(gl: &glow::Context) {
        unsafe {
            gl.enable(glow::DEPTH_TEST);

            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.update_position(delta);
        self.update_view(delta);
    }

    fn update_position(&mut self, delta: Duration) {
        let position = self.camera.position();
        let view_dir = self.camera.view_dir();
        let left_dir = self.camera.left_dir();

        let mut displacement = Vector3::zeros();

        if self.keyboard.is_key_down(VirtualKeyCode::W) {
            displacement += view_dir;
        }

        if self.keyboard.is_key_down(VirtualKeyCode::S) {
            displacement -= view_dir;
        }

        if self.keyboard.is_key_down(VirtualKeyCode::A) {
            displacement += left_dir;
        }

        if self.keyboard.is_key_down(VirtualKeyCode::D) {
            displacement -= left_dir;
        }

        let displacement =
            displacement * Self::CAMERA_MOVEMENT_SPEED * delta.as_micros() as f32 / 1000.0 / 1000.0;
        let new_position = position + displacement;

        self.camera.set_position(new_position);
    }

    fn update_view(&mut self, delta: Duration) {
        let mouse_delta = self
            .mouse
            .position()
            .zip(self.last_mouse_position)
            .map_or((0.0, 0.0), |(current, last)| {
                (current.x - last.x, current.y - last.y)
            });

        if self.mouse.is_left_button_down() {
            self.camera.angle_y -=
                mouse_delta.0 as f32 * delta.as_micros() as f32 * Self::CAMERA_ROTATION_SPEED
                    / 1000.0
                    / 1000.0;

            self.camera.angle_x -=
                mouse_delta.1 as f32 * delta.as_micros() as f32 * Self::CAMERA_ROTATION_SPEED
                    / 1000.0
                    / 1000.0;

            self.camera.angle_x = self
                .camera
                .angle_x
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
        }

        self.last_mouse_position = self.mouse.position();
    }

    pub fn render(&self) {
        self.clear();
        self.render_duck();
    }

    fn clear(&self) {
        unsafe {
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn render_duck(&self) {
        let program = self.shader_manager.program("duck");

        program.enable();
        program.uniform_matrix_4_f32("model_transform", &self.duck_mtx);
        program.uniform_matrix_4_f32("view_transform", &self.camera.view_transform());
        program.uniform_matrix_4_f32("projection_transform", &self.camera.projection_transform());

        self.duck.draw();
    }

    pub fn control_ui(&mut self, ui: &mut imgui::Ui) {
        ui.window("Control")
            .size([500.0, 500.0], imgui::Condition::Once)
            .build(|| {
                ui.text("Duck environment control");
                let position = self.camera.position();
                let view = self.camera.view_dir();
                ui.text(format!(
                    "Position: [{}, {}, {}]",
                    position.x, position.y, position.z
                ));
                ui.text(format!("View dir: [{}, {}, {}]", view.x, view.y, view.z));
            });
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
        if let Event::WindowEvent { event, .. } = event {
            self.mouse.handle_window_event(event);
            self.keyboard.handle_window_event(event);

            if let WindowEvent::Resized(resolution) = event {
                self.camera.resolution = *resolution;
            }
        }
    }
}
