use gtk::{BoxExt, ButtonExt, ContainerExt, DialogExt, NotebookExt, WidgetExt};
use relm::{Component, ContainerWidget, Relm, Update, Widget};

use self::SLMControllerMsg::*;

use crate::pattern_container::PatternContainer;

/// Hello there
pub struct SLMControllerModel {
    pattern_containers: Vec<Component<PatternContainer>>,
    relm: Relm<SLMController>,
}

#[derive(Msg)]
pub enum SLMControllerMsg {
    AddTab,
    RemoveTab,
    RemoveAllTabs,
    Quit,
}

pub struct SLMController {
    pub model: SLMControllerModel,
    pub widget: gtk::Window,
    pub container_notebook: gtk::Notebook,
}

impl SLMController {
    fn add_new_container(&mut self) {
        let widget = self.container_notebook.add_widget::<PatternContainer>(
            self.model.relm.clone(),
        );
        let close_button = gtk::Button::new_with_label("x");
        self.model
            .pattern_containers
            .push(widget);
    }

    fn remove_container(&mut self, id: usize) {
        use gtk::DialogFlags;
        let dialog = gtk::Dialog::new_with_buttons(
            Some("Really delete phase pattern?"),
            Some(&self.root()),
            DialogFlags::DESTROY_WITH_PARENT,
            &[
                ("yes", gtk::ResponseType::Accept.into()),
                ("no", gtk::ResponseType::Reject.into()),
            ],
        );
        if dialog.run() == gtk::ResponseType::Accept.into() {
            if id < self.model.pattern_containers.len() {
                let widget = self.model.pattern_containers.remove(id);
                self.container_notebook.remove(widget.widget());
            }
        }
        dialog.emit_close();
    }

    fn remove_all_containers(&mut self) {
        use gtk::DialogFlags;
        let dialog = gtk::Dialog::new_with_buttons(
            Some("Delete all phase patterns?"),
            Some(&self.root()),
            DialogFlags::DESTROY_WITH_PARENT,
            &[
                ("yes", gtk::ResponseType::Accept.into()),
                ("no", gtk::ResponseType::Reject.into()),
            ],
        );
        if dialog.run() == gtk::ResponseType::Accept.into() {
            for widget in self.model.pattern_containers.drain(0..) {
                self.container_notebook.remove(widget.widget());
            }
        }
        dialog.emit_close();
    }
}

impl Update for SLMController {
    type Model = SLMControllerModel;
    type ModelParam = ();
    type Msg = SLMControllerMsg;

    fn model(relm: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        SLMControllerModel {
            pattern_containers: vec![],
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Quit => gtk::main_quit(),
            AddTab => self.add_new_container(),
            RemoveTab => self.remove_container(self.container_notebook.get_property_page() as usize),
            RemoveAllTabs => self.remove_all_containers(),
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
        let container_control_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let container_notebook = gtk::Notebook::new();
        container_notebook.set_scrollable(true);
        let add_button = gtk::Button::new_with_label("Add container");
        let delete_button = gtk::Button::new_with_label("Delete current container");
        let delete_all_button = gtk::Button::new_with_label("Delete all containers");
        connect!(
            relm,
            widget,
            connect_delete_event(_, _),
            return (Quit, gtk::Inhibit(false))
        );
        connect!(relm, add_button, connect_clicked(_), AddTab);
        connect!(relm, delete_button, connect_clicked(_), RemoveTab);
        connect!(relm, delete_all_button, connect_clicked(_), RemoveAllTabs);

        container_control_box.pack_start(&add_button, false, false, 0);
        container_control_box.pack_end(&delete_all_button, false, false, 0);
        container_control_box.pack_end(&delete_button, false, false, 0);
        split_box.pack_start(&container_control_box, false, false, 0);
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
