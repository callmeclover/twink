use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "com.github.callmeclover.Twink";

fn main() -> glib::ExitCode {
    // Create a new application
    let app: Application = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("Twink")
        .build();

    // Present window
    window.present();
}