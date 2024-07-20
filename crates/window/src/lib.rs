use gl;
use sdl2::video::{self, GLProfile, Window};
use sdl2::Sdl;
use sdl2::{EventPump, VideoSubsystem};

pub fn init(
    title: &str,
    width: u32,
    height: u32,
) -> (
    Sdl,
    Window,
    VideoSubsystem,
    sdl2::video::GLContext,
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

    (sdl_context, window, video_subsystem, gl_context, event_pump)
}
