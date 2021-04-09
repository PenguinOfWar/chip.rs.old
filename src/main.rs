// with thanks to:
// https://sunjay.dev/learn-game-dev/opening-a-window.html

mod chip;

use std::time::Duration;

use chrono::Local;
use env_logger;
use log::LevelFilter;

use sdl2;
use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::io::Write;

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
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .map_err(|e| e.to_string())?;
    // Create a red-green gradient
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..256 {
            for x in 0..256 {
                let offset = y * pitch + x * 3;
                buffer[offset] = x as u8;
                buffer[offset + 1] = y as u8;
                buffer[offset + 2] = 0;
            }
        }
    })?;

    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
    canvas.copy_ex(
        &texture,
        None,
        Some(Rect::new(450, 100, 256, 256)),
        30.0,
        None,
        false,
        false,
    )?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    // track frames
    // this syntax is weird but i guess i kind of like it

    let mut fpsmanager: FPSManager = FPSManager::new();
    fpsmanager.set_framerate(60u32).expect("Framerate panic");

    'running: loop {
        for event in event_pump.poll_iter() {
            // log fps
            let fps: i32 = fpsmanager.get_framerate();
            log::info!("FPS: {:?}", fps);

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // Update
        i = (i + 1) % 255;

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
