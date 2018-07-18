extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Key, Style};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

use clock::Clock;
use state::State;

pub mod clock;
pub mod io;
pub mod state;

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
    next_interrupt: u8,
    window: RenderWindow,
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
            next_interrupt: 1,
            window: window,
        }
    }

    fn input(&mut self) -> () {
        let mut port1 = 0u8;
        if Key::C.is_pressed() {
            port1 |= 1 << 0;
        }
        if Key::Return.is_pressed() {
            port1 |= 1 << 2;
        }
        if Key::Space.is_pressed() {
            port1 |= 1 << 4;
        }
        if Key::Left.is_pressed() {
            port1 |= 1 << 5;
        }
        if Key::Right.is_pressed() {
            port1 |= 1 << 6;
        }

        self.state.set_input(1, port1);
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

    fn emulate(&mut self) -> () {
        loop {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => return,
                    _ => {}
                }
            }
            if self.state.int_enabled && self.interrupt_timer.elapsed() > INTERRUPT_TIME {
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
            self.input();
            match self.state.step() {
                None => break,
                _ => (),
            }
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
