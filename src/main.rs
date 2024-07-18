extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

use std::env;

static UW: u32 = 1366;
static UH: u32 = 768;

static IW: i32 = UW as i32;
static IH: i32 = UH as i32;

pub fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let (sdl, window, gl_context, mut imgui, mut imgui_sdl2, renderer, mut event_pump) =
        window::init("Rusty Engine", UW, UH);

    unsafe {
        gl::Viewport(0, 0, IW, IH);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let draw_triangle = triangle::make(
        vec![0.5, -0.5, 0.0, -0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
        vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    );

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, draw_triangle.vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (draw_triangle.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            draw_triangle.vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    unsafe {
        gl::BindVertexArray(draw_triangle.vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, draw_triangle.vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    'main: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) {
                continue;
            }

            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

        let ui = imgui.frame();
        ui.text("Hello World?!");

        unsafe {
            gl::UseProgram(draw_triangle.shader_program.id());
            gl::BindVertexArray(draw_triangle.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        imgui_sdl2.prepare_render(&ui, &window);
        renderer.render(&mut imgui);

        window.gl_swap_window();
    }
}
