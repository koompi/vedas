// use crate::page::{
//     AssetsPage, BasicFormPage, ButtonPage, CardPage, CarouselPage, DropDownPage, FormPage,
//     HomePage, LayoutsPage, ModalPage, NavbarPage, SpinnerPage, TextPage,
// };
use vedas_ui::{
    accordion::Accordion,
    accordion_item::AccordionItem,
    button::Button,
    layouts::{
        container::{AlignItems, Container, Direction, Mode, Wrap},
        item::{Item, ItemLayout},
    },
    modal::Modal,
    slider::Slider,
    styles::{Palette, Size, Style},
    tabs::tabs::Tabs,
    toast::Toast,
    toggle::Switch,
    tooltip::Tooltip,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::utils::document;
use yew::KeyboardEvent;
use yew_router::{prelude::*, route::Route, switch::Permissive};
pub struct App {
    link: ComponentLink<Self>,
    show_modal: bool,
    slider_val: String,
    is_show: bool,
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
#[derive(Clone)]
pub enum HelloMsg {
    Clicked(String),
    OpenModal,
    CloseModal,
    CloseModalByKb(KeyboardEvent),
    OnSelect(MouseEvent),
    OnToggle(bool),
    OnSlider(CustomEvent),
    OnInput(String),
    OnShowToast,
}
impl Component for App {
    type Message = HelloMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            show_modal: false,
            slider_val: String::new(),
            is_show: false,
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
            HelloMsg::CloseModal => {
                body_style.set_property("overflow", "auto").unwrap();
                self.show_modal = false;
                return true;
            }
            HelloMsg::CloseModalByKb(keyboard_event) => {
                if keyboard_event.key_code() == 27 {
                    body_style.set_property("overflow", "auto").unwrap();
                    self.show_modal = false;
                }
                true
            }
            HelloMsg::OnShowToast => {
                self.is_show = true;
                true
            }
            HelloMsg::OnInput(val) => {
                self.slider_val = val;
                return false;
            }
            HelloMsg::OnToggle(is_toggle) => {
                ConsoleService::log(&format!("Toggle clicked: {}", is_toggle));
                false
            }
            HelloMsg::OnSlider(val) => {
                ConsoleService::log(&format!("Slider: {:?}", val.current_target()));
                false
            }
            HelloMsg::OnSelect(e) => false,
            HelloMsg::OpenModal => {
                body_style.set_property("overflow", "hidden").unwrap();
                ConsoleService::log("Button clicked");
                self.show_modal = true;
                true
            }
            HelloMsg::Clicked(data) => {
                ConsoleService::log("Button clicked");
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::log("This is the first render");
        }
    }
    fn view(&self) -> Html {
        html! {
         <div>
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
                            onclick_signal= self.link.callback(|_| HelloMsg::CloseModal)
                        >{"Accept"}</Button>
                    </div>
                }
                body_style=Style::Outline
                body_palette=Palette::Link
                is_open=self.show_modal
                onclick_signal= self.link.callback(|_| HelloMsg::CloseModal)
                onkeydown_signal= self.link.callback(HelloMsg::CloseModalByKb)
            />
                            <Button
                            onclick_signal=self.link.callback(move |_| HelloMsg::OpenModal)
                            button_palette=Palette::Danger
                            button_style=Style::Light
                            button_size=Size::Big > {"Show Message"} </Button>
                            // <Tabs transition=false, on_select=self.link.callback(move |e| HelloMsg::OnSelect(e))>
                            //     <div> <span class="text">{"Hello"} </span></div>
                            // </Tabs>

                            <Switch checked=true  on_change=self.link.callback(HelloMsg::OnToggle) label="hello world" />
                            <Slider  on_input=self.link.callback(HelloMsg::OnInput) on_change=self.link.callback(HelloMsg::OnSlider)/>
                            <br/>
                            <Tooltip  text="Top level tooltip"><button class="button"> {"Tooltip"} </button></Tooltip>
                            <Toast timeout_ms=3000 show={self.is_show} header="Hello Wrold" body="Hello World is a computer program used to test"/>
                            <Button
                            onclick_signal=self.link.callback(move |_| HelloMsg::OnShowToast)
                            button_palette=Palette::Danger
                            button_style=Style::Light
                            button_size=Size::Big > {"Show Toast"} </Button><br/>


                        </Container>
                        <Accordion>
                                <AccordionItem title="What is your name?" content="My Name is Ma veasna"/>
                                <AccordionItem title="What is your name?" content="My Name is Ma veasna"/>
                                <AccordionItem title="What is your name?" content="My Name is Ma veasna"/>
                                <AccordionItem title="What is your name?" content="My Name is Ma veasna"/>
                                <AccordionItem title="What is your name?" content="My Name is Ma veasna"/>
                                <AccordionItem title="What is your name?" content="My Name is Ma veasna"/>
                                <AccordionItem title="What is your name?" content="My Name is Ma veasna"/>
                        </Accordion>
                         </div>
        }
    }
}
