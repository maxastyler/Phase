extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm_attributes::widget;

use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use gtk::{ButtonExt, LabelExt, OrientableExt, WidgetExt, SpinButtonExt};
use relm::{Relm, Update, Widget};

mod pattern_controller;
mod pattern_container;

use pattern_controller::*;
use pattern_container::*;

struct Model {}

#[derive(Msg)]
pub enum Msg {
    AddTab,
    Quit,
}

relm_widget! {
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
				PatternContainer,
				delete_event(_, _) => (Msg::Quit, Inhibit(false)),
			}
		}
	}
}

fn main() {
    Win::run(()).unwrap();
}
