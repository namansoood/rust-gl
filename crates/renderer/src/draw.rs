use crate::structs::{Draw, Program};

impl Draw {
    pub fn new(
        shader_program: Program,
        vao: gl::types::GLuint,
        vbo: gl::types::GLuint,
        vertices: Vec<f32>,
    ) -> Self {
        Draw {
            shader_program,
            vao,
            vbo,
            vertices,
        }
    }
}
