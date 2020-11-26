use iced::{button, Element, Sandbox, Settings};
use std::io::Error;
#[macro_use]
use vedas_core::*;
// #[macro_use]
// use vedas_core::macros::application;
// #[macro_use]
// use vedas_core::macros::btn;
// #[macro_use]
// use vedas_core::macros::column;
// #[macro_use]
// use vedas_core::macros::message;
// #[macro_use]
// use vedas_core::macros::mtch;
// #[macro_use]
// use vedas_core::macros::text;
fn main() -> Result<(), Error> {
    Hello::run(Settings::default());
    Ok(())
}

application!(Hello: "Hello" => value: u64, inc_state: button::State, dec_state: button::State);
message!(HelloMessage, IncBtn, DecBtn);

impl Sandbox for Hello {
    type Message = HelloMessage;
    f!(new, Self, { Self::default() });
    f_ref_self!(title, String, { String::from("Hello") });
    f_ref_mut_self!(self, update, message: HelloMessage, {
        mtch!(message, HelloMessage, IncBtn => {
            self.value += 1;
        }, DecBtn => {
            if self.value > 0 { self.value -= 1 } else { self.value = 0};
        });
    });
    f_ref_mut_self!(self, view, Element<HelloMessage>, {
        col!()
            .push(btn!(&mut self.inc_state, "+").on_press(HelloMessage::IncBtn))
            .push(text!(self.value))
            .push(btn!(&mut self.dec_state, "-").on_press(HelloMessage::DecBtn))
            .into()
    });
}
