use yew::prelude::*;
use yew::services::ConsoleService;
use std::borrow::Cow;
use gloo::{timers::callback::Timeout};
use crate::utils::get_html_element_by_class;
pub struct Toast {
    props: ToastProps,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
}

 pub enum Msg {
    OnClose,
    OnOpen
}
#[derive(Properties, Clone)]
pub struct ToastProps {
    #[prop_or_default]
    pub show: bool,
    #[prop_or(5000)]
    pub timeout_ms: u32,
    #[prop_or_default]
    pub header: String,
    #[prop_or_default]
    pub body: String,
    #[prop_or_default]
    pub onopen: Callback<()>,
    #[prop_or_default]
    pub onclose: Callback<()>
}

impl Component for Toast {
    type Message = Msg;
    type Properties = ToastProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            props,
            link,
            node_ref: Default::default()
        }
    }
    fn update(&mut self,_msg: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props =  props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="toast" >
            <div class="">
              <strong class="">{self.props.header.clone()}</strong>
              <small class="">{"5 mins ago"}</small>
              <button type="button" class="button" data-dismiss="toast">{"&#x2715"}</button>
            </div>
            <div class="toast-body">
              {self.props.body.clone()} 
            </div>
          </div>
        }
    }
    fn rendered(&mut self, _first_render: bool)  {
        
            if self.props.show   {
                let toast_ele = get_html_element_by_class("toast", 0);
                toast_ele.set_class_name("toast show");
                ConsoleService::log(&format!("{}", self.props.timeout_ms));
                Timeout::new(self.props.timeout_ms, move || {
                    toast_ele.set_class_name("toast");
                    ConsoleService::log("run after 3000 ms");
                }).forget();
            }else {
                {}
            }

    }

}
