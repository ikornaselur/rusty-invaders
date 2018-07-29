extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture, Transformable};
use sfml::window::{Event, Key, Style};

use std::thread::sleep;
use std::time::Duration;

use cpu::instructions::interrupt::di;
use cpu::instructions::restart::rst;
use cpu::CPU;
use machine::clock::Clock;

mod clock;
pub mod io;

// 1 Billion nanoseconds divided by 2 million cycles a second
const CYCLE_TIME_NANOS: u64 = 1_000_000_000 / 2_000_000;
const INTERRUPT_TIME: Duration = Duration::from_micros(1_000_000 / 120);
const SCALE: u32 = 8;

pub struct Machine {
    cpu: CPU,
    interrupt_timer: Clock,
    cpu_timer: Clock,
    next_interrupt: u8,
    window: RenderWindow,
    keys: u8,
}

impl Machine {
    pub fn new(mut buffer: Vec<u8>) -> Machine {
        buffer.resize(0x10000, 0);
        let window = RenderWindow::new(
            (224 * SCALE, 256 * SCALE),
            "Rusty Invaders",
            Style::CLOSE,
            &Default::default(),
        );

        Machine {
            cpu: CPU::new(buffer, true),
            interrupt_timer: Clock::new(),
            cpu_timer: Clock::new(),
            next_interrupt: 1,
            window,
            keys: 0,
        }
    }

    fn key_press(&mut self, key: Key) -> () {
        match key {
            Key::C => {
                self.keys |= 1;
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
        self.cpu.set_input(1, self.keys);
    }

    fn key_release(&mut self, key: Key) -> () {
        match key {
            Key::C => {
                self.keys &= !1;
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
        self.cpu.set_input(1, self.keys);
    }

    fn draw(&mut self) -> () {
        let mut texture = Texture::new(256, 224).expect("Unable to create texture");
        let mut buffer = Vec::new();
        for pixel in self.cpu.get_frame() {
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
        sprite.set_position((0f32, SCALE as f32 * 256f32));
        sprite.set_scale((SCALE as f32, SCALE as f32));

        self.window.clear(&Color::BLACK);
        self.window.draw(&sprite);
        self.window.display();
    }

    fn sync(&mut self, cycles: u8) -> () {
        let cycle_duration = Duration::from_nanos(u64::from(cycles) * CYCLE_TIME_NANOS);

        let elapsed = self.cpu_timer.elapsed();

        if cycle_duration > elapsed {
            sleep(cycle_duration - elapsed);
        }

        self.cpu_timer.reset_last_time();
    }

    pub fn emulate(&mut self) -> () {
        loop {
            if self.cpu.int_enabled && self.interrupt_timer.elapsed() > INTERRUPT_TIME {
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
                di(&mut self.cpu);
                match self.next_interrupt {
                    1 => {
                        rst(&mut self.cpu, 1);
                        self.next_interrupt = 2;
                    }
                    2 => {
                        self.draw();
                        rst(&mut self.cpu, 2);
                        self.next_interrupt = 1;
                    }
                    _ => panic!("Invalid interrupt"),
                }
            }

            let cycles = match self.cpu.step() {
                None => break,
                Some(cycles) => cycles,
            };

            self.sync(cycles);
        }
    }
}
