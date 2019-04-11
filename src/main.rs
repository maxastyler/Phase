#![feature(custom_attribute)]
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate relm_attributes;

use relm_attributes::widget;

use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use gtk::{ButtonExt, LabelExt, OrientableExt, WidgetExt, SpinButtonExt};
use relm::{Relm, Update, Widget};

mod pattern_controller;

use pattern_controller::*;

struct Model {}

#[derive(Msg)]
pub enum Msg {
    AddTab,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> () {
        ()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            _ => ()
        }
    }

    view! {
        gtk::Window {
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
