use crate::render::{gl_program::GlProgram, shader::Shader, shader_manager::ShaderManager};
use std::path::Path;

const SHADERS_PATH: &str = "shaders/";
const SHADERS_EXTENSION: &str = "glsl";

pub fn create_shader_manager(gl: &glow::Context) -> ShaderManager {
    let cube_vert = shader(gl, "cube_vert", glow::VERTEX_SHADER);
    let duck_vert = shader(gl, "duck_vert", glow::VERTEX_SHADER);
    let water_vert = shader(gl, "water_vert", glow::VERTEX_SHADER);

    let cube_frag = shader(gl, "cube_frag", glow::FRAGMENT_SHADER);
    let duck_frag = shader(gl, "duck_frag", glow::FRAGMENT_SHADER);
    let water_frag = shader(gl, "water_frag", glow::FRAGMENT_SHADER);

    ShaderManager::new(vec![
        (
            "duck",
            GlProgram::with_shaders(gl, &[&duck_vert, &duck_frag]),
        ),
        (
            "water",
            GlProgram::with_shaders(gl, &[&water_vert, &water_frag]),
        ),
        (
            "skybox",
            GlProgram::with_shaders(gl, &[&cube_vert, &cube_frag]),
        ),
    ])
}

fn shader<'gl>(gl: &'gl glow::Context, name: &str, kind: u32) -> Shader<'gl> {
    let mut path = Path::new(SHADERS_PATH).join(name);
    path.set_extension(SHADERS_EXTENSION);
    Shader::from_file(gl, &path, kind)
}
