use crate::render::{gl_program::GlProgram, shader::Shader, shader_manager::ShaderManager};
use std::path::Path;

const SHADERS_PATH: &str = "shaders/";
const SHADERS_EXTENSION: &str = "glsl";

pub fn create_shader_manager(gl: &glow::Context) -> ShaderManager {
    let duck_vert = shader(gl, "duck_vert", glow::VERTEX_SHADER);

    let duck_frag = shader(gl, "duck_frag", glow::FRAGMENT_SHADER);

    ShaderManager::new(vec![(
        "duck",
        GlProgram::with_shaders(gl, &[&duck_vert, &duck_frag]),
    )])
}

fn shader<'gl>(gl: &'gl glow::Context, name: &str, kind: u32) -> Shader<'gl> {
    let mut path = Path::new(SHADERS_PATH).join(name);
    path.set_extension(SHADERS_EXTENSION);
    Shader::from_file(gl, &path, kind)
}