use crate::Message;
use iced::{
    widget::{button, column, container, scrollable, text, text_input},
    window, Center, Element, Fill, Theme,
};

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub scale_input: String,
    pub current_scale: f64,
    pub theme: Theme,
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: "Window".to_string(),
            scale_input: "1.0".to_string(),
            current_scale: 1.0,
            theme: Theme::TokyoNightStorm.clone(),
        }
    }

    pub fn view(&self, id: window::Id) -> Element<Message> {
        let scale_input = column![
            text("Window scale factor:"),
            text_input("Window Scale", &self.scale_input)
                .on_input(move |msg| { Message::ScaleInputChanged(id, msg) })
                .on_submit(Message::ScaleChanged(id, self.scale_input.to_string()))
        ];

        let title_input = column![
            text("Window title:"),
            text_input("Window Title", &self.title)
                .on_input(move |msg| { Message::TitleChanged(id, msg) })
                .id(format!("input-{id}"))
        ];

        let new_window_button = button(text("New Window")).on_press(Message::OpenWindow);

        let content = scrollable(
            column![scale_input, title_input, new_window_button]
                .spacing(50)
                .width(Fill)
                .align_x(Center),
        );

        container(content).center_x(200).into()
    }
}

/* TODO: Find a way to reimplement audio controls

    Old code:
    ```
    column![
            row![
                button("Play").on_press(Message::Resume),
                button("Pause").on_press(Message::Pause),
                button("Stop").on_press(Message::Stop),
            ]
            .spacing(10),
            row![
                text_input("", &self.selected_file).on_input(Message::TextUpdated),
                button("Queue Good Music").on_press(Message::Enqueue(self.selected_file.clone())),
            ],
            slider(0.0..=self.duration, self.position, Message::Seek),
            text(format!(
                "{}/{}",
                format_duration(Duration::from_secs_f32(self.position)),
                format_duration(Duration::from_secs_f32(self.duration))
            ))
            .size(20),
            vertical_slider(0.0..=1.0, self.volume, Message::Volume)
                .height(100)
                .step(0.01),
            text(format!("{}%", (self.volume * 100.0).round())).size(20),
        ]
        .spacing(20)
        .padding(20)
        .into()
    ```

*/
