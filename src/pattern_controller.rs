use self::PatternControllerMsg::*;
use gtk::{
    Adjustment, BoxExt, ButtonExt, EntryExt, GridExt, LabelExt, OrientableExt, Orientation,
    SpinButtonExt, SpinButtonSignals, WidgetExt,
};
use relm::{Component, Relm, Update, Widget};

use crate::pattern_container::*;

#[derive(Clone)]
pub struct PatternControllerModel {
    L: i32,
    a: f64,
    k: (f64, f64),
    c: (f64, f64),
    phase: f64,
    pub container_relm: Relm<PatternContainer>,
    id: usize,
}

#[derive(Msg)]
pub enum PatternControllerMsg {
    NewLValue(i32),
    NewAValue(f64),
    NewKxValue(f64),
    NewKyValue(f64),
    NewCxValue(f64),
    NewCyValue(f64),
    NewPhaseValue(f64),
    DeleteSelf,
}

#[derive(Clone)]
pub struct PatternController {
    pub model: PatternControllerModel,
    pub widget: gtk::Box,
}

impl Update for PatternController {
    type Model = PatternControllerModel;
    type ModelParam = (usize, Relm<PatternContainer>);
    type Msg = PatternControllerMsg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        PatternControllerModel {
            L: 0,
            a: 0.0,
            k: (0.0, 0.0),
            c: (0.0, 0.0),
            phase: 0.0,
            id: param.0,
            container_relm: param.1,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        use crate::pattern_container::PatternContainerMsg::DeletePattern;
        match event {
            DeleteSelf => self
                .model
                .container_relm
                .stream()
                .emit(DeletePattern(self.model.id)),
            NewLValue(x) => self.model.L = x,
            NewAValue(x) => self.model.a = x,
            NewKxValue(x) => self.model.k.0 = x,
            NewKyValue(x) => self.model.k.1 = x,
            NewCxValue(x) => self.model.c.0 = x,
            NewCyValue(x) => self.model.c.1 = x,
            NewPhaseValue(x) => self.model.phase = x,
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
            0.0,
            std::i32::MIN as f64,
            std::i32::MAX as f64,
            1.0,
            0.0,
            0.0,
        );
        let a_spin_adjustment =
            gtk::Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
        let kx_spin_adjustment =
            gtk::Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
        let ky_spin_adjustment =
            gtk::Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
        let cx_spin_adjustment =
            gtk::Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
        let cy_spin_adjustment =
            gtk::Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
        let phase_spin_adjustment =
            gtk::Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0);
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

        root_widget.pack_start(&grid_widget, false, false, 0);
        root_widget.pack_start(&delete_button, false, false, 0);
        root_widget.show_all();

        PatternController {
            model: model,
            widget: root_widget,
        }
    }
}

// relm_widget! {
// 	impl Widget for PatternController {
// 		fn model(relm: &Relm<Self>) -> PatternControllerModel {
// 			PatternControllerModel {
// 				L: 0,
// 				a: 0.0,
// 				k: (0.0, 0.0),
// 				c: (0.0, 0.0),
// 				phase: 0.0,
//                 relm: relm.clone()
// 			}
// 		}
// 		fn update(&mut self, event: PatternControllerMsg) {
// 			match event {
// 				NewLValue => self.model.L = self.l_box.get_value_as_int(),
// 				NewAValue => self.model.a = self.a_box.get_value(),
// 				NewKxValue => self.model.k.0 = self.kx_box.get_value(),
// 				NewKyValue => self.model.k.1 = self.ky_box.get_value(),
// 				NewCxValue => self.model.c.0 = self.cx_box.get_value(),
// 				NewCyValue => self.model.c.1 = self.cy_box.get_value(),
// 				NewPhaseValue => self.model.phase = self.phase_box.get_value(),
// 			}
// 		}

// 		view! {
// 			gtk::Box {
// 				orientation: Orientation::Horizontal,
// 				gtk::Grid {
// 					gtk::Label {
// 						text: "L",
// 						cell: {
// 							left_attach: 0,
// 							top_attach: 0,
// 						}
// 					},
// 					#[name="l_box"]
// 					gtk::SpinButton {
// 						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
// 						cell: {
// 							left_attach: 1,
// 							top_attach: 0,
// 						},
// 						value_changed => NewLValue,
// 					},
// 					gtk::Label {
// 						text: "a",
// 						cell: {
// 							left_attach: 0,
// 							top_attach: 1,
// 						}
// 					},
// 					#[name="a_box"]
// 					gtk::SpinButton {
// 						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
// 						cell: {
// 							left_attach: 1,
// 							top_attach: 1,
// 						},
// 						value_changed => NewAValue,
// 					},
// 					gtk::Label {
// 						text: "k(x, y)",
// 						cell: {
// 							left_attach: 2,
// 							top_attach: 0,
// 						}
// 					},
// 					#[name="kx_box"]
// 					gtk::SpinButton {
// 						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
// 						cell: {
// 							left_attach: 3,
// 							top_attach: 0,
// 						},
// 						value_changed => NewKxValue,
// 					},
// 					#[name="ky_box"]
// 					gtk::SpinButton {
// 						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
// 						cell: {
// 							left_attach: 4,
// 							top_attach: 0,
// 						},
// 						value_changed => NewKyValue,
// 					},
// 					gtk::Label {
// 						text: "c(x, y)",
// 						cell: {
// 							left_attach: 2,
// 							top_attach: 1,
// 						}
// 					},
// 					#[name="cx_box"]
// 					gtk::SpinButton {
// 						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
// 						cell: {
// 							left_attach: 3,
// 							top_attach: 1,
// 						},
// 						value_changed => NewCxValue,
// 					},
// 					#[name="cy_box"]
// 					gtk::SpinButton {
// 						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
// 						cell: {
// 							left_attach: 4,
// 							top_attach: 1,
// 						},
// 						value_changed => NewCyValue,
// 					},
// 					gtk::Label {
// 						text: "Φ",
// 						cell: {
// 							left_attach: 5,
// 							top_attach: 0,
// 						}
// 					},
// 					#[name="phase_box"]
// 					gtk::SpinButton {
// 						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 0.001, 0.0, 0.0),
// 						cell: {
// 							left_attach: 6,
// 							top_attach: 0,
// 						},
// 						value_changed => NewPhaseValue,
// 					},
// 				},
// 				#[name="delete_button"]
// 				gtk::Button {
// 					label: "x"
// 				}
// 			}
// 		}
// 	}
// }
