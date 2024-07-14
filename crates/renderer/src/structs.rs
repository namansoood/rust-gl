pub struct Shader {
    pub id: gl::types::GLuint,
}

pub struct Program {
    pub id: gl::types::GLuint,
}

pub struct Draw {
    pub shader_program: Program,
    pub vao: gl::types::GLuint,
    pub vbo: gl::types::GLuint,
    pub vertices: Vec<f32>,
}
