use iced::widget::{button, column, text, Column};
use iced::Center;

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}

fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{net::Ipv4Addr, time::Duration};

    #[test]
    fn ping_8888() {
        ping::ping(
            std::net::IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            Some(Duration::from_secs(2)),
            None,
            None,
            None,
            None,
        )
        .expect("failed to ping");
    }
}
