//! This file contains structures which control an individual phase pattern

use self::PatternControllerMsg::*;
use gtk::{
    BoxExt, ButtonExt, EntryExt, GridExt, Orientation, SpinButtonExt, SpinButtonSignals, WidgetExt,
};
use relm::{Relm, Update, Widget};

use crate::pattern_container::*;
use crate::slm_data::*;

#[derive(Clone)]
pub struct PatternControllerModel {
    pattern_data: PatternData,
    id: usize,
    pub parent_relm: Relm<PatternContainer>,
}

#[derive(Msg)]
pub enum PatternControllerMsg {
    UpdatePatternL(i32),
    UpdatePatternA(f64),
    UpdatePatternKx(f64),
    UpdatePatternKy(f64),
    UpdatePatternCx(f64),
    UpdatePatternCy(f64),
    UpdatePatternPhase(f64),
    DeleteSelf,
}

#[derive(Clone)]
pub struct PatternController {
    pub model: PatternControllerModel,
    pub widget: gtk::Box,
}

impl Update for PatternController {
    type Model = PatternControllerModel;
    type ModelParam = (PatternData, usize, Relm<PatternContainer>);
    type Msg = PatternControllerMsg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        PatternControllerModel {
            pattern_data: param.0,
            id: param.1,
            parent_relm: param.2,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        use crate::pattern_container::PatternContainerMsg;
        match event {
            DeleteSelf => self
                .model
                .parent_relm
                .stream()
                .emit(PatternContainerMsg::DeletePattern(self.model.id)),
            UpdatePatternL(x) => self
                .model
                .parent_relm
                .stream()
                .emit(PatternContainerMsg::UpdatePatternL(self.model.id, x)),
            UpdatePatternA(x) => self
                .model
                .parent_relm
                .stream()
                .emit(PatternContainerMsg::UpdatePatternA(self.model.id, x)),
            UpdatePatternKx(x) => self
                .model
                .parent_relm
                .stream()
                .emit(PatternContainerMsg::UpdatePatternKx(self.model.id, x)),
            UpdatePatternKy(x) => self
                .model
                .parent_relm
                .stream()
                .emit(PatternContainerMsg::UpdatePatternKy(self.model.id, x)),
            UpdatePatternCx(x) => self
                .model
                .parent_relm
                .stream()
                .emit(PatternContainerMsg::UpdatePatternCx(self.model.id, x)),
            UpdatePatternCy(x) => self
                .model
                .parent_relm
                .stream()
                .emit(PatternContainerMsg::UpdatePatternCy(self.model.id, x)),
            UpdatePatternPhase(x) => self
                .model
                .parent_relm
                .stream()
                .emit(PatternContainerMsg::UpdatePatternPhase(self.model.id, x)),
            _ => (),
        }
    }
}

impl Widget for PatternController {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.widget.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let spinner_char_width = 7;
        let root_widget = gtk::Box::new(Orientation::Horizontal, 0);
        let delete_button = gtk::Button::new_with_label("🗙");
        connect!(relm, delete_button, connect_clicked(_), DeleteSelf);
        let l_spin_adjustment = gtk::Adjustment::new(
            model.pattern_data.l as f64,
            std::i32::MIN as f64,
            std::i32::MAX as f64,
            1.0,
            0.0,
            0.0,
        );
        let a_spin_adjustment = gtk::Adjustment::new(
            model.pattern_data.a,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let kx_spin_adjustment = gtk::Adjustment::new(
            model.pattern_data.k.0,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let ky_spin_adjustment = gtk::Adjustment::new(
            model.pattern_data.k.1,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let cx_spin_adjustment = gtk::Adjustment::new(
            model.pattern_data.c.0,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let cy_spin_adjustment = gtk::Adjustment::new(
            model.pattern_data.c.1,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let phase_spin_adjustment = gtk::Adjustment::new(
            model.pattern_data.phase,
            std::f64::MIN,
            std::f64::MAX,
            1.0,
            0.0,
            0.0,
        );
        let l_label = gtk::Label::new("l");
        let l_spinner = gtk::SpinButton::new(&l_spin_adjustment, 0.0, 0);
        l_spinner.set_width_chars(spinner_char_width);
        let a_label = gtk::Label::new("a");
        let a_spinner = gtk::SpinButton::new(&a_spin_adjustment, 0.0, 3);
        a_spinner.set_width_chars(spinner_char_width);
        let k_label = gtk::Label::new("k(x, y)");
        let kx_spinner = gtk::SpinButton::new(&kx_spin_adjustment, 0.0, 3);
        kx_spinner.set_width_chars(spinner_char_width);
        let ky_spinner = gtk::SpinButton::new(&ky_spin_adjustment, 0.0, 3);
        ky_spinner.set_width_chars(spinner_char_width);
        let c_label = gtk::Label::new("c(x, y)");
        let cx_spinner = gtk::SpinButton::new(&cx_spin_adjustment, 0.0, 3);
        cx_spinner.set_width_chars(spinner_char_width);
        let cy_spinner = gtk::SpinButton::new(&cy_spin_adjustment, 0.0, 3);
        cy_spinner.set_width_chars(spinner_char_width);
        let phase_label = gtk::Label::new("φ");
        let phase_spinner = gtk::SpinButton::new(&phase_spin_adjustment, 0.0, 3);
        phase_spinner.set_width_chars(spinner_char_width);

        let grid_widget = gtk::Grid::new();
        grid_widget.attach(&l_label, 0, 0, 1, 1);
        grid_widget.attach(&l_spinner, 1, 0, 1, 1);
        grid_widget.attach(&a_label, 0, 1, 1, 1);
        grid_widget.attach(&a_spinner, 1, 1, 1, 1);
        grid_widget.attach(&k_label, 2, 0, 1, 1);
        grid_widget.attach(&kx_spinner, 3, 0, 1, 1);
        grid_widget.attach(&ky_spinner, 4, 0, 1, 1);
        grid_widget.attach(&c_label, 2, 1, 1, 1);
        grid_widget.attach(&cx_spinner, 3, 1, 1, 1);
        grid_widget.attach(&cy_spinner, 4, 1, 1, 1);
        grid_widget.attach(&phase_label, 5, 0, 1, 1);
        grid_widget.attach(&phase_spinner, 5, 1, 1, 1);

        connect!(
            relm,
            l_spinner,
            connect_value_changed(x),
            NewLValue(x.get_value_as_int())
        );
        connect!(
            relm,
            a_spinner,
            connect_value_changed(x),
            NewAValue(x.get_value())
        );
        connect!(
            relm,
            kx_spinner,
            connect_value_changed(x),
            NewKxValue(x.get_value())
        );
        connect!(
            relm,
            ky_spinner,
            connect_value_changed(x),
            NewKyValue(x.get_value())
        );
        connect!(
            relm,
            cx_spinner,
            connect_value_changed(x),
            NewCxValue(x.get_value())
        );
        connect!(
            relm,
            cy_spinner,
            connect_value_changed(x),
            NewCyValue(x.get_value())
        );
        connect!(
            relm,
            phase_spinner,
            connect_value_changed(x),
            NewPhaseValue(x.get_value())
        );

        root_widget.pack_start(&grid_widget, false, false, 0);
        root_widget.pack_end(&delete_button, false, false, 0);
        root_widget.show_all();

        PatternController {
            model: model,
            widget: root_widget,
        }
    }
}
