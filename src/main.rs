use iced::{
    button, text_input, Application, Button, Clipboard, Column, Command, Element, Text, TextInput,
};
use std::process::Command as Cmd;
use std::str;

#[derive(Debug, Clone)]
struct ClonableIoError {
    description: String,
}

impl From<std::io::Error> for ClonableIoError {
    fn from(error: std::io::Error) -> Self {
        ClonableIoError {
            description: error.to_string(),
        }
    }
}

// Now you can use `ClonableIoError` in your `Result` type:
#[derive(Debug, Clone)]
enum Message {
    PingIpChanged(String),
    PingButtonPressed,
    PingLoaded(Result<String, ClonableIoError>),
}

struct Gui {
    ping_ip: String,
    ping_ip_state: text_input::State,
    ping_button: button::State,
    ping: Option<String>,
}

impl iced::Application for Gui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Gui, Command<Message>) {
        (
            Gui {
                ping_ip: String::new(),
                ping_ip_state: text_input::State::new(),
                ping_button: button::State::new(),
                ping: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Network Ping")
    }

    fn view(&mut self) -> Element<Message> {
        let input = TextInput::new(
            &mut self.ping_ip_state,
            "Enter IP...",
            &self.ping_ip,
            Message::PingIpChanged,
        );

        let button = Button::new(&mut self.ping_button, Text::new("Ping"))
            .on_press(Message::PingButtonPressed);

        let content = match &self.ping {
            Some(ping) => format!("Ping: {}", ping),
            None => String::from("Loading..."),
        };

        Column::new()
            .push(input)
            .push(button)
            .push(Text::new(content))
            .into()
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::PingIpChanged(ping_ip) => {
                self.ping_ip = ping_ip;
            }
            Message::PingButtonPressed => {
                return Command::perform(load_ping(self.ping_ip.clone()), Message::PingLoaded);
            }
            Message::PingLoaded(Ok(ping)) => {
                self.ping = Some(ping);
            }
            Message::PingLoaded(Err(error)) => {
                self.ping = Some(format!("Error: {}", error.description));
            }
        }

        Command::none()
    }
}

async fn load_ping(ip: String) -> Result<String, ClonableIoError> {
    let output = Cmd::new("ping")
        .arg("-i")
        .arg("0.1") // 100 milliseconds
        .arg("-w")
        .arg("30") // 30 seconds
        .arg(ip)
        .output()
        .map_err(|e| ClonableIoError {
            description: e.to_string(),
        })?;

    let output_str = str::from_utf8(&output.stdout).unwrap();

    if output.status.success() {
        Ok(output_str.to_string())
    } else {
        Err(ClonableIoError {
            description: output_str.to_string(),
        })
    }
}

fn main() -> iced::Result {
    Gui::run(iced::Settings::default())
}
