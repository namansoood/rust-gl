extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;

use imgui_sdl2::ImguiSdl2;
use sdl2::VideoSubsystem;

pub fn make(
    window: &sdl2::video::Window,
    video_subsystem: VideoSubsystem,
) -> (imgui::Context, ImguiSdl2, imgui_opengl_renderer::Renderer) {
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    (imgui, imgui_sdl2, renderer)
}
