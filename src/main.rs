// with thanks to:
// https://sunjay.dev/learn-game-dev/opening-a-window.html
// https://docs.rs/sdl2/0.30.0/sdl2/render/struct.Canvas.html

mod chip;

use std::io::Write;
use std::time::Duration;

use chrono::Local;
use env_logger;
use log::LevelFilter;

use sdl2;
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// const CHIP8_WIDTH: usize = 64;
// const CHIP8_HEIGHT: usize = 32;
// const CHIP8_RAM: usize = 4096;

// const SCALE_FACTOR: u32 = 20;
// const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
// const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

fn main() -> Result<(), String> {
    // set up our logger prefix
    // i guess i'll move this somewhere more sensible soon

    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::info!("Logger: {:?}", true);

    // configure video

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // fills the canvas with the color we set in `set_draw_color`.
    canvas.clear();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    // track frames
    // this syntax is weird but i guess i kind of like it

    let mut fpsmanager: FPSManager = FPSManager::new();
    fpsmanager.set_framerate(60u32).expect("Framerate panic");

    'running: loop {
        // log fps
        let fps: i32 = fpsmanager.get_framerate();
        log::info!("FPS: {:?}", fps);

        // change the colour so we know it's working
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        // change the color of our drawing with a gold-color ...
        canvas.set_draw_color(Color::RGB(255, 210, 0));
        // A draw a rectangle which almost fills our window with it !
        let rectangle: Rect = Rect::new(10, 10, 400, 200);
        canvas.fill_rect(rectangle).unwrap();

        let square: Rect = Rect::new(250, 250, 200, 200);
        canvas.fill_rect(square).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        // Present the layers
        // However the canvas has not been updated to the window yet,
        // everything has been processed to an internal buffer,
        // but if we want our buffer to be displayed on the window,
        // we need to call `present`. We need to call this everytime
        // we want to render a new frame on the window.
        canvas.present();

        // sleep until the next frame
        ::std::thread::sleep(Duration::from_millis(1000 / 60));
    }

    Ok(())
}
