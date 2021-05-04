use yew::prelude::*;
use yewtil::NeqAssign;
use crate::constants::ENTER_CODE;

pub struct TransButton {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub auto_focus: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub aria_label: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub on_click: Callback<Option<MouseEvent>>,
    pub children: Children,
}

#[derive(Debug, Clone)]
pub enum Msg {
    OnClicked(MouseEvent),
    OnKeyDown(KeyboardEvent),
    OnKeyUp(KeyboardEvent),
}

impl Component for TransButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender { 
        use Msg::*;
        let mut res = false;

        match msg {
            OnClicked(e) => if !self.props.disabled {
                self.props.on_click.emit(Some(e));
                res = true;
            },
            OnKeyDown(e) => if e.key_code() == ENTER_CODE {
                e.prevent_default();
            },
            OnKeyUp(e) => if e.key_code() == ENTER_CODE {
                if !self.props.disabled {
                    self.props.on_click.emit(None);
                    res = true;
                }
            }
        }
        res
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div
                role="button"
                tabindex=0
                class=self.props.class.to_owned()
                ref=self.props.node_ref.to_owned(),
                onclick=self.link.callback(Msg::OnClicked)
                onkeyup=self.link.callback(Msg::OnKeyUp),
                onkeydown=self.link.callback(Msg::OnKeyDown),
                aria-label=self.props.aria_label
            >
                { self.props.children.clone() }
            </div>
        }
    }
}