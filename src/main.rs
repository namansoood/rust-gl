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

    let (sdl, window, video_subsystem, gl_context, mut event_pump) =
        window::init("Rusty Engine", UW, UH);

    let (mut imgui, mut imgui_sdl2, imgui_renderer) = gui::make(&window, video_subsystem);

    let r1: &mut f32 = &mut 1.0;
    let g1: &mut f32 = &mut 0.0;
    let b1: &mut f32 = &mut 0.0;

    let r2: &mut f32 = &mut 0.0;
    let g2: &mut f32 = &mut 1.0;
    let b2: &mut f32 = &mut 0.0;

    let r3: &mut f32 = &mut 0.0;
    let g3: &mut f32 = &mut 0.0;
    let b3: &mut f32 = &mut 1.0;

    unsafe {
        gl::Viewport(0, 0, IW, IH);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
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
        ui.slider("R1", 0.0, 1.0, r1);
        ui.slider("G1", 0.0, 1.0, g1);
        ui.slider("B1", 0.0, 1.0, b1);
        ui.separator();
        ui.slider("R2", 0.0, 1.0, r2);
        ui.slider("G2", 0.0, 1.0, g2);
        ui.slider("B2", 0.0, 1.0, b2);
        ui.separator();
        ui.slider("R3", 0.0, 1.0, r3);
        ui.slider("G3", 0.0, 1.0, g3);
        ui.slider("B3", 0.0, 1.0, b3);

        let draw_triangle = triangle::make(
            vec![0.5, -0.5, 0.0, -0.5, -0.5, 0.0, 0.0, 0.5, 0.0],
            vec![*r1, *g1, *b1, *r2, *g2, *b2, *r3, *g3, *b3],
        );

        draw_triangle.draw();

        imgui_sdl2.prepare_render(&ui, &window);
        imgui_renderer.render(&mut imgui);

        window.gl_swap_window();
    }
}
