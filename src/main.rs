mod app;
pub mod utils;
pub mod windowing;

use anyhow::Result;
use app::App;
use iced::window;

pub fn main() -> Result<()> {
    iced::daemon("Twink", App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .scale_factor(App::scale_factor)
        .run_with(App::new)?;
    Ok(())
}

/// Event messages.
#[derive(Debug, Clone)]
pub enum Message {
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
    /// Sets the selected path.
    TextUpdated(String),
    /// Ran every 100ms to update some UI components.
    Tick,

    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    ScaleInputChanged(window::Id, String),
    ScaleChanged(window::Id, String),
    TitleChanged(window::Id, String),
}
