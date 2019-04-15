//! This module contains the definition of the container for a group of patterns
use gtk::{
    BoxExt, ButtonExt, EntryExt, GridExt, Orientation, ScrolledWindowExt, SpinButtonExt,
    SpinButtonSignals, WidgetExt,
};
use relm::{Component, ContainerWidget, Relm, Update, Widget};
use std::collections::HashMap;

use self::PatternContainerMsg::*;
use crate::gui::SLMController;
use crate::pattern_controller::PatternController;
use crate::slm_data::*;

/// The model for the pattern container
#[derive(Clone)]
pub struct PatternContainerModel {
    patterns_data: PatternContainerData,
    parent_relm: Relm<SLMController>,
    current_controller_id: usize,
    id: usize,
}

/// The messages that the container accepts
#[derive(Msg)]
pub enum PatternContainerMsg {
    AddPattern,
    UpdatePatternL(usize, i32),
    UpdatePatternA(usize, f64),
    UpdatePatternKx(usize, f64),
    UpdatePatternKy(usize, f64),
    UpdatePatternCx(usize, f64),
    UpdatePatternCy(usize, f64),
    UpdatePatternPhase(usize, f64),
    UpdateContainerCx(f64),
    UpdateContainerCy(f64),
    UpdateContainerScaleX(f64),
    UpdateContainerScaleY(f64),
    UpdateContainerTLX(f64),
    UpdateContainerTLY(f64),
    UpdateContainerBRX(f64),
    UpdateContainerBRY(f64),
    DeletePattern(usize),
}

#[derive(Clone)]
pub struct PatternContainer {
    model: PatternContainerModel,
    root_box: gtk::Box,
    pattern_box: gtk::Box,
    relm: Relm<Self>,
    parent_relm: Relm<SLMController>,
    patterns: HashMap<usize, Component<PatternController>>,
}

impl PatternContainer {
    /// Add a new pattern to this pattern container
    pub fn add_new_pattern(&mut self) {
        let widget = self.pattern_box.add_widget::<PatternController>((
            PatternData::default(),
            self.model.current_controller_id,
            self.relm.clone(),
        ));
        self.patterns
            .insert(self.model.current_controller_id, widget);
        self.model.current_controller_id += 1;
    }

    /// Delete the pattern at `id` from this container
    fn delete_pattern(&mut self, id: usize) {
        if let Some(pattern) = self.patterns.remove(&id) {
            self.pattern_box.remove_widget(pattern);
        }
    }
}

impl Update for PatternContainer {
    type Model = PatternContainerModel;
    type ModelParam = (PatternContainerData, Relm<SLMController>, usize);
    type Msg = PatternContainerMsg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        PatternContainerModel {
            patterns_data: param.0,
            parent_relm: param.1,
            id: param.2,
            current_controller_id: 0,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        use crate::gui::SLMControllerMsg;
        match event {
            AddPattern => self.add_new_pattern(),
            DeletePattern(id) => self.delete_pattern(id),
            UpdatePatternL(id, x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdatePatternL(self.model.id, id, x)),
            UpdatePatternA(id, x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdatePatternA(self.model.id, id, x)),
            UpdatePatternKx(id, x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdatePatternKx(self.model.id, id, x)),
            UpdatePatternKy(id, x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdatePatternKy(self.model.id, id, x)),
            UpdatePatternCx(id, x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdatePatternCx(self.model.id, id, x)),
            UpdatePatternCy(id, x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdatePatternCy(self.model.id, id, x)),
            UpdatePatternPhase(id, x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdatePatternPhase(self.model.id, id, x)),
            UpdateContainerCx(x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdateContainerCx(self.model.id, x)),
            UpdateContainerCy(x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdateContainerCy(self.model.id, x)),
            UpdateContainerScaleX(x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdateContainerScaleX(self.model.id, x)),
            UpdateContainerScaleY(x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdateContainerScaleY(self.model.id, x)),
            UpdateContainerTLX(x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdateContainerTLX(self.model.id, x)),
            UpdateContainerTLY(x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdateContainerTLY(self.model.id, x)),
            UpdateContainerBRX(x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdateContainerBRX(self.model.id, x)),
            UpdateContainerBRY(x) => self.parent_relm.stream().emit(SLMControllerMsg::UpdateContainerBRY(self.model.id, x)),
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

        let cx_spin_adjustment = gtk::Adjustment::new(
            model.patterns_data.pos.0,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let cy_spin_adjustment = gtk::Adjustment::new(
            model.patterns_data.pos.1,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let top_left_x_spin_adjustment = gtk::Adjustment::new(
            model.patterns_data.top_left.0,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let top_left_y_spin_adjustment = gtk::Adjustment::new(
            model.patterns_data.top_left.1,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let bottom_right_x_spin_adjustment = gtk::Adjustment::new(
            model.patterns_data.bottom_right.0,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let bottom_right_y_spin_adjustment = gtk::Adjustment::new(
            model.patterns_data.bottom_right.1,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let scalex_spin_adjustment = gtk::Adjustment::new(
            model.patterns_data.scale.0,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let scaley_spin_adjustment = gtk::Adjustment::new(
            model.patterns_data.scale.1,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );

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
        let parent_relm = model.parent_relm.clone();
        PatternContainer {
            model,
            root_box: root_box,
            pattern_box: pattern_box,
            parent_relm: parent_relm,
            patterns: HashMap::new(),
            relm: relm.clone(),
        }
    }
}
