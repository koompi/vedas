use yew::prelude::*;
use yewtil::NeqAssign;
use super::button::{self, Button};
use crate::utils::Size;
use crate::helper::{get_prefix_class, get_prefix_concat_with};

pub struct ButtonGroup {
    _link: ComponentLink<Self>,
    props: Props,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub button_props: Option<button::Props>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub key: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    pub children: ChildrenWithProps<Button>,
}

impl Component for ButtonGroup {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props, _link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let prefix_class = get_prefix_class(Some("btn-group"), None);
        let mut classes = vec![
            prefix_class.clone(),
            get_prefix_concat_with(&prefix_class, size),
        ];
        classes.push(self.props.class.clone());

        html! {
            <div
                id=self.props.id
                class=classes
                key=self.props.key.clone()
                ref=self.props.node_ref.clone()
            >{
                for self.props.children.iter().map(|mut child| {
                    if let Some(btn_props) = &self.props.button_props {
                        child.props = btn_props.to_owned();
                    }
                    child
                })
            }</div>
        }
    }
}