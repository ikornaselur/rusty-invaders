extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Key, Style};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use clock::Clock;
use state::State;

pub mod clock;
pub mod io;
pub mod state;

// 1 Billion nanoseconds divided by 2 million cycles a second
const CYCLE_TIME_NANOS: u64 = 1_000_000_000 / 2_000_000;
const INTERRUPT_TIME: Duration = Duration::from_micros(1_000_000 / 120);
const SCALE: f32 = 8f32;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let mut machine = Machine::new(buffer);
    machine.emulate();

    Ok(())
}

struct Machine {
    state: State,
    interrupt_timer: Clock,
    cpu_timer: Clock,
    next_interrupt: u8,
    window: RenderWindow,
    keys: u8,
}

impl Machine {
    fn new(mut buffer: Vec<u8>) -> Machine {
        buffer.resize(0x10000, 0);
        let window = RenderWindow::new(
            (224 * (SCALE as u32), 256 * (SCALE as u32)),
            "Rusty Invaders",
            Style::CLOSE,
            &Default::default(),
        );

        Machine {
            state: State::new(buffer, true),
            interrupt_timer: Clock::new(),
            cpu_timer: Clock::new(),
            next_interrupt: 1,
            window: window,
            keys: 0,
        }
    }

    fn key_press(&mut self, key: Key) -> () {
        match key {
            Key::C => {
                self.keys |= 1 << 0;
            }
            Key::Return => {
                self.keys |= 1 << 2;
            }
            Key::Space => {
                self.keys |= 1 << 4;
            }
            Key::Left => {
                self.keys |= 1 << 5;
            }
            Key::Right => {
                self.keys |= 1 << 6;
            }
            _ => (),
        }
        self.state.set_input(1, self.keys);
    }

    fn key_release(&mut self, key: Key) -> () {
        match key {
            Key::C => {
                self.keys &= !(1 << 0);
            }
            Key::Return => {
                self.keys &= !(1 << 2);
            }
            Key::Space => {
                self.keys &= !(1 << 4);
            }
            Key::Left => {
                self.keys &= !(1 << 5);
            }
            Key::Right => {
                self.keys &= !(1 << 6);
            }
            _ => (),
        }
        self.state.set_input(1, self.keys);
    }

    fn draw(&mut self) -> () {
        let mut texture = Texture::new(256, 224).expect("Unable to create texture");
        let mut buffer = Vec::new();
        for pixel in self.state.get_frame() {
            for i in 0..8 {
                let res = if (pixel & 1 << i) > 0 { 0xff } else { 0x00 };
                buffer.push(res as u8);
                buffer.push(res as u8);
                buffer.push(res as u8);
                buffer.push(255);
            }
        }

        texture.update_from_pixels(&buffer, 256, 224, 0, 0);

        let mut sprite = Sprite::with_texture(&texture);
        sprite.set_rotation(270f32);
        sprite.set_position((0f32, SCALE * 256f32));
        sprite.set_scale((SCALE, SCALE));

        self.window.clear(&Color::BLACK);
        self.window.draw(&sprite);
        self.window.display();
    }

    fn sync(&mut self, cycles: u8) -> () {
        let cycle_duration = Duration::from_nanos(cycles as u64 * CYCLE_TIME_NANOS);

        let elapsed = self.cpu_timer.elapsed();
        let elapsed = elapsed.subsec_nanos() as u64 + elapsed.as_secs() * 1_000_000_000;
        let elapsed = Duration::from_nanos(elapsed);

        if cycle_duration > elapsed {
            sleep(cycle_duration - elapsed);
        }

        self.cpu_timer.reset_last_time();
    }

    fn emulate(&mut self) -> () {
        loop {
            if self.state.int_enabled && self.interrupt_timer.elapsed() > INTERRUPT_TIME {
                while let Some(event) = self.window.poll_event() {
                    match event {
                        Event::Closed
                        | Event::KeyPressed {
                            code: Key::Escape, ..
                        } => return,
                        Event::KeyPressed { code, .. } => self.key_press(code),
                        Event::KeyReleased { code, .. } => self.key_release(code),
                        _ => {}
                    }
                }
                self.interrupt_timer.reset_last_time();
                self.state.di();
                match self.next_interrupt {
                    1 => {
                        self.state.rst(1);
                        self.next_interrupt = 2;
                    }
                    2 => {
                        self.draw();
                        self.state.rst(2);
                        self.next_interrupt = 1;
                    }
                    _ => panic!("Invalid interrupt"),
                }
            }

            let cycles = match self.state.step() {
                None => break,
                Some(cycles) => cycles,
            };

            self.sync(cycles);
        }
    }
}

pub struct Config {
    pub filename: String,
    pub debug: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename missing"),
        };

        Ok(Config {
            filename,
            debug: false,
        })
    }
}
