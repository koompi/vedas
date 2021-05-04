use super::base::{Base, BaseProps};

use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Paragraph {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub base_props: BaseProps,
    // generic props
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub key: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    pub children: Children,
}

pub enum Msg {
    OnClicked(MouseEvent),
}

impl Component for Paragraph {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnClicked(e) => self.props.on_click.emit(e)
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Props { base_props, id, class, key, node_ref, children, .. } = self.props;
        html! {
            <Base
                component="p"
                id=id
                class=class
                key=key
                node_ref=node_ref
                base_props=base_props
            >{ children }</Base>
        }
    }
}
