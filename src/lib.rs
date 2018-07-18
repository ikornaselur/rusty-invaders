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

const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / 60);
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
            window: window,
        }
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

        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        self.window.clear(&Color::BLACK);
        self.window.draw(&sprite);
        self.window.display();
    }

    fn emulate(&mut self) -> () {
        loop {
            if self.state.int_enabled && self.interrupt_timer.elapsed() > FRAME_TIME {
                self.interrupt_timer.reset_last_time();
                self.state.rst(2);
                self.state.di();
                self.draw();
            }
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
