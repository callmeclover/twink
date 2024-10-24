use anyhow::Result;
use iced::{
    widget::{button, column},
    Element,
};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, sync::mpsc, thread};

pub fn main() -> Result<()> {
    iced::run("Twink", App::update, App::view)?;

    Ok(())
}

/// Event messages.
#[derive(Debug, Clone)]
enum Message {
    AudioHandlerResume,
    AudioHandlerPause,
}

#[derive(Debug, Clone)]
enum AudioHandlerEvent {
    Resume,
    Pause,
}

/// The holder for app data.
struct App {
    //version: u8,
    audio_handler: Sink, //mpsc::Sender<AudioHandlerEvent>,
                         //input: String,
}

impl Default for App {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::new(stream));
        let audio_handler: Sink = Sink::try_new(&stream_handle).unwrap();
        let file: BufReader<File> = BufReader::new(
            File::open("C:\\Users\\hacker man __)\\Music\\Weezer - Weezer\\Hash Pipe.wav").unwrap(),
        );
        let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
        audio_handler.append(source);
        Self {
            //version: 0,
            audio_handler,
            //input: String::default(),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::AudioHandlerPause => {
                self.audio_handler.pause();
            }
            Message::AudioHandlerResume => {
                self.audio_handler.play();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Play").on_press(Message::AudioHandlerResume),
            button("Pause").on_press(Message::AudioHandlerPause)
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
