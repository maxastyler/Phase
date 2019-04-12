use self::PatternControllerMsg::*;
use gtk::{
    Adjustment, BoxExt, ButtonExt, GridExt, LabelExt, OrientableExt, Orientation, SpinButtonExt,
    SpinButtonSignals, WidgetExt,
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
    NewLValue,
    NewAValue,
    NewKxValue,
    NewKyValue,
    NewCxValue,
    NewCyValue,
    NewPhaseValue,
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
        let root_widget = gtk::Box::new(Orientation::Horizontal, 0);
        let button = gtk::Button::new_with_label("hi");
        connect!(relm, button, connect_clicked(_), DeleteSelf);

        root_widget.pack_start(&button, false, false, 0);
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
