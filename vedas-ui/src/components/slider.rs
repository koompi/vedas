use super::to_option_string;
use crate::utils::random_id;
use gloo::events::EventListener;
use wasm_bindgen::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::web_sys::{CustomEvent, Element, HtmlInputElement};
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or(0)]
    pub min: u32,
    #[prop_or(10)]
    pub value: u32,
    #[prop_or(100)]
    pub max: u32,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or(0)]
    pub step: u32,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_change: Callback<CustomEvent>,
    #[prop_or_default]
    pub on_input: Callback<String>,
}

pub struct Slider {
    link: ComponentLink<Self>,
    props: Props,
    input_ref: NodeRef,
    input_listener: Option<EventListener>,
    change_listener: Option<EventListener>,
}
pub enum Msg {
    Changed,
    Input,
}
impl Component for Slider {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let id = props.id.as_ref().cloned().unwrap_or_else(|| random_id());
        Self {
            link,
            props,
            input_ref: Default::default(),
            input_listener: None,
            change_listener: None,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        html! {
          <div >
            <input ref=self.input_ref.clone() value=to_option_string(self.props.value).unwrap() type="range" min=to_option_string(self.props.min).unwrap() max=to_option_string(self.props.max).unwrap() step=to_option_string(self.props.step).unwrap()/>
          </div>
        }
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let element = self.input_ref.cast::<Element>().unwrap();
            let oninput = self.props.on_input.clone();
            self.input_listener = Some(EventListener::new(&element, "input", move |event| {
                let custom_event = JsValue::from(event).unchecked_into::<CustomEvent>();
                let jsvalue: JsValue = custom_event.current_target().unwrap().value_of().into();
                let html_input: HtmlInputElement = jsvalue.into();
                ConsoleService::log(&format!("Value: {:?}", html_input.value()));
                oninput.emit(html_input.value())
            }));
            let onchange = self.props.on_change.clone();
            self.change_listener = Some(EventListener::new(&element, "change", move |event| {
                onchange.emit(JsValue::from(event).unchecked_into::<CustomEvent>())
            }));
        }
    }
}
