use adw::{prelude::*, AboutDialog, ActionRow, ApplicationWindow, HeaderBar};
use gtk::{gio, Application, Box, ListBox, Orientation, ShortcutsWindow};

mod uisrc;
use uisrc::MainWindow;

fn main() {
    gio::resources_register_include!("twink.gresource")
        .expect("Failed to register resources.");

    let app: Application = Application::builder()
        .application_id("com.github.Twink")
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
    /*
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
    */
    let window: MainWindow = MainWindow::new(app);

    window.present();
}
