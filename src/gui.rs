//! This is the main window that the use interacts with to generate phase patterns

use gtk::{
    BoxExt, ButtonExt, ContainerExt, DialogExt, FileChooserExt, NotebookExt, ResponseType,
    WidgetExt,
};
use relm::{Component, ContainerWidget, Relm, Update, Widget};
use std::collections::HashMap;
use std::convert::*;
use std::fs::File;

use self::SLMControllerMsg::*;

use crate::pattern_container::{PatternContainer, PatternContainerMsg};
use crate::slm_data::*;

macro_rules! update_from_pattern_spinner {
    ($self:ident, $c_id:ident, $p_id:ident, $x:ident, $l:tt) => {
        if let Some(container) = $self.model.pattern_data_containers.get_mut(&$c_id) {
            if let Some(pattern) = container.patterns.get_mut(&$p_id) {
                pattern.$l = $x;
            }
        };
    };
    ($self:ident, $c_id:ident, $p_id:ident, $x:ident, $l:tt, $n:tt) => {
        if let Some(container) = $self.model.pattern_data_containers.get_mut(&$c_id) {
            if let Some(pattern) = container.patterns.get_mut(&$p_id) {
                pattern.$l.$n = $x;
            }
        };
    };
}
macro_rules! update_from_container_spinner {
    ($self:ident, $c_id:ident, $x:ident, $l:tt) => {
        if let Some(container) = $self.model.pattern_data_containers.get_mut(&$c_id) {
            container.$l = $x;
        };
    };
    ($self:ident, $c_id:ident, $x:ident, $l:tt, $n:tt) => {
        if let Some(container) = $self.model.pattern_data_containers.get_mut(&$c_id) {
            container.$l.$n = $x;
        };
    };
}

/// The model for the SLM controller.
pub struct SLMControllerModel {
    /// A vector of the pattern containers. For use in the gtk notebook
    pub pattern_data_containers: HashMap<usize, PatternContainerData>,
    current_container_id: usize,
}

/// The messages which the slm controller accepts
#[derive(Msg)]
pub enum SLMControllerMsg {
    AddTab,
    RemoveTab,
    RemoveAllTabs,
    SaveContainers,
    LoadContainers,
    Quit,
    AddController(usize, usize, PatternData),
    RemoveController(usize, usize),
    UpdatePatternL(usize, usize, i32),
    UpdatePatternA(usize, usize, f64),
    UpdatePatternKx(usize, usize, f64),
    UpdatePatternKy(usize, usize, f64),
    UpdatePatternCx(usize, usize, f64),
    UpdatePatternCy(usize, usize, f64),
    UpdatePatternPhase(usize, usize, f64),
    UpdateContainerCx(usize, f64),
    UpdateContainerCy(usize, f64),
    UpdateContainerScaleX(usize, f64),
    UpdateContainerScaleY(usize, f64),
    UpdateContainerTLX(usize, f64),
    UpdateContainerTLY(usize, f64),
    UpdateContainerBRX(usize, f64),
    UpdateContainerBRY(usize, f64),
}

/// The relm slm controller struct
pub struct SLMController {
    pub model: SLMControllerModel,
    /// the root is a ```gtk::window```
    pub widget: gtk::Window,
    /// reference to the ```gtk::Notebook``` to allow manipulation while in use
    pub container_notebook: gtk::Notebook,
    /// reference to the relm
    pub relm: Relm<Self>,
    pub pattern_containers: HashMap<usize, Component<PatternContainer>>,
}

impl SLMController {
    /// Add a new container into the notebook
    pub fn add_new_container(&mut self, container: PatternContainerData) {
        self.add_new_container_no_increment(container);
        self.model.current_container_id += 1;
    }

    pub fn add_new_container_no_increment(&mut self, mut container: PatternContainerData) {
        container.patterns.clear();
        let widget = self.container_notebook.add_widget::<PatternContainer>((
            container.clone(),
            self.relm.clone(),
            self.model.current_container_id,
        ));
        self.pattern_containers
            .insert(self.model.current_container_id, widget);
        self.model
            .pattern_data_containers
            .insert(self.model.current_container_id, container);
    }

    /// Remove the container at position ```id``` in the ```model.pattern_containers``` vector.
    /// Displays a dialog box
    pub fn remove_container(&mut self, id: usize) {
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
        if ResponseType::from(dialog.run()) == ResponseType::Accept {
            if let Some(widget) = self.pattern_containers.remove(&id) {
                self.container_notebook.remove(widget.widget());
            }
            self.model.pattern_data_containers.remove(&id);
        }
        dialog.emit_close();
    }

    /// Remove all containers. Drains the `model.pattern_containers` vector
    pub fn remove_all_containers(&mut self) {
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
        if ResponseType::from(dialog.run()) == ResponseType::Accept {
            for (_, widget) in self.pattern_containers.drain() {
                self.container_notebook.remove(widget.widget());
            }
            self.model.pattern_data_containers.clear();
        }
        dialog.emit_close();
    }

    pub fn save_containers(&self) {
        use gtk::ResponseType;
        let dialog = gtk::FileChooserDialog::with_buttons(
            Some("Save containers"),
            Some(&self.root()),
            gtk::FileChooserAction::Save,
            &[
                ("_Cancel", ResponseType::Cancel),
                ("_Save", ResponseType::Accept),
            ],
        );
        dialog.set_do_overwrite_confirmation(true);
        if ResponseType::from(dialog.run()) == ResponseType::Accept {
            if let Some(filename) = dialog.get_filename() {
                if let Ok(file) = File::create(filename) {
                    serde_json::ser::to_writer_pretty(file, &self.model.pattern_data_containers);
                }
            }
        }
        dialog.emit_close();
    }

