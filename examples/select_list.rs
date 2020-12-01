use iced::{scrollable, pick_list, Element, Sandbox, Settings, Length, Text, Align};
use vedas_core::*;
fn main() -> iced::Result<> {
    SelectList::run(Settings::default())
}
message_cpeq!(Language, Rust, C, Python, Javascript, Ruby);
message!(PickMessage, LanguageSelected(Language));

select_all!(
    Language,
    ALL,
    5,
    Language::Rust,
    Language::C,
    Language::Python,
    Language::Javascript,
    Language::Ruby
);
select_default!(Language, Language::Rust);
select_display!(Language, 
Language::Rust => "Rust programmig",
Language::C => "Elm Programming Language",
Language::Python => "Ruby",
Language::Javascript => "Haskell",
Language::Ruby=> "C Programming");

component!(SelectList: "SelectList" =>  scroll: scrollable::State, pick_list: pick_list::State<Language>, select_language: Language );

impl Sandbox for SelectList {
    type Message = PickMessage;
    f!(new, Self, { Self::default() });
    f_ref_self!(self, title, String, { String::from("Pick_list") });
    f_ref_mut_self!(self, update, message: PickMessage, {
        match message {
            PickMessage::LanguageSelected(lang) => self.select_language = lang,
        }
    });
    f_ref_mut_self!(self, view, Element<PickMessage>,{
        let pick_list = select!(
            &mut self.pick_list,
            &Language::ALL[..],
            Some(self.select_language),
            PickMessage::LanguageSelected
        );
        let  content = scroll!(&mut self.scroll).width(Length::Fill)
        .align_items(Align::Center)
        .spacing(10)
        .push(Text::new("Which is your favorite language?"))
        .push(pick_list);
        container!(Length::Fill, Length::Fill, content).center_x().center_y().into()
    });
}