#![no_main]
#![no_std]
#![feature(error_in_core)]

extern crate alloc;

use alloc::boxed::Box;
use core::error::Error;

use vexide::prelude::*;

struct Robot {}

impl Robot {
    fn new(peripherals: Peripherals) -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }
}

impl CompetitionRobot for Robot {
    type Error = Box<dyn Error>;

    async fn driver(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Hello world!");
        Ok(())
    }

    async fn autonomous(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) -> Result<(), Box<dyn Error>> {
    let robot = Robot::new(peripherals)?;
    robot.compete().await
}
