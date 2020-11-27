use iced::{executor, Application, Column, Command, Container};
#[derive(Debug, Clone)]
pub struct VedasSDK;

#[derive(Debug, Clone)]
pub enum VedasMsg {}

impl Application for VedasSDK {
    type Executor = executor::Default;

    type Message = VedasMsg;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Vedas SDK")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let child = Column::new();
        Container::new(child).into()
    }
}
