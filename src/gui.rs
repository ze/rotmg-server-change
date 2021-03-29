use iced::{button, Align, Button, Checkbox, Column, Container, Length, Radio, Sandbox, Text};

use crate::registry;

pub struct Gui {
    key: String,
    current_server: Option<String>,
    selected_server: Option<String>,
    use_testing_servers: bool,
    button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    UseTestingServers(bool),
    ChangeSelectedServer(String),
    ButtonPressed,
}

macro_rules! center_x {
    ($e:expr) => {
        Container::new($e)
            .width(Length::Fill)
            .align_x(Align::Center);
    };
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Self {
        let (key, selected_server) = match registry::get_server_value() {
            Ok((s, r)) => {
                let mut v = std::str::from_utf8(&r.bytes).unwrap().to_string();
                v.pop();
                (s, Some(v))
            }
            Err(_) => (
                String::from(""),
                Some(registry::PROD_SERVERS[0].to_string()),
            ),
        };

        Self {
            key,
            current_server: selected_server.clone(),
            selected_server,
            use_testing_servers: false,
            button: button::State::default(),
        }
    }

    fn title(&self) -> String {
        String::from("RotMG Server Changer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::UseTestingServers(value) => self.use_testing_servers = value,
            Message::ChangeSelectedServer(value) => self.selected_server = Some(value),
            Message::ButtonPressed => {
                let selected_value = self.selected_server.as_ref().unwrap();
                match registry::set_server_value(&self.key, selected_value) {
                    Ok(()) => self.current_server = Some(selected_value.clone()),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    }

    fn view(&mut self) -> iced::Element<Self::Message> {
        let label = center_x!(Text::new(match &self.current_server {
            Some(s) => format!("Current Preferred Server: {}", s),
            None => "No Preferred Server Set".to_string(),
        }));

        let prod_checkbox = Checkbox::new(
            self.use_testing_servers,
            "Use Testing Servers",
            Message::UseTestingServers,
        )
        .spacing(10)
        .size(16);

        let servers = if self.use_testing_servers {
            registry::TESTING_SERVERS.iter()
        } else {
            registry::PROD_SERVERS.iter()
        };

        let changer = servers.fold(Column::new().spacing(1), |column, server| {
            column.push(
                Radio::new(
                    *server,
                    *server,
                    self.selected_server.as_ref().map(|s| s.as_str()),
                    |s| Message::ChangeSelectedServer(s.to_owned()),
                )
                .spacing(10)
                .size(16),
            )
        });

        let button_text = Text::new(format!(
            "Set Preferred Server to {}",
            self.selected_server.as_ref().unwrap()
        ));
        let button =
            center_x!(Button::new(&mut self.button, button_text).on_press(Message::ButtonPressed));

        let column = Column::new()
            .spacing(16)
            .push(label)
            .push(prod_checkbox)
            .push(changer)
            .push(button);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .into()
    }
}
