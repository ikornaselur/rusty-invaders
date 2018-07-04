use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let f = File::open(config.filename)?;

    for (idx, byte) in f.bytes().enumerate().take(256) {
        if idx % 16 == 0 {
            println!("");
            print!("{:07X?}   ", idx);
        }
        print!("{:02X?} ", byte.unwrap());
    }

    Ok(())
}

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename missing"),
        };

        Ok(Config { filename })
    }
}
