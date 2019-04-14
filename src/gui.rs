use gtk::{BoxExt, ButtonExt, ContainerExt, NotebookExt, WidgetExt};
use relm::{Component, ContainerWidget, Relm, Update, Widget};
use std::collections::HashMap;

use self::SLMControllerMsg::*;

use crate::pattern_container::PatternContainer;

pub struct SLMControllerModel {
    pattern_containers: HashMap<usize, Component<PatternContainer>>,
    current_container_id: usize,
    relm: Relm<SLMController>,
}

#[derive(Msg)]
pub enum SLMControllerMsg {
    AddTab,
    Quit,
}

pub struct SLMController {
    pub model: SLMControllerModel,
    pub widget: gtk::Window,
    pub container_notebook: gtk::Notebook,
}

impl SLMController {

    fn add_new_container(&mut self) {
        let widget = self.container_notebook.add_widget::<PatternContainer>((
            self.model.relm.clone(),
            self.model.current_container_id,
        ));
        self.model.pattern_containers.insert(self.model.current_container_id, widget);
        self.model.current_container_id += 1;
    }

    fn remove_container(&mut self, id: usize) {
        if let Some(widget) = self.model.pattern_containers.remove(&id) {
            self.container_notebook.remove(widget.widget());
        };
    }
}

impl Update for SLMController {
    type Model = SLMControllerModel;
    type ModelParam = ();
    type Msg = SLMControllerMsg;

    fn model(relm: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        SLMControllerModel {
            pattern_containers: HashMap::new(),
            current_container_id: 0,
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Quit => gtk::main_quit(),
            AddTab => self.add_new_container(),
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
        let split_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let container_notebook = gtk::Notebook::new();
        let add_button = gtk::Button::new_with_label("Add container");

        connect!(
            relm,
            widget,
            connect_delete_event(_, _),
            return (Quit, gtk::Inhibit(false))
        );
        connect!(relm, add_button, connect_clicked(_), AddTab);

        // let a = widget.add_widget::<PatternContainer>((relm.clone()));

        split_box.pack_start(&add_button, false, false, 0);
        split_box.pack_start(&container_notebook, true, true, 0);
        widget.add(&split_box);
        widget.show_all();

        SLMController {
            model,
            widget,
            container_notebook: container_notebook,
        }
    }
}
