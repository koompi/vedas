use iced::{executor, Application, Color, Command};
use vedas_core::*;
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

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        container!(
            fill!(),
            container!(fill!(), units!(50), col!(fill!()).push(text!("Hi")))
                .style(HeaderContainer)
                .center_y()
        )
        .into()
    }
}

style_container!(HeaderContainer {
    text_color: Some(Color::WHITE),
    background: Some(iced::Background::Color(Color::BLACK)),
    border_radius: 0.,
    border_width: 1.,
    border_color: Color::BLACK
});
