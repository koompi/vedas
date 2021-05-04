use crate::utils::Target;
use super::base::{Base, BaseProps};

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub target: Target,
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

pub struct Link {
    props: Props,
}

impl Component for Link {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { unimplemented!() }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Props { base_props, id, class, key, node_ref, children, .. } = self.props;

        html! {
            <Base
                component="a"
                id=id
                class=class
                key=key
                node_ref=node_ref
                base_props=base_props
            >
                { children.to_owned() }
            </Base>
        }
    }
}