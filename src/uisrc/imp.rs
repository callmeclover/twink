use std::cell::Cell;
use adw::ActionRow
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate, subclass::prelude::*, prelude::*};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/mainwindow.ui")]
pub struct Window {
    #[template_child]
    pub content_box: TemplateChild<Box>,
    #[template_child]
    pub content_box: TemplateChild<List>,
    #[template_child]
    pub button: TemplateChild<ActionRow>,
    pub number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_button_clicked(&self, button: &ActionRow) {
        let number_increased = self.number.get() + 1;
        self.number.set(number_increased);
        button.set_label(&number_increased.to_string())
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}