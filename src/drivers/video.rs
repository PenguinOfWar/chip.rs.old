use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf;
use sdl2::video::Window;

pub static FRAMERATE: u32 = 60;

pub struct Video {
  pub sdl_context: sdl2::Sdl,
  pub canvas: Canvas<Window>,
  pub ttf_context: ttf::Sdl2TtfContext,
}

pub trait VideoContext {
  fn new(screen_width: u32, screen_height: u32) -> Self;
  fn get(self: Self) -> Self;
}

impl VideoContext for Video {
  fn new(screen_width: u32, screen_height: u32) -> Video {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // ttf font context

    let ttf_context = ttf::init().unwrap();

    let window = video_subsystem
      .window("rust-sdl2 demo", screen_width, screen_height)
      .position_centered()
      .opengl()
      .build()
      .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // fills the canvas with the color we set in `set_draw_color`.
    canvas.clear();

    Video {
      sdl_context,
      canvas,
      ttf_context,
    }
  }
  fn get(self: Video) -> Video {
    return self;
  }
}
