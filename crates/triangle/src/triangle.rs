extern crate gl;

use renderer::structs::{Draw, Program, Shader};
use std::ffi::CString;
use utils::vector;

pub fn get_vert_shader() -> Shader {
    return Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
        .unwrap();
}

pub fn get_frag_shader() -> Shader {
    return Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
        .unwrap();
}

pub fn get_shader_program() -> Program {
    return Program::from_shaders(&[get_vert_shader(), get_frag_shader()]).unwrap();
}

pub fn make(positions: Vec<f32>, colors: Vec<f32>) -> Draw {
    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;

    let shader_program = get_shader_program();

    let vertices: Vec<f32> = vector::combine_positions_and_colors(positions, colors);

    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    return Draw::new(shader_program, vao, vbo, vertices);
}
