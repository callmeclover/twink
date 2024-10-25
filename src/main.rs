mod utils;

use anyhow::Result;
use iced::{
    time::{self, Duration},
    widget::{button, column, row, slider, text, vertical_slider},
    Element, Subscription, Theme,
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{fs::File, io::BufReader};
use utils::format_duration;

pub fn main() -> Result<()> {
    iced::application("Twink", App::update, App::view)
        .subscription(App::subscription)
        .run()?;
    Ok(())
}

/// Event messages.
#[derive(Debug, Clone)]
enum Message {
    /// Resumes the audio handler.
    Resume,
    /// Pauses audio handler.
    Pause,
    /// Pauses audio and clears the queue.
    Stop,
    /// Adds a file from a path to the queue.
    Enqueue(String),
    /// Seeks through the audio.
    Seek(f32),
    /// Sets the volume of the audio handler.
    Volume(f32),
    /// Ran every 100ms to update some UI components.
    Tick,
}

/// The holder for app data.
struct App {
    audio_handler: Sink,
    file: Option<String>,
    duration: f32,
    position: f32,
    volume: f32,
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
            volume: 1.0,
            is_playing: false,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Resume => {
                self.audio_handler.play();
                self.is_playing = true;
            }
            Message::Pause => {
                self.audio_handler.pause();
                self.is_playing = false;
            }
            Message::Stop => {
                self.audio_handler.clear();
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
            Message::Volume(volume) => {
                self.audio_handler.set_volume(volume);
                self.volume = volume;
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
                )),
                button("Stop").on_press(Message::Stop)
            ]
            .spacing(10),
            slider(0.0..=self.duration, self.position, Message::Seek),
            vertical_slider(0.0..=1.0, self.volume, Message::Volume),
            text(format!(
                "{}/{}",
                format_duration(Duration::from_secs_f32(self.position)),
                format_duration(Duration::from_secs_f32(self.duration))
            ))
            .size(20)
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
