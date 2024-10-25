use anyhow::Result;
use iced::{
    widget::{button, column},
    Element,
};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

pub fn main() -> Result<()> {
    iced::run("Twink", App::update, App::view)?;

    Ok(())
}

/// Event messages.
#[derive(Debug, Clone)]
enum Message {
    Resume,
    Pause,
    Enqueue(String),
}

/// The holder for app data.
struct App {
    //version: u8,
    audio_handler: Sink,
    file: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::new(stream));
        let audio_handler: Sink = Sink::try_new(&stream_handle).unwrap();
        
        Self {
            //version: 0,
            audio_handler,
            file: None,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Pause => {
                self.audio_handler.pause();
            }
            Message::Resume => {
                self.audio_handler.play();
            }
            Message::Enqueue(path) => {
                let file: BufReader<File> = BufReader::new(
                    File::open(path).unwrap(),
                );
                let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
                self.audio_handler.append(source);
            },
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Play").on_press(Message::Resume),
            button("Pause").on_press(Message::Pause)
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
