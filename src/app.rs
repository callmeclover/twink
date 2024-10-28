use crate::{windowing::Window, Message};
use iced::{
    time::{self, Duration},
    widget::{center, horizontal_space, text_input},
    window::{self, Position},
    Element, Point, Subscription, Task, Theme, Vector,
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{collections::BTreeMap, fs::File, io::BufReader};

/// The holder for app data.
pub struct App {
    windows: BTreeMap<window::Id, Window>,
    audio_handler: Sink,
    file: Option<String>,
    selected_file: String,
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
            windows: BTreeMap::new(),
            audio_handler,
            volume: 1.0,
            file: None,
            selected_file: String::new(),
            duration: 0.0,
            position: 0.0,
            is_playing: false,
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
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
                self.duration = 0.0;
                self.position = 0.0;
                self.file = None;
                self.is_playing = false;
            }
            Message::Enqueue(path) => {
                if let Ok(file) = File::open(&path) {
                    let reader: BufReader<File> = BufReader::new(file);
                    let source: Decoder<BufReader<File>> = Decoder::new(reader).unwrap();
                    self.duration = source
                        .total_duration()
                        .unwrap_or(Duration::new(0, 0))
                        .as_secs_f32();
                    self.audio_handler.append(source);
                    self.is_playing = true;
                    self.file = Some(path);
                }
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
            Message::TextUpdated(text) => {
                self.selected_file = text;
            }
            Message::Tick => {
                if self.is_playing {
                    self.is_playing = !self.audio_handler.is_paused();
                    self.position = self.audio_handler.get_pos().as_secs_f32();
                }
            }
            Message::OpenWindow => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                return window::get_position(*last_window)
                    .then(|last_position: Option<Point>| {
                        let position: Position = last_position.map_or(
                            window::Position::Default,
                            |last_position: Point| {
                                window::Position::Specific(last_position + Vector::new(20.0, 20.0))
                            },
                        );

                        let (_id, open) = window::open(window::Settings {
                            position,
                            ..window::Settings::default()
                        });

                        open
                    })
                    .map(Message::WindowOpened);
            }
            Message::WindowOpened(id) => {
                let window: Window = Window::new();
                let focus_input: Task<Message> = text_input::focus(format!("input-{id}"));

                self.windows.insert(id, window);

                return focus_input;
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    return iced::exit();
                }
            }
            Message::ScaleInputChanged(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.scale_input = scale;
                }
            }
            Message::ScaleChanged(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.current_scale = scale
                        .parse::<f64>()
                        .unwrap_or(window.current_scale)
                        .clamp(0.5, 5.0);
                }
            }
            Message::TitleChanged(id, title) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.title = title;
                }
            }
        }
        Task::none()
    }

    pub fn view(&self, id: window::Id) -> Element<Message> {
        self.windows.get(&id).map_or_else(
            || horizontal_space().into(),
            |window| center(window.view(id)).into(),
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Subscribe to periodic ticks to update the audio position
        Subscription::batch([
            time::every(Duration::from_millis(100)).map(|_| Message::Tick),
            window::close_events().map(Message::WindowClosed),
        ])
    }

    pub fn theme(&self, window: window::Id) -> Theme {
        self.windows
            .get(&window)
            .map_or(Theme::default(), |window| window.theme.clone())
    }

    pub fn scale_factor(&self, window: window::Id) -> f64 {
        self.windows
            .get(&window)
            .map_or(1.0, |window| window.current_scale)
    }
}
