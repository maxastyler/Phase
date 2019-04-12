use gtk::{
    BoxExt, ButtonExt, Cast, Container, ContainerExt, LabelExt, OrientableExt, Orientation,
    WidgetExt,
};
use relm::{Component, ContainerWidget, Relm, Update, Widget};
use std::collections::HashMap;

use self::PatternContainerMsg::*;
use crate::pattern_controller::PatternController;

pub struct PatternContainerModel {
    patterns: HashMap<usize, Component<PatternController>>,
    dims: (f64, f64),
    pos: (f64, f64),
    relm: Relm<PatternContainer>,
    cur_id: usize,
}

#[derive(Msg)]
pub enum PatternContainerMsg {
    AddPattern,
    DeletePattern(usize),
}

pub struct PatternContainer {
    model: PatternContainerModel,
    root_box: gtk::Box,
    add_pattern_button: gtk::Button,
    pattern_box: gtk::Box,
}

impl Update for PatternContainer {
    type Model = PatternContainerModel;
    type ModelParam = ();
    type Msg = PatternContainerMsg;

    fn model(relm: &Relm<Self>, _: ()) -> Self::Model {
        PatternContainerModel {
            patterns: HashMap::new(),
            dims: (0.0, 0.0),
            pos: (0.0, 0.0),
            relm: relm.clone(),
            cur_id: 0,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            AddPattern => {
                let widget = self
                    .pattern_box
                    .add_widget::<PatternController>((self.model.cur_id, self.model.relm.clone()));
                self.model.patterns.insert(self.model.cur_id, widget);
                self.model.cur_id += 1;
            }
            DeletePattern(id) => if let Some(pattern) = self.model.patterns.remove(&id) {
                self.pattern_box.remove_widget(pattern); 
            },
            _ => (),
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
        }
    }
}
