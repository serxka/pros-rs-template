#![no_std]
#![no_main]

#[macro_use]
extern crate pros;
use pros::prelude::*;

struct VexRobot;

impl Robot for VexRobot {
	fn new(device: Devices) -> Self {
		VexRobot
	}

	fn competition_init(&self) {
		println!("competition_init()");
	}

	fn disabled(&self) {
		println!("disabled()");
	}

	fn autonomous(&self) {
		println!("autonomous()");
	}

	fn opcontrol(&self) {
		println!("opcontrol()");
	}
}

robot!(VexRobot);