    pub fn load_containers(&mut self) {
        use gtk::ResponseType;
        let dialog = gtk::FileChooserDialog::with_buttons(
            Some("Load containers"),
            Some(&self.root()),
            gtk::FileChooserAction::Open,
            &[
                ("_Cancel", ResponseType::Cancel),
                ("_Save", ResponseType::Accept),
            ],
        );
        if ResponseType::from(dialog.run()) == ResponseType::Accept {
            if let Some(filename) = dialog.get_filename() {
                if let Ok(file) = File::open(filename) {
                    if let Ok(data) =
                        serde_json::de::from_reader::<_, HashMap<usize, PatternContainerData>>(file)
                    {
                        for (_, container) in data.iter() {
                            self.add_new_container_no_increment(container.clone());
                            if let Some(comp) = self
                                .pattern_containers
                                .get(&self.model.current_container_id)
                            {
                                for (_, pattern) in container.patterns.iter() {
                                    comp.stream()
                                        .emit(PatternContainerMsg::AddPattern(pattern.clone()));
                                }
                            }
                            self.model.current_container_id += 1;
                        }
                    }
                }
            }
        }
        dialog.emit_close();
    }
}

impl Update for SLMController {
    type Model = SLMControllerModel;
    type ModelParam = ();
    type Msg = SLMControllerMsg;

    fn model(_: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        SLMControllerModel {
            pattern_data_containers: HashMap::new(),
            current_container_id: 0,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Quit => gtk::main_quit(),
            AddTab => self.add_new_container(PatternContainerData::default()),
            RemoveTab => {
                let tab_id = self.container_notebook.get_property_page() as usize;
                let mut ids = self
                    .model
                    .pattern_data_containers
                    .iter()
                    .map(|(x, _)| x)
                    .collect::<Vec<_>>();
                ids.sort();
                if tab_id < ids.len() {
                    self.remove_container(*ids[tab_id]);
                }
            }
            RemoveAllTabs => self.remove_all_containers(),
            SaveContainers => self.save_containers(),
            LoadContainers => self.load_containers(),
            RemoveController(c_id, p_id) => {
                if let Some(container) = self.model.pattern_data_containers.get_mut(&c_id) {
                    container.patterns.remove(&p_id);
                }
            }
            AddController(c_id, p_id, data) => {
                if let Some(container) = self.model.pattern_data_containers.get_mut(&c_id) {
                    container.patterns.insert(p_id, data);
                }
            }
            UpdatePatternL(c_id, p_id, x) => update_from_pattern_spinner!(self, c_id, p_id, x, l),
            UpdatePatternA(c_id, p_id, x) => update_from_pattern_spinner!(self, c_id, p_id, x, a),
            UpdatePatternKx(c_id, p_id, x) => {
                update_from_pattern_spinner!(self, c_id, p_id, x, k, 0)
            }
            UpdatePatternKy(c_id, p_id, x) => {
                update_from_pattern_spinner!(self, c_id, p_id, x, k, 1)
            }
            UpdatePatternCx(c_id, p_id, x) => {
                update_from_pattern_spinner!(self, c_id, p_id, x, c, 0)
            }
            UpdatePatternCy(c_id, p_id, x) => {
                update_from_pattern_spinner!(self, c_id, p_id, x, c, 1)
            }
            UpdatePatternPhase(c_id, p_id, x) => {
                update_from_pattern_spinner!(self, c_id, p_id, x, phase)
            }
            UpdateContainerCx(c_id, x) => update_from_container_spinner!(self, c_id, x, pos, 0),
            UpdateContainerCy(c_id, x) => update_from_container_spinner!(self, c_id, x, pos, 1),
            UpdateContainerScaleX(c_id, x) => {
                update_from_container_spinner!(self, c_id, x, scale, 0)
            }
            UpdateContainerScaleY(c_id, x) => {
                update_from_container_spinner!(self, c_id, x, scale, 1)
            }
            UpdateContainerTLX(c_id, x) => {
                update_from_container_spinner!(self, c_id, x, top_left, 0)
            }
            UpdateContainerTLY(c_id, x) => {
                update_from_container_spinner!(self, c_id, x, top_left, 1)
            }
            UpdateContainerBRX(c_id, x) => {
                update_from_container_spinner!(self, c_id, x, bottom_right, 0)
            }
            UpdateContainerBRY(c_id, x) => {
                update_from_container_spinner!(self, c_id, x, bottom_right, 1)
            }
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
        let save_button = gtk::Button::new_with_label("Save containers");
        let load_button = gtk::Button::new_with_label("Load containers");
        let delete_button = gtk::Button::new_with_label("Delete current container");
        let delete_all_button = gtk::Button::new_with_label("Delete all containers");
        connect!(
            relm,
            widget,
            connect_delete_event(_, _),
            return (Quit, gtk::Inhibit(false))
        );
        connect!(relm, add_button, connect_clicked(_), AddTab);
        connect!(relm, save_button, connect_clicked(_), SaveContainers);
        connect!(relm, load_button, connect_clicked(_), LoadContainers);
        connect!(relm, delete_button, connect_clicked(_), RemoveTab);
        connect!(relm, delete_all_button, connect_clicked(_), RemoveAllTabs);

        container_control_box.pack_start(&add_button, false, false, 0);
        container_control_box.pack_start(&save_button, false, false, 0);
        container_control_box.pack_start(&load_button, false, false, 0);
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
            relm: relm.clone(),
            pattern_containers: HashMap::new(),
        }
    }
}
