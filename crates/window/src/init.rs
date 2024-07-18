extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;

use gl;
use imgui_sdl2::ImguiSdl2;
use sdl2::video::{GLProfile, Window};
use sdl2::EventPump;
use sdl2::Sdl;

pub fn init(
    title: &str,
    width: u32,
    height: u32,
) -> (
    Sdl,
    Window,
    sdl2::video::GLContext,
    imgui::Context,
    ImguiSdl2,
    imgui_opengl_renderer::Renderer,
    EventPump,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window(title, width, height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let event_pump = sdl_context.event_pump().unwrap();

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    (
        sdl_context,
        window,
        gl_context,
        imgui,
        imgui_sdl2,
        renderer,
        event_pump,
    )
}
