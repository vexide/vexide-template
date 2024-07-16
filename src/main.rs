#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core::error::Error;

use vexide::prelude::*;

struct Robot {}

impl Robot {
    /// Initialize the robot's peripherals, storing the subsystems
    /// it will need to interact with.
    fn new(_peripherals: Peripherals) -> Result<Self, Box<dyn Error>> {
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
