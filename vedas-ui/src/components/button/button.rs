use super::{Style, Type, Shape};

// use std::time::Duration;
use crate::utils::{Color, Size, Target, Wave};
use crate::helper::{get_prefix_class, get_prefix_concat_with};
use yew::prelude::*;
// use yew::services::timeout::{TimeoutService, TimeoutTask};
use yewtil::NeqAssign;

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
    // task: Option<TimeoutTask>
}

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub shape: Shape,
    #[prop_or_default]
    pub size: Size,
    #[prop_or_default]
    pub loading: Option<u32>,
    #[prop_or_default]
    pub block: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub kind: ButtonType,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub key: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    pub children: Children,
}

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct AnchorProps {
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub target: Target,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonType {
    NativeProps(Type),
    AnchorProps(AnchorProps),
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::NativeProps(Type::default())
    }
}

pub enum Msg {
    Clicked(MouseEvent),
    // StartLoading,
    // StopLoading,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let task = props.loading.map(|time| 
        //     TimeoutService::spawn(
        //         Duration::from_millis(time.into()), 
        //         link.callback(|_| Msg::StopLoading)
        //     )
        // );

        Self {
            props, 
            link,
            // task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(mouse_event) => if !self.props.disabled {
                self.props.on_click.emit(mouse_event)
            },
            // Msg::StopLoading => self.task = None
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Props {
            style,
            color,
            shape,
            size,
            loading,
            block,
            disabled,
            kind,
            class,
            id,
            key,
            node_ref,
            children,
            ..
        } = self.props.clone();

        // let has_loading = self.task.is_some();
        let prefix_class = get_prefix_class(Some("btn"), None);
        let mut classes = vec![
            prefix_class.clone(), 
            get_prefix_concat_with(&prefix_class, style),
            get_prefix_concat_with(&prefix_class, color),
            get_prefix_concat_with(&prefix_class, shape),
            get_prefix_concat_with(&prefix_class, size),
        ];
        if block {
            classes.push(get_prefix_concat_with(&prefix_class, "block"))
        }
        if loading.is_some() {
            classes.push(get_prefix_concat_with(&prefix_class, "loading"))
        }
        classes.push(class.clone());
        match kind {
            ButtonType::AnchorProps(anchor_props) => html! {
                <a
                    id=id
                    class=classes
                    key=key.clone()
                    ref=node_ref.clone()
                    href=anchor_props.href
                    target=anchor_props.target.to_string()
                    onclick=self.link.callback(Msg::Clicked)
                > { children.clone() }
                </a>
            },
            ButtonType::NativeProps(html_type) => {
                let button_node = html! {
                    <button
                        id=id
                        class=classes
                        key=key.clone()
                        ref=node_ref.clone()
                        type=html_type
                        disabled=disabled
                        onclick=self.link.callback(Msg::Clicked)
                    > { children.clone() }
                    </button>
                };

                if !self.props.style.is_unbordered() {
                    html! {
                        <Wave>{ button_node }</Wave>
                    }
                } else {
                    button_node
                }
            }
        }
    }
}