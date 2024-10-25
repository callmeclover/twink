/*use anyhow::Result;
use iced::{
    widget::{button, column, slider},
    Element, Subscription,
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{fs::File, io::BufReader, time::Duration};

pub fn main() -> Result<()> {
    iced::application("Twink", App::update, App::view)
        .subscription(App::subsciption)
        .run()?;

    Ok(())
}

/// Event messages.
#[derive(Debug, Clone)]
enum Message {
    Resume,
    Pause,
    Enqueue(String),
    Seek(f32),
}

/// The holder for app data.
struct App {
    //version: u8,
    audio_handler: Sink,
    file: Option<String>,
    duration: f32,
    position: f32,
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
            duration: 0.0,
            position: 0.0,
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
                let file: BufReader<File> = BufReader::new(File::open(path).unwrap());
                let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
                self.duration = source
                    .total_duration()
                    .unwrap_or(Duration::new(0, 0))
                    .as_secs_f32();
                self.audio_handler.append(source);
            }
            Message::Seek(position) => {
                self.audio_handler
                    .try_seek(Duration::from_secs_f32(position))
                    .unwrap();
                self.position = position;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Play").on_press(Message::Resume),
            button("Pause").on_press(Message::Pause),
            button("Queue Good Music").on_press(Message::Enqueue(
                "E:\\Music\\Weezer\\Weezer (Green Album) (2001-04-24)\\1.3 - Hash Pipe.flac"
                    .to_string()
            )),
            slider(0.0..=self.duration, self.position, Message::Seek)
        ]
        .spacing(20)
        .padding(20)
        .into()
    }

}
*/

use anyhow::Result;
use iced::{
    time,
    widget::{button, column, row, slider, text},
    Element, Subscription,
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{fs::File, io::BufReader, time::Duration};

pub fn main() -> Result<()> {
    iced::application("Twink", App::update, App::view)
        .subscription(App::subscription)
        .run()?;
    Ok(())
}

/// Event messages.
#[derive(Debug, Clone)]
enum Message {
    Resume,
    Pause,
    Enqueue(String),
    Seek(f32),
    Tick, // New message for updating position
}

/// The holder for app data.
struct App {
    audio_handler: Sink,
    file: Option<String>,
    duration: f32,
    position: f32,
    is_playing: bool,
}

impl Default for App {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Box::leak(Box::new(stream));
        let audio_handler: Sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            audio_handler,
            file: None,
            duration: 0.0,
            position: 0.0,
            is_playing: false,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Pause => {
                self.audio_handler.pause();
                self.is_playing = false;
            }
            Message::Resume => {
                self.audio_handler.play();
                self.is_playing = true;
            }
            Message::Enqueue(path) => {
                let file: BufReader<File> = BufReader::new(File::open(&path).unwrap());
                let source: Decoder<BufReader<File>> = Decoder::new(file).unwrap();
                self.duration = source
                    .total_duration()
                    .unwrap_or(Duration::new(0, 0))
                    .as_secs_f32();
                self.audio_handler.append(source);
                self.is_playing = true;
                self.file = Some(path);
            }
            Message::Seek(position) => {
                self.audio_handler
                    .try_seek(Duration::from_secs_f32(position))
                    .unwrap();
                self.position = position;
            }
            Message::Tick => {
                if self.is_playing {
                    self.position = self.audio_handler.get_pos().as_secs_f32();
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![
                button("Play").on_press(Message::Resume),
                button("Pause").on_press(Message::Pause),
                button("Queue Good Music").on_press(Message::Enqueue(
                    "E:\\Music\\Weezer\\Weezer (Green Album) (2001-04-24)\\1.3 - Hash Pipe.flac"
                        .to_string()
                ))
            ].spacing(10),
            slider(0.0..=self.duration, self.position, Message::Seek),
            text(format!("{}/{}", format_duration(Duration::from_secs_f32(self.position)), format_duration(Duration::from_secs_f32(self.duration)))).size(20)
        ]
        .spacing(20)
        .padding(20)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        // Subscribe to periodic ticks to update the audio position
        time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds: u64 = duration.as_secs();
    let hours: u64 = total_seconds / 3600;
    let minutes: u64 = (total_seconds % 3600) / 60;
    let seconds: u64 = total_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}