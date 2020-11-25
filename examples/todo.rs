#![allow(dead_code, unused_variables, unused_mut, non_camel_case_types)]

#[macro_use]
mod macros;
use iced::{button, Element, Sandbox, Settings};
use std::io::Error;

fn main() -> Result<(), Error> {
    ToDoApp::run(Settings::default());
    Ok(())
}
// Compnent
application!(ToDoItem: "ToDoItem" => completed: bool, description: String);
message!(ToDoItemMessage, ToggleItem: bool);
// App
application!(ToDoApp: "ToDoApp" => data: Vec<ToDoItem>);
message!(ToDoAppMessage, ToggleItemMessage: usize;ToDoItemMessage);

impl ToDoItem {
    f!(new, Self, { Self::default() });
    f_ref_mut_self!(self, update, message: ToDoItemMessage, {
        match message {
            ToDoItemMessage::ToggleItem(b) => self.completed = b,
        }
    });
    f_ref_mut_self!(self, view, Element<ToDoItemMessage>, {
        column!()
            .push(checkbox!(
                self.completed,
                &self.description,
                ToDoItemMessage::ToggleItem
            ))
            .into()
    });
}
impl Sandbox for ToDoApp {
    type Message = ToDoAppMessage;
    f!(new, Self, {
        let mut data: Vec<ToDoItem> = Vec::new();
        for i in 0..5 {
            let item = ToDoItem {
                completed: false,
                description: "Hello".to_string(),
            };
            data.push(item)
        }
        Self { data }
    });
    f_ref_self!(title, String, { String::from("Hello") });
    f_ref_mut_self!(self, update, message: ToDoAppMessage, {
        mtch!(message, ToDoAppMessage, <ToDoAppMessage>::ToggleItemMessage(i,m) => {
            mtch!(m, ToDoItemMessage, <ToDoItemMessage>::ToggleItem(b) => {
                if let Some(item) = self.data.get_mut(i) {
                    item.update(m);
                }
            })
        })
    });
    f_ref_mut_self!(self, view, Element<ToDoAppMessage>, {
        self.data
            .iter_mut()
            .enumerate()
            .fold(column!(), |c, (i, item)| {
                c.push(
                    item.view()
                        .map(move |m| ToDoAppMessage::ToggleItemMessage(i, m)),
                )
            })
            .into()
    });
}
