// use crate::page::{
//     AssetsPage, BasicFormPage, ButtonPage, CardPage, CarouselPage, DropDownPage, FormPage,
//     HomePage, LayoutsPage, ModalPage, NavbarPage, SpinnerPage, TextPage,
// };
use vedas_ui::{
    button::Button,
    layouts::{
        container::{AlignItems, Container, Direction, Mode, Wrap},
        item::{Item, ItemLayout},
    },
    modal::Modal,
    styles::{Palette, Size, Style},
};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::utils::document;
use yew::KeyboardEvent;
use yew_prism::Prism;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
pub struct App {
    link: ComponentLink<Self>,
    show_modal: bool,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/button!"]
    ButtonPath,
    #[to = "/layouts!"]
    LayoutsPath,
    #[to = "/navbars!"]
    NavbarPath,
    #[to = "/forms!"]
    FormPath,
    #[to = "/basic-form!"]
    BasicFormPath,
    #[to = "/card!"]
    CardPagePath,
    #[to = "/modal!"]
    ModalPagePath,
    #[to = "/text!"]
    TextPagePath,
    #[to = "/dropdown!"]
    DropDownPath,
    #[to = "/spinner!"]
    SpinnerPath,
    #[to = "/assets!"]
    AssetsPath,
    #[to = "/carousel!"]
    CarouselPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}
pub enum AppMessage {
    Clicked(String),
    OpenModal,
    CloseModal,
    CloseModalByKb(KeyboardEvent),
}
impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            show_modal: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let body_style = document()
            .body()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .style();
        match msg {
            AppMessage::CloseModal => {
                body_style.set_property("overflow", "auto").unwrap();
                self.show_modal = false;
            }
            AppMessage::CloseModalByKb(keyboard_event) => {
                if keyboard_event.key_code() == 27 {
                    body_style.set_property("overflow", "auto").unwrap();
                    self.show_modal = false;
                }
            }
            AppMessage::OpenModal => {
                body_style.set_property("overflow", "hidden").unwrap();
                ConsoleService::log("Button clicked");
                self.show_modal = true;
            }
            AppMessage::Clicked(data) => ConsoleService::log("Button clicked"),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
                    <Container direction=Direction::Row wrap=Wrap::Wrap>
                    <Modal
            header=html!{
                <b>{"Standard modal"}</b>
            }
            header_palette=Palette::Link
            body=html!{
                <div class="body-content">
                    <p>{"this is a modal example"}</p>
                    <Button
                        button_palette= Palette::Info
                        onclick_signal= self.link.callback(|_| AppMessage::CloseModal)
                    >{"Accept"}</Button>
                </div>
            }
            body_style=Style::Outline
            body_palette=Palette::Link
            is_open=self.show_modal
            onclick_signal= self.link.callback(|_| AppMessage::CloseModal)
            onkeydown_signal= self.link.callback(AppMessage::CloseModalByKb)
        />
                        <Button
                        onclick_signal=self.link.callback(move |_| AppMessage::OpenModal)
                        class_name="hello-world"
                        button_palette=Palette::Success
                        button_style=Style::Light
                        button_size=Size::Big > {"Show Message"} </Button>
                    </Container>
                }
    }
}
