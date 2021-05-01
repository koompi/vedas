<<<<<<< HEAD
// use iced::{button, Element, Sandbox, Settings};
// use std::io::Error;
// use vedas_core::*;

// fn main() -> Result<(), Error> {
//     Hello::run(Settings::default());
//     Ok(())
// }

// component!(Hello: "Hello" => value: u64, inc_state: button::State, dec_state: button::State);
// message!(HelloMessage, IncBtn, DecBtn);

// impl Sandbox for Hello {
//     type Message = HelloMessage;
//     f!(new, Self, { Self::default() });
//     f_ref_self!(self, title, String, { String::from("Hello") });
//     f_ref_mut_self!(self, update, message: HelloMessage, {
//         match message {
//             HelloMessage::IncBtn => self.value += 1,
//             HelloMessage::DecBtn => {
//                 if self.value > 0 {
//                     self.value -= 1
//                 } else {
//                     self.value = 0
//                 }
//             }
//         }
//     });
//     f_ref_mut_self!(self, view, Element<HelloMessage>, {
//         col!()
//             .push(btn!(&mut self.inc_state, "+").on_press(HelloMessage::IncBtn))
//             .push(text!(self.value))
//             .push(btn!(&mut self.dec_state, "-").on_press(HelloMessage::DecBtn))
//             .into()
//     });
// }

fn main() {}
=======
use iced::{button, Element, Error, Sandbox, Settings};
use vedas_core::*;

fn main() -> Result<(), Error> {
    Hello::run(Settings::default())
}

component!(Hello: "Hello" => value: u64, inc_state: button::State, dec_state: button::State);
message!(pub HelloMessage, IncBtn, DecBtn);

impl Sandbox for Hello {
    type Message = HelloMessage;
    f!(new, Self, { Self::default() });
    f_ref_self!(self, title, String, { String::from("Hello") });
    f_ref_mut_self!(self, update, message: HelloMessage, {
        match message {
            HelloMessage::IncBtn => self.value += 1,
            HelloMessage::DecBtn => {
                if self.value > 0 {
                    self.value -= 1
                } else {
                    self.value = 0
                }
            }
        }
    });
    f_ref_mut_self!(self, view, Element<HelloMessage>, {
        // col!()
        //     .push(btn!(&mut self.inc_state, "+").on_press(HelloMessage::IncBtn))
        //     .push(text!(self.value))
        //     .push(btn!(&mut self.dec_state, "-").on_press(HelloMessage::DecBtn))
        //     .into()
        // container!(col!(fill!())
        //     .push(text!("Counter: {data}"))
        //     .push(btn!(&mut self.inc_state, "+").on_press(HelloMessage::IncBtn))
        //     .push(btn!(&mut self.dec_state, "+").on_press(HelloMessage::DecBtn)))
        // .into()
        container!(col!()
            // .push(text!("Counter: {data}"))
            // .push(btn!(&mut self.inc_state, "+"))
            // .push(btn!(&mut self.dec_state, "+"))
            // .push(
            //     col!()
            //         .push(btn!(&mut self.inc_state, "+"))
            //         .push(text!(" {data.name} "))
            // ) //
            .push(container!(text!("Counter: {data}")))
            .push(
                col!()
                    .push(container!(col!()
                        .push(btn!(&mut self.inc_state, "+"))
                        .push(text!(" {data.name} "))))
                    .push(btn!(&mut self.dec_state, "+"))
                    .push(text!(" {data.name} "))
            ))
        .into()
    });
}
>>>>>>> origin/brilliant
