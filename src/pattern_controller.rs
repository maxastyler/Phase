use self::PatternControllerMsg::*;
use gtk::{
	Adjustment, ButtonExt, GridExt, LabelExt, OrientableExt, Orientation, SpinButtonExt,
	SpinButtonSignals,
};
use relm::Widget;

pub struct PatternControllerModel {
	L: i32,
	a: f64,
	k: (f64, f64),
	c: (f64, f64),
	phase: f64,
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
}

relm_widget! {
	impl Widget for PatternController {
		fn model() -> PatternControllerModel {
			PatternControllerModel {
				L: 0,
				a: 0.0,
				k: (0.0, 0.0),
				c: (0.0, 0.0),
				phase: 0.0,
			}
		}
		fn update(&mut self, event: PatternControllerMsg) {
			match event {
				NewLValue => self.model.L = self.l_box.get_value_as_int(),
				NewAValue => self.model.a = self.a_box.get_value(),
				NewKxValue => self.model.k.0 = self.kx_box.get_value(),
				NewKyValue => self.model.k.1 = self.ky_box.get_value(),
				NewCxValue => self.model.c.0 = self.cx_box.get_value(),
				NewCyValue => self.model.c.1 = self.cy_box.get_value(),
				NewPhaseValue => self.model.phase = self.phase_box.get_value(),
			}
		}

		view! {
			gtk::Box {
				orientation: Orientation::Horizontal,
				gtk::Grid {
					gtk::Label {
						text: "L",
						cell: {
							left_attach: 0,
							top_attach: 0,
						}
					},
					#[name="l_box"]
					gtk::SpinButton {
						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
						cell: {
							left_attach: 1,
							top_attach: 0,
						},
						value_changed => NewLValue,
					},
					gtk::Label {
						text: "a",
						cell: {
							left_attach: 0,
							top_attach: 1,
						}
					},
					#[name="a_box"]
					gtk::SpinButton {
						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
						cell: {
							left_attach: 1,
							top_attach: 1,
						},
						value_changed => NewAValue,
					},
					gtk::Label {
						text: "k(x, y)",
						cell: {
							left_attach: 2,
							top_attach: 0,
						}
					},
					#[name="kx_box"]
					gtk::SpinButton {
						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
						cell: {
							left_attach: 3,
							top_attach: 0,
						},
						value_changed => NewKxValue,
					},
					#[name="ky_box"]
					gtk::SpinButton {
						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
						cell: {
							left_attach: 4,
							top_attach: 0,
						},
						value_changed => NewKyValue,
					},
					gtk::Label {
						text: "c(x, y)",
						cell: {
							left_attach: 2,
							top_attach: 1,
						}
					},
					#[name="cx_box"]
					gtk::SpinButton {
						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
						cell: {
							left_attach: 3,
							top_attach: 1,
						},
						value_changed => NewCxValue,
					},
					#[name="cy_box"]
					gtk::SpinButton {
						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 1.0, 1.0),
						cell: {
							left_attach: 4,
							top_attach: 1,
						},
						value_changed => NewCyValue,
					},
					gtk::Label {
						text: "Φ",
						cell: {
							left_attach: 5,
							top_attach: 0,
						}
					},
					#[name="phase_box"]
					gtk::SpinButton {
						adjustment: &Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 0.001, 0.0, 0.0),
						cell: {
							left_attach: 6,
							top_attach: 0,
						},
						value_changed => NewPhaseValue,
					},
				},
				#[name="delete_button"]
				gtk::Button {
					label: "x"
				}
			}
		}
	}
}
