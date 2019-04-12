use gtk::{BoxExt, ButtonExt, Container, LabelExt, OrientableExt, Orientation, WidgetExt, ContainerExt, Cast};
use relm::{Component, ContainerWidget, Relm, Update, Widget};

use crate::pattern_controller::PatternController;

pub struct PatternContainerModel {
	patterns: Vec<Component<PatternController>>,
	dims: (f64, f64),
	pos: (f64, f64),
}

#[derive(Msg)]
pub enum PatternContainerMsg {
	AddPattern,
}

pub struct PatternContainer {
	model: PatternContainerModel,
	root_box: gtk::Box,
	add_pattern_button: gtk::Button,
	pattern_box: gtk::Box,
	relm: Relm<Self>,
}

impl Update for PatternContainer {
	type Model = PatternContainerModel;
	type ModelParam = ();
	type Msg = PatternContainerMsg;

	fn model(_: &Relm<Self>, _: ()) -> Self::Model {
		PatternContainerModel {
			patterns: vec![],
			dims: (0.0, 0.0),
			pos: (0.0, 0.0),
		}
	}

	fn update(&mut self, event: Self::Msg) {
		match event {
			PatternContainerMsg::AddPattern => {
				// let widget = self.pattern_box.add_widget::<PatternController>(());
				// let b = widget.widget().get_children()[1].downcast_ref::<gtk::Button>();
				// connect!(&self.relm, b.unwrap(), connect_clicked(_), PatternContainerMsg::AddPattern);
			}
		}
	}
}

impl Widget for PatternContainer {
	type Root = gtk::Box;

	fn root(&self) -> Self::Root {
		self.root_box.clone()
	}

	fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
		let root_box = gtk::Box::new(Orientation::Vertical, 0);
		let pattern_box = gtk::Box::new(Orientation::Vertical, 0);
		let add_pattern_button = gtk::Button::new_with_label("Add new thing");
		connect!(
			relm,
			add_pattern_button,
			connect_clicked(_),
			PatternContainerMsg::AddPattern
		);
		root_box.pack_start(&add_pattern_button, false, false, 0);
		root_box.pack_start(&pattern_box, false, false, 0);
		root_box.show_all();
		PatternContainer {
			model,
			root_box: root_box,
			add_pattern_button: add_pattern_button,
			pattern_box: pattern_box,
			relm: relm.clone()
		}
	}
}
