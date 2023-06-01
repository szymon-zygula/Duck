use crate::{
    camera::Camera,
    constants::*,
    keyboard::KeyboardState,
    math::affine::transforms,
    mouse::MouseState,
    primitives::vertex::SimpleVertex,
    render::{
        gl_drawable::GlDrawable, gl_mesh::GlMesh, gl_program::GlProgram, gl_texture::GlTexture,
        mesh::Mesh, shader_manager::ShaderManager, texture::Texture,
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
use rand::{distributions, rngs::ThreadRng, thread_rng, Rng};
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

    water_mesh: GlMesh<'gl>,
    water_texture: WaterTexture<'gl>,
    water_mtx: Matrix4<f32>,

    light_position: Vector3<f32>,
    light_intensity: f32,

    rng: ThreadRng,
    uniform_dist: distributions::Uniform<f32>,
}

impl<'gl> DuckApp<'gl> {
    const CAMERA_ROTATION_SPEED: f32 = 0.5;
    const CAMERA_MOVEMENT_SPEED: f32 = 5.0;

    const WATER_SAMPLES: usize = 256;
    const DEFAULT_WAVE_SPEED: f32 = 0.75;
    const RAIN_CHANCE: f32 = 1.3e-6;
    const RAIN_DISTURBANCE: f32 = -0.10;

    const DEFAULT_LIGHT_POSITION: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    const DEFAULT_LIGHT_INTENSITY: f32 = 1.0;

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
            duck_mtx: transforms::translate(Vector3::new(0.0, -2.7, 0.0))
                * transforms::uniform_scale(0.01),

            water_mesh: GlMesh::new(gl, &water_mesh),
            water_texture: WaterTexture::new(gl, Self::WATER_SAMPLES, Self::DEFAULT_WAVE_SPEED),
            water_mtx: transforms::translate(Vector3::new(0.0, -2.5, 0.0))
                * transforms::uniform_scale(10.0)
                * transforms::translate(Vector3::new(-0.5, 0.0, -0.5)),

            light_position: Self::DEFAULT_LIGHT_POSITION,
            light_intensity: Self::DEFAULT_LIGHT_INTENSITY,

            rng: thread_rng(),
            uniform_dist: distributions::Uniform::new(0.0, 1.0),
        }
    }

    fn init_gl(gl: &glow::Context) {
        unsafe {
            gl.enable(glow::DEPTH_TEST);

            gl.enable(glow::CULL_FACE);
            gl.cull_face(glow::BACK);
        }
    }

    pub fn update(&mut self, delta: Duration, mouse_captured: bool) {
        self.update_water();
        self.update_position(delta);

        if !mouse_captured {
            self.update_view(delta);
        }
    }

    fn update_water(&mut self) {
        for x in 0..(Self::WATER_SAMPLES as isize) {
            for y in 0..(Self::WATER_SAMPLES as isize) {
                if self.rng.sample(self.uniform_dist) < Self::RAIN_CHANCE {
                    self.water_texture.disturb(x, y, Self::RAIN_DISTURBANCE);
                }
            }
        }

        self.water_texture.update();
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
        program.uniform_3_f32(
            "camera_position",
            self.camera.position.x,
            self.camera.position.y,
            self.camera.position.z,
        );
        self.light_uniforms(program);

        self.duck_texture.bind();
        self.duck_mesh.draw();
    }

    fn render_water(&self) {
        let program = self.shader_manager.program("water");
        program.enable();
        program.uniform_matrix_4_f32("model_transform", &self.water_mtx);
        program.uniform_matrix_4_f32("view_transform", &self.camera.view_transform());
        program.uniform_matrix_4_f32("projection_transform", &self.camera.projection_transform());
        program.uniform_3_f32(
            "camera_position",
            self.camera.position.x,
            self.camera.position.y,
            self.camera.position.z,
        );
        self.light_uniforms(program);

        self.water_texture.normal_texture().bind();
        self.water_mesh.draw();
    }

    fn light_uniforms(&self, program: &GlProgram) {
        program.uniform_3_f32(
            "light_position",
            self.light_position.x,
            self.light_position.y,
            self.light_position.z,
        );
        program.uniform_f32("light_intensity", self.light_intensity);
    }

    pub fn control_ui(&mut self, ui: &imgui::Ui) {
        ui.window("Control")
            .size([400.0, 200.0], imgui::Condition::Once)
            .position([0.0, 0.0], imgui::Condition::Once)
            .build(|| {
                ui.text("Duck environment control");

                self.camera_control(ui);
                self.light_control(ui);
            });
    }

    fn camera_control(&mut self, ui: &imgui::Ui) {
        let position = self.camera.position();
        let view = self.camera.view_dir();

        ui.text(format!(
            "Position: [{}, {}, {}]",
            position.x, position.y, position.z
        ));
        ui.text(format!("View dir: [{}, {}, {}]", view.x, view.y, view.z));
    }

    fn light_control(&mut self, ui: &imgui::Ui) {
        ui.text("Light position");
        ui.slider("x", -5.0, 5.0, &mut self.light_position.x);
        ui.slider("y", -5.0, 5.0, &mut self.light_position.y);
        ui.slider("z", -5.0, 5.0, &mut self.light_position.z);
        ui.slider("Light intensity", 0.0, 10.0, &mut self.light_intensity);
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
