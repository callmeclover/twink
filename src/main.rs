mod ui;
mod actions;

use ui::window::main_window;

use adw::{prelude::*, AboutDialog, ActionRow, ApplicationWindow, HeaderBar};
use gtk::{gio, Application, Box, ListBox, Orientation, ShortcutsWindow};

fn main() {
    let app: Application = Application::builder()
        .application_id("com.github.callmeclover.Twink")
        .build();

    app.connect_startup(|_| {
        adw::init().unwrap();
    });
    app.connect_activate(on_activate);

    app.set_accels_for_action("win.about", &["<Ctrl>slash"]);
    app.set_accels_for_action("win.show-help-overlay", &["<Ctrl>h"]);

    app.run();
}

fn on_activate(app: &Application) {
    let row: ActionRow = ActionRow::builder()
        .activatable(true)
        .selectable(false)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Clicked!");
    });

    let list: ListBox = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        // the content class makes the list look nicer
        .css_classes(vec![String::from("content")])
        .build();
    list.append(&row);

    // Combine the content in a box
    let content: Box = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(
        &HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("Twink", "Nothing playing"))
            .build(),
    );
    content.append(&list);

    let window = main_window();
    
    window.present();
}
