mod emulator;
extern crate sdl2;

use emulator::{CHIP8_SCREEN_HEIGHT, CHIP8_SCREEN_WIDTH};
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
        .window("CHIP8 emulator", SDL_SCREEN_WIDTH, SDL_SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut emulator = emulator::Emulator::new();
    emulator.load_program(&[
        // Set V0 to sprite value
        0x60, 0b10101010, // Set I to random address in memory
        0xA3, 0x00, // Store the content of V0 to I in memory
        0xF0, 0x55, // Set x and y position to paste sprite in V0 and V1
        0x60, 0x05, 0x80, 0x24, 0x61, 0x06, // Finally, paste sprite
        0xD0, 0x10, // Increment V2 by 1
        0x72, 0x01, // Back to square one
        0x12, 0x00,
    ]);

    emulator.load_program(std::fs::read("INVADERS").unwrap().as_slice());

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
                    keycode: Some(Keycode::KP_0),
                    ..
                } => emulator.input_key(0x00, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_0),
                    ..
                } => emulator.input_key(0x00, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_1),
                    ..
                } => emulator.input_key(0x01, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_1),
                    ..
                } => emulator.input_key(0x01, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_2),
                    ..
                } => emulator.input_key(0x02, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_2),
                    ..
                } => emulator.input_key(0x02, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_3),
                    ..
                } => emulator.input_key(0x03, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_3),
                    ..
                } => emulator.input_key(0x03, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_4),
                    ..
                } => emulator.input_key(0x04, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_4),
                    ..
                } => emulator.input_key(0x04, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_5),
                    ..
                } => emulator.input_key(0x05, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_5),
                    ..
                } => emulator.input_key(0x05, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_6),
                    ..
                } => emulator.input_key(0x06, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_6),
                    ..
                } => emulator.input_key(0x06, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_7),
                    ..
                } => emulator.input_key(0x07, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_7),
                    ..
                } => emulator.input_key(0x07, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_8),
                    ..
                } => emulator.input_key(0x08, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_8),
                    ..
                } => emulator.input_key(0x08, false),

                Event::KeyDown {
                    keycode: Some(Keycode::KP_9),
                    ..
                } => emulator.input_key(0x09, true),
                Event::KeyUp {
                    keycode: Some(Keycode::KP_9),
                    ..
                } => emulator.input_key(0x09, false),

                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => emulator.input_key(0xA, true),
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => emulator.input_key(0xA, false),

                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => emulator.input_key(0xB, true),
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => emulator.input_key(0xB, false),

                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => emulator.input_key(0xC, true),
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => emulator.input_key(0xC, false),

                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => emulator.input_key(0xD, true),
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => emulator.input_key(0xD, false),

                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => emulator.input_key(0xE, true),
                Event::KeyUp {
                    keycode: Some(Keycode::T),
                    ..
                } => emulator.input_key(0xE, false),

                Event::KeyDown {
                    keycode: Some(Keycode::Y),
                    ..
                } => emulator.input_key(0xF, true),
                Event::KeyUp {
                    keycode: Some(Keycode::Y),
                    ..
                } => emulator.input_key(0xF, false),

                _ => {}
            }
        }

        // Several cpu cycles per render cycle
        if !emulator.waiting_for_key {
            for _ in 0..20 {
                emulator.process_next_instruction();
                println!();
            }
        }

        if emulator.system_clock > 0 {
            emulator.system_clock -= 1;
        }

        canvas.set_draw_color(Color::WHITE);
        for i in 0..CHIP8_SCREEN_WIDTH {
            for j in 0..CHIP8_SCREEN_HEIGHT {
                match emulator.screen[j * CHIP8_SCREEN_WIDTH + i] {
                    emulator::PixelStatus::Black => (),
                    emulator::PixelStatus::White => {
                        let i = i as i32;
                        let j = j as i32;
                        let pixel_size_ratio = PIXEL_SIZE_RATIO as i32;
                        let white_pixel = sdl2::rect::Rect::new(
                            i * pixel_size_ratio,
                            j * pixel_size_ratio,
                            PIXEL_SIZE_RATIO as u32,
                            PIXEL_SIZE_RATIO as u32,
                        );
                        canvas.fill_rect(white_pixel).unwrap();
                    }
                }
            }
        }

        // Display new screen
        canvas.present();

        // About 60Hz of refresh time
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
