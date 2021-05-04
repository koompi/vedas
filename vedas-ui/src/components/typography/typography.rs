use crate::helper::get_prefix_class;

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct Props {
    #[prop_or(String::from("article"))]
    pub component: String,
    #[prop_or_default]
    pub aria_label: String,
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

pub struct Typography {
    props: Props,
}

impl Component for Typography {
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
        let Props { id, class, key, node_ref, aria_label, children, component } = self.props;
        let component = component.to_owned();
        let prefix_class = get_prefix_class(Some("typography"), None);
        let mut classes = Classes::from(prefix_class);
        classes.extend(class);

        html! {
            <component
                class=classes
                id=id
                key=key
                ref=node_ref
                aria-label=aria_label
            >
                { children.to_owned() }
            </component>
        }
    }
} 