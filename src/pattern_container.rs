use gtk::{
    BoxExt, ButtonExt, Cast, Container, ContainerExt, EntryExt, GridExt, LabelExt, OrientableExt,
    Orientation, ScrolledWindow, ScrolledWindowExt, SpinButtonExt, WidgetExt, SpinButtonSignals,
};
use relm::{Component, ContainerWidget, Relm, Update, Widget};
use std::collections::HashMap;

use self::PatternContainerMsg::*;
use crate::gui::{SLMController, SLMControllerMsg};
use crate::pattern_controller::PatternController;

pub struct PatternContainerModel {
    patterns: HashMap<usize, Component<PatternController>>,
    top_left: (f64, f64),
    bottom_right: (f64, f64),
    pos: (f64, f64),
    scale: (f64, f64),
    relm: Relm<PatternContainer>,
    parent_relm: Relm<SLMController>,
    current_controller_id: usize,
    id: usize,
}

#[derive(Msg)]
pub enum PatternContainerMsg {
    AddPattern,
    NewCxValue(f64),
    NewCyValue(f64),
    NewScaleXValue(f64),
    NewScaleYValue(f64),
    NewTLXValue(f64),
    NewTLYValue(f64),
    NewBRXValue(f64),
    NewBRYValue(f64),
    DeletePattern(usize),
}

pub struct PatternContainer {
    model: PatternContainerModel,
    root_box: gtk::Box,
    pattern_box: gtk::Box,
}

impl PatternContainer {
    fn add_new_pattern(&mut self) {
        let widget = self.pattern_box.add_widget::<PatternController>((
            self.model.current_controller_id,
            self.model.relm.clone(),
        ));
        self.model
            .patterns
            .insert(self.model.current_controller_id, widget);
        self.model.current_controller_id += 1;
    }
    fn delete_pattern(&mut self, id: usize) {
        if let Some(pattern) = self.model.patterns.remove(&id) {
            self.pattern_box.remove_widget(pattern);
        }
    }
}

