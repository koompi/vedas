use iced::{button, Element, Sandbox, Settings};
use std::io::Error;
use vedas_core::*;

fn main() -> Result<(), Error> {
    ToDoApp::run(Settings::default());
    Ok(())
}
// Compnent
application!(ToDoItem: "ToDoItem" => completed: bool, description: String,remove_btn: button::State);
message!(ToDoItemMessage, ToggleItem: bool, RemoveItem: bool);
// App
application!(ToDoApp: "ToDoApp" => data: Vec<ToDoItem>);
message!(ToDoAppMessage, ToggleItemMessage: usize;ToDoItemMessage);

impl ToDoItem {
    f!(new, Self, { Self::default() });
    f_ref_mut_self!(self, update, message: ToDoItemMessage, {
        mtch!(message, ToDoItemMessage,
        <ToDoItemMessage>::ToggleItem(b) => {
            self.completed = b
        },
        <ToDoItemMessage>::RemoveItem(b) => {}
        )
    });
    f_ref_mut_self!(self, view, Element<ToDoItemMessage>, {
        row!()
            .push(checkbox!(
                self.completed,
                &self.description,
                ToDoItemMessage::ToggleItem
            ))
            .push(btn!(&mut self.remove_btn, "x").on_press(ToDoItemMessage::RemoveItem(false)))
            .into()
    });
}
impl Sandbox for ToDoApp {
    type Message = ToDoAppMessage;
    f!(new, Self, {
        let mut s = Self::default();
        let mut data: Vec<ToDoItem> = Vec::new();
        for _ in 0..5 {
            let mut item = ToDoItem::default();
            item.description = "Hello".to_string();
            data.push(item)
        }
        s.data = data;
        s
    });
    f_ref_self!(title, String, { String::from("Hello") });
    f_ref_mut_self!(self, update, message: ToDoAppMessage, {
        mtch!(message, ToDoAppMessage, <ToDoAppMessage>::ToggleItemMessage(i,m) => {
            mtch!(m, ToDoItemMessage, <ToDoItemMessage>::ToggleItem(b) => {
                if let Some(item) = self.data.get_mut(i) {
                    item.update(m);
                }
            },
            <ToDoItemMessage>::RemoveItem(b) => {
                self.data.remove(i);
            }
        )
        })
    });
    f_ref_mut_self!(self, view, Element<ToDoAppMessage>, {
        self.data
            .iter_mut()
            .enumerate()
            .fold(col!(), |c, (i, item)| {
                c.push(
                    item.view()
                        .map(move |m| ToDoAppMessage::ToggleItemMessage(i, m)),
                )
                .into()
            })
            .into()
    });
}
