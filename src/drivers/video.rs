use sdl2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf;
use sdl2::video::Window;

pub static FRAMERATE: u32 = 1;

pub struct VideoContext {
  pub sdl_context: sdl2::Sdl,
  pub canvas: Canvas<Window>,
  pub ttf_context: ttf::Sdl2TtfContext,
}

pub fn init(screen_width: u32, screen_height: u32) -> Result<VideoContext, String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;

  // ttf context

  let ttf_context = ttf::init().map_err(|e| e.to_string())?;

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

  let result = VideoContext {
    sdl_context,
    canvas,
    ttf_context,
  };

  Ok(result)
}

pub fn fps() -> u32 {
  return FRAMERATE;
}
