use glow::HasContext;
use nalgebra::{Point3, Vector2, Vector3};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DuckVertex {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
    pub tex: Vector2<f32>,
}

impl DuckVertex {
    pub fn new(position: Point3<f32>, normal: Vector3<f32>, tex: Vector2<f32>) -> Self {
        Self {
            position,
            normal,
            tex,
        }
    }
}

impl Vertex for DuckVertex {
    fn set_vertex_attrib_pointers(gl: &glow::Context) {
        unsafe {
            // Positions
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<DuckVertex>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);

            // Normals
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<DuckVertex>() as i32,
                std::mem::size_of::<Point3<f32>>() as i32,
            );
            gl.enable_vertex_attrib_array(1);

            // Texture coords
            gl.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                std::mem::size_of::<DuckVertex>() as i32,
                std::mem::size_of::<Point3<f32>>() as i32
                    + std::mem::size_of::<Vector3<f32>>() as i32,
            );
            gl.enable_vertex_attrib_array(2);
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SimpleVertex(pub Point3<f32>);

impl SimpleVertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Point3::new(x, y, z))
    }
}

impl Vertex for SimpleVertex {
    fn set_vertex_attrib_pointers(gl: &glow::Context) {
        unsafe {
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<DuckVertex>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);
        }
    }
}

pub trait Vertex {
    fn set_vertex_attrib_pointers(gl: &glow::Context);
}
