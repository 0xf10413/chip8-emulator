mod emulator;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const PIXEL_SIZE_RATIO: u32 = 10;
const SDL_SCREEN_WIDTH: u32 = (emulator::CHIP8_SCREEN_WIDTH as u32) * PIXEL_SIZE_RATIO;
const SDL_SCREEN_HEIGHT: u32 = (emulator::CHIP8_SCREEN_HEIGHT as u32) * PIXEL_SIZE_RATIO;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "rust-sdl2 demo",SDL_SCREEN_WIDTH, SDL_SCREEN_HEIGHT
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut emulator = emulator::EmulatorCpuMemory::new();
    emulator.load_program(&[0x6A, 0x15]);

    'running: loop {
        // Clear screen
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => {
                    emulator.process_next_instruction();
                }
                _ => {}
            }
        }

        // Display new screen
        canvas.present();

        // Slow down a bit
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
