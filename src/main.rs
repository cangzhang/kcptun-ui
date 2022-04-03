use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Text, Settings,
};

pub mod cmd;

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    OnClick,
    Initialize,
}

struct App {
    list_button: button::State,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App {
                list_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("kcptun ui")
    }

    fn update(&mut self, message: Message, _c: &mut Clipboard) -> Command<Message> {
        match message {
            Message::OnClick => Command::none(),
            _ => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let col = Column::new()
            .max_width(600)
            .spacing(10)
            .padding(10)
            .align_items(Align::Center)
            .push(Button::new(&mut self.list_button, Text::new("Home")).on_press(Message::OnClick));

        col.into()
    }
}
