use crate::utils::get_html_element_by_class;
use web_sys::window;
use yew::prelude::*;
use yew::{utils, App};
pub struct Tabs {
    link: ComponentLink<Self>,
    props: TabsProps,
}

struct TabsProps {
    active_key: String,
    default_activekey: String,
    class_name: String,
    id: String,
    code_ref: NodeRef,
    on_select: Callback<MouseEvent>,
    transition: bool,
    children: Children,
}
impl From<Props> for TabsProps {
    fn from(props: Props) -> Self {
        TabsProps {
            active_key: props.active_key,
            default_activekey: props.default_activekey,
            class_name: props.class_name,
            id: props.id,
            code_ref: props.code_ref,
            on_select: props.on_select,
            transition: props.transition,
            children: props.children,
        }
    }
}
#[derive(Clone, Properties)]
pub struct Props {
    /// Type botton style. Default `Palette::Standard`
    #[prop_or(String::from("unknown"))]
    pub active_key: String,
    #[prop_or(String::from("unknown"))]
    pub default_activekey: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// Select  event for Tabs. Required
    pub on_select: Callback<MouseEvent>,
    pub transition: bool,
    pub children: Children,
}
pub enum Msg {
    OnSelect(MouseEvent),
}

impl Component for Tabs {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: TabsProps::from(props),
        }
    }
    fn rendered(&mut self, _first_render: bool) {
        let tab_class = get_html_element_by_class("tab", 0);
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnSelect(mouse_event) => {
                self.props.on_select.emit(mouse_event);
            }
        };
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = TabsProps::from(props);
        true
    }
    fn view(&self) -> Html {
        html! {
           <div class="tab" onclick=self.link.callback(Msg::OnSelect)> {self.props.children.clone()} </div>
        }
    }
}
