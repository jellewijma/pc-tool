use iced::{executor, Application, Command, Element, Settings, Text};
use std::process::Command as Cmd;
use std::str;

#[derive(Debug, Clone)]
enum Message {
    PingLoaded(Result<String, std::io::Error>),
}

struct Gui {
    ping: Option<String>,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Gui, Command<Message>) {
        (Gui { ping: None }, Command::perform(load_ping(), Message::PingLoaded))
    }

    fn title(&self) -> String {
        String::from("Network Ping")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PingLoaded(result) => {
                self.ping = result.ok();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = match &self.ping {
            Some(ping) => format!("Ping: {}", ping),
            None => String::from("Loading..."),
        };
        Text::new(content).into()
    }
}

async fn load_ping() -> Result<String, std::io::Error> {
    let output = Cmd::new("ping")
        .arg("-c")
        .arg("1")
        .arg("8.8.8.8") // You can replace this with the IP you want to ping
        .output()?;

    let output_str = str::from_utf8(&output.stdout).unwrap();
    let ping_time_line = output_str
        .lines()
        .find(|line| line.contains("time="))
        .unwrap();
    let ping_time = ping_time_line
        .split("time=")
        .nth(1)
        .unwrap()
        .split(' ')
        .nth(0)
        .unwrap();

    Ok(ping_time.to_string())
}

fn main() -> iced::Result {
    Gui::run(Settings::default())
}