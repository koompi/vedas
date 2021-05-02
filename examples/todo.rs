<<<<<<< HEAD
// use iced::{button, text_input, Element, Sandbox, Settings};
// use std::io::Error;
// use vedas_core::*;

// fn main() -> Result<(), Error> {
//     ToDoApp::run(Settings::default());
//     Ok(())
// }
// // Compnent
// component!(ToDoItem: "ToDoItem" => completed: bool, description: String,remove_btn: button::State);
// message!(ToDoItemMessage, ToggleItem(bool), RemoveItem(bool));
// // App
// component!(ToDoApp: "ToDoApp" => data: Vec<ToDoItem>, input_state: text_input::State, input_value: String, add_btn: button::State);
// message!(
//     ToDoAppMessage,
//     ToggleItemMessage(usize, ToDoItemMessage),
//     InputChanged(String),
//     AddTodo
// );
=======
use iced::{button, text_input, Element, Error, Sandbox, Settings};
use vedas_core::*;

fn main() -> Result<(), Error> {
    ToDoApp::run(Settings::default())
}
// Compnent
component!(ToDoItem: "ToDoItem" => completed: bool, description: String,remove_btn: button::State);
message!(pub ToDoItemMessage, ToggleItem(bool), RemoveItem(bool));
// App
component!(ToDoApp: "ToDoApp" => data: Vec<ToDoItem>, input_state: text_input::State, input_value: String, add_btn: button::State);
message!(
    pub ToDoAppMessage,
    ToggleItemMessage(usize, ToDoItemMessage),
    InputChanged(String),
    AddTodo
);
>>>>>>> origin/brilliant

// impl ToDoItem {
//     f!(new, Self, { Self::default() });
//     f_ref_mut_self!(self, update, message: ToDoItemMessage, {
//         match message {
//             ToDoItemMessage::ToggleItem(b) => self.completed = b,
//             ToDoItemMessage::RemoveItem(_) => {}
//         }
//     });
//     f_ref_mut_self!(self, view, Element<ToDoItemMessage>, {
//         row!()
//             .spacing(10)
//             .padding(10)
//             .push(
//                 checkbox!(
//                     self.completed,
//                     &self.description,
//                     ToDoItemMessage::ToggleItem
//                 )
//                 .width(fill!()),
//             )
//             .push(
//                 btn!(&mut self.remove_btn, "x")
//                     .on_press(ToDoItemMessage::RemoveItem(false))
//                     .width(units!(50)),
//             )
//             .into()
//     });
// }
// impl Sandbox for ToDoApp {
//     type Message = ToDoAppMessage;
//     f!(new, Self, {
//         let mut s = Self::default();
//         let mut data: Vec<ToDoItem> = Vec::new();
//         for _ in 0..5 {
//             let mut item = ToDoItem::default();
//             item.description = "Hello".to_string();
//             data.push(item)
//         }
//         s.data = data;
//         s
//     });
//     f_ref_self!(self, title, String, { String::from("Hello") });
//     f_ref_mut_self!(self, update, message: ToDoAppMessage, {
//         match message {
//             ToDoAppMessage::ToggleItemMessage(i, m) => match m {
//                 ToDoItemMessage::ToggleItem(_) => {
//                     self.data.get_mut(i).unwrap().update(m);
//                 }
//                 ToDoItemMessage::RemoveItem(_) => {
//                     self.data.remove(i);
//                 }
//             },
//             ToDoAppMessage::InputChanged(s) => self.input_value = s,
//             ToDoAppMessage::AddTodo => {
//                 let mut new_item = ToDoItem::default();
//                 new_item.description = self.input_value.clone();
//                 self.input_value.clear();
//                 self.data.push(new_item);
//             }
//         }
//     });
//     f_ref_mut_self!(self, view, Element<ToDoAppMessage>, {
//         container!(self
//             .data
//             .iter_mut()
//             .enumerate()
//             .fold(col!().spacing(10), |c, (i, item)| {
//                 c.push(
//                     item.view()
//                         .map(move |m| ToDoAppMessage::ToggleItemMessage(i, m)),
//                 )
//                 .into()
//             })
//             .push(txt_input!(
//                 &mut self.input_state,
//                 "Add to do here!",
//                 &self.input_value,
//                 ToDoAppMessage::InputChanged
//             ))
//             .push(btn!(&mut self.add_btn, "Add").on_press(ToDoAppMessage::AddTodo)))
//         .width(fill!())
//         .center_x()
//         .into()
//     });
// }

fn main() {
    
}