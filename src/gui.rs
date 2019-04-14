use relm::{Component, ContainerWidget, Relm, Update, Widget};
use std::collections::HashMap;
use gtk::{WidgetExt};

use self::SLMControllerMsg::*;

pub struct SLMControllerModel {}

#[derive(Msg)]
pub enum SLMControllerMsg {
    AddTab,
    Quit,
}

pub struct SLMController {
    pub model: SLMControllerModel,
    pub widget: gtk::Window,
}

impl Update for SLMController {
    type Model = SLMControllerModel;
    type ModelParam = ();
    type Msg = SLMControllerMsg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        SLMControllerModel {}
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Quit => gtk::main_quit(),
            _ => (),
        }
    }
}

impl Widget for SLMController {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.widget.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {

        let widget = gtk::Window::new(gtk::WindowType::Toplevel);

        connect!(relm, widget, connect_delete_event(_, _), return (Quit, gtk::Inhibit(false)));

        SLMController {
            model,
            widget,
        }

    }
}
