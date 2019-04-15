//! This program uses a GUI to generate patterns for SLMs

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate serde;
extern crate serde_json;

pub mod gui;
pub mod pattern_container;
pub mod pattern_controller;
pub mod slm_data;

use relm::Widget;

use gui::SLMController;

fn main() -> Result<(), ()> {
    Ok(SLMController::run(())?)
}
