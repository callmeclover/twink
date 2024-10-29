use gtk::{glib, prelude::*, Application, ApplicationWindow, Button};
use rodio::Sink;
use std::{
    fs::File,
    sync::{Arc, LazyLock, Mutex, MutexGuard},
};

static AUDIO_HANDLER: LazyLock<Arc<Mutex<Sink>>> = LazyLock::new(|| {
    Arc::new(Mutex::new({
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Sink::try_new(&stream_handle).unwrap()
    }))
});

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
    // Create a button with label and margins
    let button: Button = Button::builder()
        .label("Add song to queue")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button: &Button| {
        let mut handler: MutexGuard<'_, Sink> = AUDIO_HANDLER.lock().unwrap();
        let file: File = File::open(
            "E:\\Music\\Weezer\\Weezer (Green Album) (2001-05-07)\\1.3 - Hash Pipe.flac",
        )
        .expect("Cannot open file");
        let reader: BufReader<File> = BufReader::new(file);
        let source: Decoder<BufReader<File>> = Decoder::new(reader).unwrap();
        *handler.append(source);

        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(&format!("Twink {}", env!("CARGO_PKG_VERSION")))
        .child(&button)
        .build();

    // Present window
    window.present();
}
