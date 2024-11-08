mod components;
use components::Window;

use gtk::{glib, prelude::*, Application, ApplicationWindow, Box as GtkBox, Button, Orientation};
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, LazyLock},
};

pub static AUDIO_HANDLER: LazyLock<Arc<Sink>> = LazyLock::new(|| {
    Arc::new({
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::new(stream));
        Sink::try_new(&stream_handle).unwrap()
    })
});

const APP_ID: &str = "com.callmeclover.Twink";

fn main() -> glib::ExitCode {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    /*let queue_button: Button = Button::builder()
        .label("Add song to queue")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let play_button: Button = Button::builder()
        .label("Play")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let pause_button: Button = Button::builder()
        .label("Pause")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    queue_button.connect_clicked(|_| {
        let file: File = File::open(
            "/run/media/callmeclover/Main/Music/Weezer/Weezer (Green Album) (2001-04-24)/1.3 - Hash Pipe.flac"
        )
        .expect("Cannot open file");
        let reader: BufReader<File> = BufReader::new(file);
        let source: Decoder<BufReader<File>> = Decoder::new(reader).unwrap();
        AUDIO_HANDLER.append(source);
    });

    play_button.connect_clicked(|_| {
        AUDIO_HANDLER.play();
    });

    pause_button.connect_clicked(|_| {
        AUDIO_HANDLER.pause();
    });

    let elements: GtkBox = GtkBox::new(Orientation::Horizontal, 10);

    elements.append(&queue_button);
    elements.append(&play_button);
    elements.append(&pause_button);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(format!("Twink {}", env!("CARGO_PKG_VERSION")))
        .child(&elements)
        .build();

    window.present();*/
    let window: Window = Window::new(app);
    window.present();
}
