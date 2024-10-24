use anyhow::Result;
use iced::widget::{button, column, text, Column};

struct Counter {
    value: i64,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement)
        ]
    }
}

enum Message {
    Increment,
    Decrement,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
