use adw::{prelude::*, AboutDialog, ActionRow, ApplicationWindow, HeaderBar};
use gtk::{gio, Application, Box, ListBox, Orientation, ShortcutsWindow};

fn main_window(app: &Application) -> Window {
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .width_request(640)
        .height_request(480)
        .content(&content)
        .show_menubar(true)
        .build();

    let about: AboutDialog = AboutDialog::builder()
        .application_name("Twink")
        .version("0.1.0")
        .release_notes(include_str!("../CHANGELOG.md"))
        .license_type(gtk::License::MitX11)
        .license(include_str!("../LICENSE.md"))
        .developer_name("Clover Johnson (callmeclover)")
        .build();

    let about_action: gio::ActionEntry<ApplicationWindow> = gio::ActionEntry::builder("about")
        .activate(move |window: &ApplicationWindow, _, _| {
            about.present(Some(window));
        })
        .build();

    window.add_action_entries([about_action]);

    let help: ShortcutsWindow = ShortcutsWindow::builder().application(app).transient_for(&window).child(child).build();
    window.set_help_overlay(Some(&help));
}