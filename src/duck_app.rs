use crate::{
    camera::Camera,
    constants::*,
    keyboard::KeyboardState,
    math::affine::transforms,
    mouse::MouseState,
    primitives::vertex::SimpleVertex,
    render::{
        gl_drawable::GlDrawable, gl_mesh::GlMesh, gl_texture::GlTexture, mesh::Mesh,
        shader_manager::ShaderManager, texture::Texture,
    },
    shaders,
    water_texture::WaterTexture,
};
use glow::HasContext;
use glutin::{
    dpi::PhysicalPosition,
    event::{Event, VirtualKeyCode, WindowEvent},
};
use nalgebra::{Matrix4, Vector3};
use std::{path::Path, time::Duration};

pub struct DuckApp<'gl> {
    gl: &'gl glow::Context,
    shader_manager: ShaderManager<'gl>,

    camera: Camera,

    last_mouse_position: Option<PhysicalPosition<f64>>,
    mouse: MouseState,
    keyboard: KeyboardState,

    duck_mesh: GlMesh<'gl>,
    duck_texture: GlTexture<'gl>,
    duck_mtx: Matrix4<f32>,

    water_texture: WaterTexture<'gl>,
    water_mesh: GlMesh<'gl>,
}

impl<'gl> DuckApp<'gl> {
    const CAMERA_ROTATION_SPEED: f32 = 0.5;
    const CAMERA_MOVEMENT_SPEED: f32 = 1.0;

    const WATER_SAMPLES: usize = 256;
    const DEFAULT_WAVE_SPEED: f32 = 1.0;

    pub fn init(gl: &'gl glow::Context) -> Self {
        let duck = Mesh::from_file(Path::new(DUCK_MODEL_PATH));
        let duck_mesh = GlMesh::new(gl, &duck);

        let duck_texture = Texture::from_file(Path::new(&DUCK_TEXTURE_PATH));
        let duck_texture = GlTexture::new(gl, &duck_texture);

        let water_mesh = Mesh::<SimpleVertex>::rect();

        Self::init_gl(gl);

        Self {
            gl,
            shader_manager: shaders::create_shader_manager(gl),

            camera: Camera::new(),

            last_mouse_position: None,
            mouse: MouseState::new(),
            keyboard: KeyboardState::new(),

            duck_mesh,
            duck_texture,
            duck_mtx: transforms::translate(Vector3::new(0.0, 0.0, -3.0))
                * transforms::uniform_scale(0.01),

            water_texture: WaterTexture::new(gl, Self::WATER_SAMPLES, Self::DEFAULT_WAVE_SPEED),
            water_mesh: GlMesh::new(gl, &water_mesh),
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
        self.water_texture.update(delta);
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

        let displacement = displacement * Self::CAMERA_MOVEMENT_SPEED * delta.as_secs_f32();
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
                mouse_delta.0 as f32 * delta.as_secs_f32() * Self::CAMERA_ROTATION_SPEED;

            self.camera.angle_x -=
                mouse_delta.1 as f32 * delta.as_secs_f32() * Self::CAMERA_ROTATION_SPEED;

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
        self.render_water();
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

        self.duck_texture.bind();

        self.duck_mesh.draw();
    }

    fn render_water(&self) {}

    pub fn control_ui(&mut self, ui: &mut imgui::Ui) {
        ui.window("Control")
            .size([400.0, 200.0], imgui::Condition::Once)
            .position([0.0, 0.0], imgui::Condition::Once)
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
