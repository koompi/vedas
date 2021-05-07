use crate::utils::random_id;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::utils;
pub struct AccordionItem {
    props: AccordionPropItem,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    id: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct AccordionPropItem {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub active_key: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub content: String,
    #[prop_or_default]
    pub on_click: Callback<u32>,
}
pub enum Msg {
    OnClick(u32),
}
impl Component for AccordionItem {
    type Message = Msg;
    type Properties = AccordionPropItem;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let id = props.id.as_ref().cloned().unwrap_or_else(|| random_id());
        Self {
            props,
            link,
            node_ref: Default::default(),
            id,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnClick(index) => {
                self.props.on_click.emit(index);
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
            <div class="accordion-item">
            <button id=self.id  aria-expanded="false"><span class="accordion-title">{self.props.title.clone()}</span><span class="icon" aria-hidden="true"></span></button>
            <div class="accordion-content">
              <p>{self.props.content.clone()}</p>
            </div>
          </div>
        }
    }
}