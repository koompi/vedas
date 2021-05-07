use gloo::events::EventListener;
use wasm_bindgen::JsValue;
use web_sys::{HtmlButtonElement, HtmlElement};
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::utils;
pub struct Accordion {
    props: AccordionProp,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}

#[derive(Clone, Properties, PartialEq)]
pub struct AccordionProp {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub active_key: String,
    #[prop_or_default]
    pub on_select: Callback<String>,
    pub children: Children,
}
pub enum Msg {
    OnSelect(String),
}
impl Component for Accordion {
    type Message = Msg;
    type Properties = AccordionProp;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node_ref: Default::default(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnSelect(index) => {
                self.props.on_select.emit(index);
                true
            }
        }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        html! {
            <div class="accordion_container">
            <div id=self.props.id
            acitve_key=self.props.active_key
            class=("accordion", self.props.class_name.clone())
            onclick=self.link.callback(|_| Msg::OnSelect("0".to_string())) >
            {self.props.children.clone()}</div>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let items = utils::document()
                .query_selector_all(".accordion button")
                .unwrap();
            for i in 0..items.length() {
                let node: JsValue = items.item(i).unwrap().into();
                let node_inner: HtmlButtonElement = node.into();

                let item_clone = items.clone();
                EventListener::new(&node_inner, "click", move |event| {
                    let current_target: JsValue = event.current_target().unwrap().into();
                    let current_ele: HtmlButtonElement = current_target.into();
                    let item_toggle = current_ele.get_attribute("aria-expanded").unwrap();
                    for j in 0..item_clone.length() {
                        let node_inside: JsValue = item_clone.item(j).unwrap().into();
                        let each_node: HtmlButtonElement = node_inside.into();
                        each_node.set_attribute("aria-expanded", "false").unwrap();
                    }
                    if item_toggle == "false" {
                        current_ele.set_attribute("aria-expanded", "true").unwrap();
                    } else {
                    }
                    ConsoleService::log(&format!(
                        "Collapse message pressed: {:?}  toggle_state:  {}",
                        event.type_(),
                        item_toggle
                    ));
                })
                .forget();
                // let html_input : HtmlButtonElement  = node.into();
            }
            ConsoleService::log(&format!("Item: {:?}", items.length()));
        }
    }
}
