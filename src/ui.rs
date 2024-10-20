extern crate sdl2;
use crate::emulator;

use emulator::{CHIP8_SCREEN_HEIGHT, CHIP8_SCREEN_WIDTH};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const PIXEL_SIZE_RATIO: u32 = 15;
const SDL_SCREEN_WIDTH: u32 = (emulator::CHIP8_SCREEN_WIDTH as u32) * PIXEL_SIZE_RATIO;
const SDL_SCREEN_HEIGHT: u32 = (emulator::CHIP8_SCREEN_HEIGHT as u32) * PIXEL_SIZE_RATIO;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

fn map_sdl_keycode_to_chip8_code(sdl_code: Keycode) -> Option<u8> {
    match sdl_code {
        Keycode::KP_0 => Some(0x00),
        Keycode::KP_1 => Some(0x01),
        Keycode::KP_2 => Some(0x02),
        Keycode::KP_3 => Some(0x03),
        Keycode::KP_4 => Some(0x04),
        Keycode::KP_5 => Some(0x05),
        Keycode::KP_6 => Some(0x06),
        Keycode::KP_7 => Some(0x07),
        Keycode::KP_8 => Some(0x08),
        Keycode::KP_9 => Some(0x09),
        Keycode::A => Some(0x0A),
        Keycode::Z => Some(0x0B),
        Keycode::E => Some(0x0C),
        Keycode::R => Some(0x0D),
        Keycode::T => Some(0x0E),
        Keycode::Y => Some(0x0F),
        _ => None,
    }
}

pub fn run_program() {
    // SDL setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    // Sound setup
    let desired_audio_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };
    let device = audio_subsystem
        .open_playback(None, &desired_audio_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        })
        .unwrap();

    // Window setup
    let window = video_subsystem
        .window("CHIP8 emulator", SDL_SCREEN_WIDTH, SDL_SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Emulator setup
    let mut emulator = emulator::Emulator::new();

    emulator.load_program(std::fs::read("roms/BLINKY").unwrap().as_slice());

    // Event setup
    let mut event_pump = sdl_context.event_pump().unwrap();

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
                    keycode: Some(keycode),
                    ..
                } => match map_sdl_keycode_to_chip8_code(keycode) {
                    Some(chip8_code) => emulator.input_key(chip8_code, true),
                    None => (),
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match map_sdl_keycode_to_chip8_code(keycode) {
                    Some(chip8_code) => emulator.input_key(chip8_code, false),
                    None => (),
                },
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

        if emulator.sound_clock > 0 {
            device.resume();
            emulator.sound_clock -= 1;
        } else {
            device.pause();
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
