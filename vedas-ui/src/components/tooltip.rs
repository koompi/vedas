use yew::prelude::*;
use super::to_option_string;
pub struct Tooltip {
    link: ComponentLink<Self>,
    props: Props,
    tooltip_ref: NodeRef
}
#[derive(Debug, Clone, PartialEq)]
enum TooltipDirection {
    Top,
    Left,
    Right,
    Bottom
}
impl Default for TooltipDirection {
    fn default() -> Self {
        TooltipDirection::Top
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or("Please Fill tooltip Text".to_string())]
    pub text: String,
    #[prop_or_default] 
    pub direction: String,
    pub children: Children
}
impl From<String> for TooltipDirection {
    fn from(data_str: String) -> Self {
        match data_str.as_str() {
            "top" => TooltipDirection::Top,
            "left" => TooltipDirection::Left,
            "right" => TooltipDirection::Right,
            "bottom" => TooltipDirection::Bottom,
            _=> TooltipDirection::Top
        }
    }
}

impl Component for Tooltip {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            tooltip_ref:Default::default()
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
        <span flow="down" tooltip=to_option_string(self.props.text.clone()).unwrap() id=self.props.id class_name=self.props.class_name direction=self.props.direction>
            {self.props.children.clone()}
        </span>
       }
    }

}