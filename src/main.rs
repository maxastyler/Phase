extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

mod pattern_controller;
mod pattern_container;
mod gui;

use relm::Widget;

use gui::SLMController;

fn main() {
    SLMController::run(()).unwrap();
}
