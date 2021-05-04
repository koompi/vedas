use crate::constants::{ENTER_CODE, ESC_CODE};

use yew::prelude::*;
use yewtil::NeqAssign;
use yew_assets::editing_assets::{EditingAssets, EditingIcon};
use regex::Regex;

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct Props {
    pub prefix_class: String,
    pub value: String,
    #[prop_or_default]
    pub aria_label: String,
    pub on_save: Callback<String>,
    #[prop_or_default]
    pub on_end: Callback<()>,
    pub on_cancel: Callback<()>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub max_length: usize,
}

#[derive(Debug)]
pub enum Msg {
    OnChange(ChangeData),
    // OnCompositionStart,
    // OnCompositionEnd,
    OnKeyDown(KeyboardEvent),
    OnKeyUp(KeyboardEvent),
    OnBlur(FocusEvent),
}

pub struct Editable {
    node_ref: NodeRef,
    current: String,
    last_keycode: Option<u32>,
    in_composition: bool,
    props: Props,
    link: ComponentLink<Self>
}

impl Editable {
    fn confirm_change(&self) {
        self.props.on_save.emit(self.current.trim().to_owned());
    }
}

impl Component for Editable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let current = props.value.clone();
        Self {
            props, link, current,
            node_ref: NodeRef::default(),
            last_keycode: None,
            in_composition: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        let mut res = false;

        match msg {
            OnChange(e) => if let ChangeData::Value(val) = e {
                let reg = Regex::new(r"/[\n\r]/").unwrap();
                self.current = reg.replace(val, "");
                res = true;
            },
            // OnCompositionStart => self.in_composition = true,
            // OnCompositionEnd => self.in_composition = false,
            OnKeyDown(e) => if !self.in_composition {
                self.last_keycode = Some(e.key_code());
            },
            OnKeyUp(e) => if let Some(last_keycode) = self.last_keycode {
                if last_keycode == e.key_code() && 
                    // !self.in_composition && 
                    !e.ctrl_key() && !e.alt_key() && !e.meta_key() && !e.shift_key() {
                    if e.key_code() == ENTER_CODE {
                        self.confirm_change();
                        self.props.on_end.emit(());
                    } else if e.key_code() == ESC_CODE {
                        self.props.on_cancel.emit(());
                    }
                    res = true;
                }
            },
            OnBlur(_) => {
                self.confirm_change();
                res = true;
            }
        }

        res
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from(vec![
            self.props.prefix_class.to_owned(),
            format!("{}-edit-content", &self.props.prefix_class),
        ]);
        classes.extend(self.props.class);

        html! {
            <div class=classes>
                <textarea 
                    ref=self.node_ref
                    maxlength=self.props.max_length
                    onchange=self.link.callback(Msg::OnChange)
                    onkeydown=self.link.callback(Msg::OnKeyDown)
                    onkeyup=self.link.callback(Msg::OnKeyUp)
                    onblur=self.link.callback(Msg::OnBlur)
                    // compositionstart=self.link.callback(|_| Msg::OnCompositionStart)
                    // compositionend=self.link.callback(|_| Msg::OnCompositionEnd)
                    aria-label=self.props.aria_label.clone()
                >{ self.current.clone() }</textarea>
                <EditingAssets icon=EditingIcon::Check class_name=format!("{}-edit-content-confirm", self.props.prefix_class.as_str()) />
            </div>
        }
    }
}