impl Update for PatternContainer {
    type Model = PatternContainerModel;
    type ModelParam = (Relm<SLMController>, usize);
    type Msg = PatternContainerMsg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        PatternContainerModel {
            patterns: HashMap::new(),
            top_left: (0.0, 0.0),
            bottom_right: (1920.0, 1080.0),
            scale: (1.0, 1.0),
            pos: (0.0, 0.0),
            relm: relm.clone(),
            parent_relm: param.0,
            current_controller_id: param.1,
            id: 0,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            AddPattern => self.add_new_pattern(),
            DeletePattern(id) => self.delete_pattern(id),
            NewCxValue(x) => self.model.pos.0 = x,
            NewCyValue(x) => self.model.pos.1 = x,
            NewScaleXValue(x) => self.model.scale.0 = x,
            NewScaleYValue(x) => self.model.scale.1 = x,
            NewTLXValue(x) => self.model.top_left.0 = x,
            NewTLYValue(x) => self.model.top_left.1 = x,
            NewBRXValue(x) => self.model.bottom_right.0 = x,
            NewBRYValue(x) => self.model.bottom_right.1 = x,
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
        let spinner_char_width = 7;
        let root_box = gtk::Box::new(Orientation::Vertical, 0);
        let pattern_box = gtk::Box::new(Orientation::Vertical, 0);
        let scroll_view = gtk::ScrolledWindow::new(None, None);
        let add_pattern_button = gtk::Button::new_with_label("Add pattern");
        let view_control_box = gtk::Box::new(Orientation::Horizontal, 0);
        let view_control_grid = gtk::Grid::new();

        let cx_spin_adjustment =
            gtk::Adjustment::new(model.pos.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
        let cy_spin_adjustment =
            gtk::Adjustment::new(model.pos.1, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
        let top_left_x_spin_adjustment = gtk::Adjustment::new(
            model.top_left.0,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let top_left_y_spin_adjustment = gtk::Adjustment::new(
            model.top_left.1,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let bottom_right_x_spin_adjustment = gtk::Adjustment::new(
            model.bottom_right.0,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let bottom_right_y_spin_adjustment = gtk::Adjustment::new(
            model.bottom_right.1,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let scalex_spin_adjustment =
            gtk::Adjustment::new(model.scale.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
        let scaley_spin_adjustment =
            gtk::Adjustment::new(model.scale.1, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);

        let cx_spin = gtk::SpinButton::new(&cx_spin_adjustment, 0.0, 3);
        let cy_spin = gtk::SpinButton::new(&cy_spin_adjustment, 0.0, 3);
        let top_left_x_spin = gtk::SpinButton::new(&top_left_x_spin_adjustment, 0.0, 3);
        let top_left_y_spin = gtk::SpinButton::new(&top_left_y_spin_adjustment, 0.0, 3);
        let bottom_right_x_spin = gtk::SpinButton::new(&bottom_right_x_spin_adjustment, 0.0, 3);
        let bottom_right_y_spin = gtk::SpinButton::new(&bottom_right_y_spin_adjustment, 0.0, 3);
        let scalex_spin = gtk::SpinButton::new(&scalex_spin_adjustment, 0.0, 3);
        let scaley_spin = gtk::SpinButton::new(&scaley_spin_adjustment, 0.0, 3);
        let pos_label = gtk::Label::new("pos (x, y)");
        let scale_label = gtk::Label::new("scale (x, y)");
        let top_left_label = gtk::Label::new("top left (x, y)");
        let bottom_right_label = gtk::Label::new("bottom right (x, y)");

        cx_spin.set_width_chars(spinner_char_width);
        cy_spin.set_width_chars(spinner_char_width);
        top_left_x_spin.set_width_chars(spinner_char_width);
        top_left_y_spin.set_width_chars(spinner_char_width);
        bottom_right_x_spin.set_width_chars(spinner_char_width);
        bottom_right_y_spin.set_width_chars(spinner_char_width);
        scalex_spin.set_width_chars(spinner_char_width);
        scaley_spin.set_width_chars(spinner_char_width);

        pattern_box.set_spacing(10);
        connect!(
            relm,
            add_pattern_button,
            connect_clicked(_),
            PatternContainerMsg::AddPattern
        );

        connect!(
            relm,
            cx_spin,
            connect_value_changed(x),
            NewCxValue(x.get_value())
        );
        connect!(
            relm,
            cy_spin,
            connect_value_changed(x),
            NewCyValue(x.get_value())
        );
        connect!(
            relm,
            scalex_spin,
            connect_value_changed(x),
            NewScaleXValue(x.get_value())
        );
        connect!(
            relm,
            scaley_spin,
            connect_value_changed(x),
            NewScaleYValue(x.get_value())
        );
        connect!(
            relm,
            top_left_x_spin,
            connect_value_changed(x),
            NewTLXValue(x.get_value())
        );
        connect!(
            relm,
            top_left_y_spin,
            connect_value_changed(x),
            NewTLYValue(x.get_value())
        );
        connect!(
            relm,
            bottom_right_x_spin,
            connect_value_changed(x),
            NewBRXValue(x.get_value())
        );
        connect!(
            relm,
            bottom_right_y_spin,
            connect_value_changed(x),
            NewBRYValue(x.get_value())
        );

        view_control_grid.attach(&top_left_label, 0, 0, 1, 1);
        view_control_grid.attach(&top_left_x_spin, 1, 0, 1, 1);
        view_control_grid.attach(&top_left_y_spin, 2, 0, 1, 1);
        view_control_grid.attach(&bottom_right_label, 0, 1, 1, 1);
        view_control_grid.attach(&bottom_right_x_spin, 1, 1, 1, 1);
        view_control_grid.attach(&bottom_right_y_spin, 2, 1, 1, 1);
        view_control_grid.attach(&pos_label, 0, 2, 1, 1);
        view_control_grid.attach(&cx_spin, 1, 2, 1, 1);
        view_control_grid.attach(&cy_spin, 2, 2, 1, 1);
        view_control_grid.attach(&scale_label, 0, 3, 1, 1);
        view_control_grid.attach(&scalex_spin, 1, 3, 1, 1);
        view_control_grid.attach(&scaley_spin, 2, 3, 1, 1);
        view_control_box.pack_start(&view_control_grid, false, false, 0);
        view_control_box.pack_end(&add_pattern_button, false, false, 0);

        scroll_view.add_with_viewport(&pattern_box);
        root_box.pack_start(&view_control_box, false, false, 10);
        root_box.pack_end(&scroll_view, true, true, 0);

        root_box.show_all();
        PatternContainer {
            model,
            root_box: root_box,
            pattern_box: pattern_box,
        }
    }
}
