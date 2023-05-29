use super::{gl_drawable::GlDrawable, mesh::Mesh, opengl};
use crate::{primitives::vertex::Vertex, utils};
use glow::HasContext;
use nalgebra::Point3;

pub struct GlMesh<'gl> {
    vertex_buffer: u32,
    element_buffer: u32,
    element_count: u32,
    vertex_array: u32,
    gl: &'gl glow::Context,
}

impl<'gl> GlMesh<'gl> {
    pub fn new(gl: &'gl glow::Context, mesh: Mesh) -> Self {
        let vertex_buffer = unsafe { gl.create_buffer() }.unwrap();
        let element_buffer = unsafe { gl.create_buffer() }.unwrap();

        let vertex_array = opengl::init_vao(gl, || unsafe {
            let raw_points = utils::slice_as_raw(&mesh.vertices);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, raw_points, glow::STATIC_DRAW);

            let raw_elements = utils::slice_as_raw(&mesh.triangles);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(element_buffer));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, raw_elements, glow::STATIC_DRAW);

            // Positions
            gl.vertex_attrib_pointer_f32(
                0,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);

            // Normals
            gl.vertex_attrib_pointer_f32(
                1,
                3,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                std::mem::size_of::<Point3<f32>>() as i32,
            );
            gl.enable_vertex_attrib_array(0);

            // Texture coords
            gl.vertex_attrib_pointer_f32(
                2,
                2,
                glow::FLOAT,
                false,
                std::mem::size_of::<Vertex>() as i32,
                2 * std::mem::size_of::<Point3<f32>>() as i32,
            );
            gl.enable_vertex_attrib_array(0);
        });

        Self {
            vertex_buffer,
            element_buffer,
            element_count: 3 * mesh.triangles.len() as u32,
            vertex_array,
            gl,
        }
    }
}

impl<'gl> GlDrawable for GlMesh<'gl> {
    fn draw(&self) {
        opengl::with_vao(self.gl, self.vertex_array, || unsafe {
            self.gl.draw_elements(
                glow::TRIANGLES,
                self.element_count as i32,
                glow::UNSIGNED_INT,
                0,
            );
        });
    }
}

impl<'gl> Drop for GlMesh<'gl> {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vertex_array);
            self.gl.delete_buffer(self.vertex_buffer);
            self.gl.delete_buffer(self.element_buffer);
        }
    }
}
