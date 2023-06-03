#![no_std]
#![no_main]

extern crate pros;
use pros::prelude::*;

struct VexRobot;

impl Robot for VexRobot {
	fn new(device: Devices) -> Self {
		VexRobot
	}

	fn competition_init(&'static self, state: CompetitionState) {
		println!("competition_init()");
	}

	fn disabled(&'static self, state: CompetitionState) {
		println!("disabled()");
	}

	fn autonomous(&'static self, state: CompetitionState) {
		println!("autonomous()");
	}

	fn opcontrol(&'static self, state: CompetitionState) {
		println!("opcontrol()");
	}
}

robot!(VexRobot);
