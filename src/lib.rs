// with thanks to:
// https://sunjay.dev/learn-game-dev/opening-a-window.html
// https://docs.rs/sdl2/0.30.0/sdl2/render/struct.Canvas.html
// https://docs.rs/sdl2/0.5.0/src/sdl2/.cargo/registry/src/github.com-1ecc6299db9ec823/sdl2-0.5.0/src/sdl2/rect.rs.html#83-237
// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/ttf-demo.rs
// https://github.com/redox-os/rusttype/blob/master/dev/examples/gpu_cache.rs

pub mod drivers;
pub mod logger;

use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use sdl2;
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf;
use sdl2::video::Window;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;
static PADDING: u32 = 64;

// macro rules seem cool
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub fn main() -> Result<(), String> {
    logger::main().expect("panic bootstrapping");

    // configure video

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // ttf context

    let ttf_context = ttf::init().map_err(|e| e.to_string())?;

    // Load a font
    let font_path: &Path = Path::new("./src/assets/fonts/MobileFont.ttf");
    log::info!("path: {:?}", font_path);

    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);

    let window = video_subsystem
        .window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // fills the canvas with the color we set in `set_draw_color`.
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    // track frames
    let mut fpsmanager: FPSManager = FPSManager::new();
    fpsmanager
        .set_framerate(drivers::video::FRAMERATE)
        .expect("Framerate panic");

    'running: loop {
        // log fps
        let fps: i32 = fpsmanager.get_framerate();

        // change the colour so we know it's working
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));

        // clear the stage
        canvas.clear();

        // render a surface, and convert it to a texture bound to the canvas
        let title: String = format!("Hello Rust! FPS: {}", fps);
        let surface = font
            .render(title.as_str())
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        // texture query

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let target = get_centered_rect(
            width,
            height,
            SCREEN_WIDTH - PADDING,
            SCREEN_HEIGHT - PADDING,
        );

        // change the color of our drawing with a gold-color ...
        canvas.set_draw_color(Color::RGB(255, 210, 0));

        // draw a rectangle
        let rectangle: Rect =
            get_centered_rect(400, 200, SCREEN_WIDTH - PADDING, SCREEN_HEIGHT - PADDING);
        canvas.fill_rect(rectangle).unwrap();

        // change color
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let square: Rect =
            get_centered_rect(250, 250, SCREEN_WIDTH - PADDING, SCREEN_HEIGHT - PADDING);
        canvas.fill_rect(square).unwrap();

        canvas.copy(&texture, None, Some(target))?;

        drivers::cpu::tick().expect("Panic ticking CPU");

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    log::info!("Closing");
                    break 'running;
                }
                _ => {
                    log::info!("General input");
                }
            }
        }
        // Present the layers
        // However the canvas has not been updated to the window yet,
        // everything has been processed to an internal buffer,
        // but if we want our buffer to be displayed on the window,
        // we need to call `present`. We need to call this everytime
        // we want to render a new frame on the window.
        canvas.present();

        // time between frames
        let frame_time: u64 = (1000 / fps) as u64;

        // sleep until the next frame
        sleep(Duration::from_millis(frame_time));
    }

    Ok(())
}

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
pub fn get_centered_rect(
    rect_width: u32,
    rect_height: u32,
    cons_width: u32,
    cons_height: u32,
) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            // println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            // println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

pub fn games() -> Vec<&'static str> {
    let games: Vec<&str> = vec![
        "Invaders", "Brix", "Tetris", "Pong", "UFO", "IBM", "Missile", "Tank", "Maze",
    ];
    return games;
}

// tests below

#[cfg(test)]
mod tests {
    #[test]
    fn it_gets_centered_rect() {
        let test_width: u32 = 400;
        let test_height: u32 = 200;

        let rectangle: super::Rect = super::get_centered_rect(
            test_width,
            test_height,
            super::SCREEN_WIDTH - super::PADDING,
            super::SCREEN_HEIGHT - super::PADDING,
        );

        assert_eq!(rectangle.x, 200);
        assert_eq!(rectangle.y, 200);
        assert_eq!(rectangle.w, test_width as i32);
        assert_eq!(rectangle.h, test_height as i32);
    }
}
