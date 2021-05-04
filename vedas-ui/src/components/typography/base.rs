use crate::utils::{Color, TransButton};
use crate::helper::{get_prefix_class, get_prefix_concat_with};
use super::{Style, LineStyle};
use super::editable::Editable;
use super::typography::Typography;

use std::time::Duration;
use yew::prelude::*;
use yew::services::timeout::*;
use yewtil::NeqAssign;
use yew_assets::editing_assets::{EditingAssets, EditingIcon};
use clipboard::{ClipboardContext, ClipboardProvider};
use web_sys::HtmlElement;

// const ELLIPSIS_STR: &str = "...";

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct CopyProps {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub on_copy: Callback<()>,
}

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct EditProps {
    #[prop_or_default]
    pub editing: bool,
    #[prop_or_default]
    pub on_start: Callback<()>,
    #[prop_or_default]
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub on_end: Callback<()>,
    #[prop_or_default]    
    pub on_cancel: Callback<()>,
    #[prop_or(127)]
    pub max_length: usize,
    #[prop_or_default]
    pub auto_size: Option<u8>
}

// #[derive(Debug, Clone, Default, PartialEq, Properties)]
// pub struct EllipsisProps {
//     #[prop_or_default]
//     pub rows: u8,
//     #[prop_or_default]
//     pub expandable: bool,
//     #[prop_or(String::from("more"))]
//     pub expand_text: String,
//     #[prop_or_default]
//     pub on_expand: Callback<MouseEvent>,
//     #[prop_or_default]
//     pub on_ellipsis: Callback<bool>,
// }

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct BaseProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub editable: Option<EditProps>,
    #[prop_or_default]
    pub copyable: Option<CopyProps>,
    // #[prop_or_default]
    // pub ellipsis: Option<EllipsisProps>,
    // decorations
    #[prop_or_default]
    pub bold: bool,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub style: Option<Style>,
    #[prop_or_default]
    pub line_style: Option<LineStyle>,
}

#[derive(Debug, Clone, Default, PartialEq, Properties)]
pub struct Props {
    pub component: String,
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
    #[prop_or_default]
    pub children: String,
}

#[derive(Debug, Clone, Default)]
pub struct BaseState {
    edit: bool,
    copied: bool,
    // is_ellipsis: bool,
    // expanded: bool,
    // ellipsis_text: String,
}

#[derive(Debug)]
pub struct Base {
    state: BaseState,
    edit_icon_ref: NodeRef,
    contend_ref: NodeRef,
    job: Option<TimeoutTask>,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    // OnExpandClick(MouseEvent),
    OnEditClick(Option<MouseEvent>),
    OnEditChange(String),
    OnEditCancel,
    OnCopyClick(MouseEvent),
    OnCopied,
}

impl Base {
    fn prefix_class(&self) -> String {
        get_prefix_class(Some("typography"), None)
    }

    fn wrap_decoration(&self, content: Html) -> Html {
        let BaseProps { bold, line_style, style, .. } = self.props.base_props;
        let mut current_content = content;
        let wrap = |wrapper| {
            current_content = html! {
                <wrapper>
                    { current_content }
                </wrapper>
            }
        };

        if bold {
            wrap("strong")
        }
        if let Some(line_style) = &line_style {
            wrap(&line_style.to_string())
        }
        if let Some(style) = &style {
            wrap(&style.to_string())
        }
        current_content
    }

    fn get_editable(&self) -> EditProps {
        let edit = self.state.edit;
        if let Some(editable) = self.props.base_props.editable {
            EditProps {
                editing: edit,
                ..editable
            }
        } else {
            EditProps {
                editing: edit,
                ..Default::default()
            }
        }
    }

    // fn get_ellipsis(&self, props: Option<BaseProps>) -> EllipsisProps {
    //     let BaseProps { ellipsis, .. } = props.unwrap_or(self.props);
    //     if let Some(ellipsis) = ellipsis {
    //         EllipsisProps {
    //             rows: 1,
    //             expandable: false,
    //             ..ellipsis
    //         }
    //     } else {
    //         EllipsisProps::default()
    //     }
    // }

    fn trigger_edit(&self, edit: bool) {
        if edit {
            self.get_editable().on_start.emit(());
            self.state.edit = edit;
            if !edit && self.edit_icon_ref != Default::default() {
                self.edit_icon_ref.cast::<HtmlElement>().map(|e| e.focus());
            }
        }
    }

    // fn render_expand(&self, force_render: bool) -> Option<Html> {
    //     let EllipsisProps { expandable, expand_text, .. } = self.get_ellipsis(None);
    //     let BaseState { expanded, is_ellipsis, .. } = self.state;

    //     if !(expandable || (force_render && (expanded || !is_ellipsis))) {
    //         None
    //     } else {
    //         Some(html! {
    //             <a
    //                 key="expand",
    //                 class=format!("{}-expand", self.prefix_class())
    //                 onclick=self.link.callback(|e| Msg::OnExpandClick(e))
    //                 aria-label=expand_text
    //             >
    //             { expand_text }
    //             </a>
    //         })
    //     }
    // }

    fn render_edit(&self) -> Option<Html> {
        if let Some(editable) = self.props.base_props.editable {
            const aria_label: &str = "Edit";

            Some(html! {
                <div key="edit">
                    <TransButton
                        node_ref=self.edit_icon_ref
                        class=Classes::from(format!("{}-edit", self.prefix_class()))
                        on_click=self.link.callback(Msg::OnEditClick)
                        aria_label=aria_label
                    >
                        <EditingAssets icon=EditingIcon::Edit/>
                    </TransButton>
                </div>
            })
        } else {
            None
        }
    }

    fn render_copy(&self) -> Option<Html> {
        let copied = self.state.copied;
        if let Some(copyable) = self.props.base_props.copyable {
            const tooltips: [&str; 2] = ["Copy", "Copied"];  
            const icons: [EditingIcon; 2] = [EditingIcon::Copy, EditingIcon::Check];
            let idx = if copied { 1 } else { 0 };
            let mut classes = Classes::from(vec![format!("{}-copy", self.prefix_class())]);
            if copied {
                classes.push(format!("{}-copy-success", self.prefix_class()));
            }

            Some(html! {
                <div key="copy">
                    <TransButton
                        class=classes
                        on_click=self.link.callback(Msg::OnEditClick)
                        aria_label=tooltips[idx]
                    >
                        <EditingAssets icon=icons[idx] />
                    </TransButton>
                </div>
            })
        } else {
            None
        }
    }

    fn render_edit_input(&self) -> Html {
        let Props { children, class, .. } = self.props;
        let EditProps { max_length, auto_size, on_end, .. } = self.get_editable();

        html! {
            <Editable 
                prefix_class=self.prefix_class()
                class=class
                value=children.to_owned()
                on_save=self.link.callback(Msg::OnEditChange)
                on_cancel=self.link.callback(|_| Msg::OnEditCancel)
                on_end=on_end
                max_length=max_length
            />
        }
    }

    fn render_operation(&self) -> Vec<Html> {
        // self.render_expand(force_render), 
        vec![self.render_edit(), self.render_copy()].into_iter().filter_map(|e| e).collect()
    }

    fn render_content(&self) -> Html {
        // let BaseState { expanded, is_ellipsis, ellipsis_text,  .. } = self.state;
        // let EllipsisProps { rows, expand_text, .. } = self.get_ellipsis(None);
        let Props { base_props, id, class, key, component, .. } = self.props;
        let BaseProps { color, disabled, style, title, .. } = base_props;
        let prefix_class = self.prefix_class();
        let mut text_node = Html::from(self.props.children.to_owned());

        // if rows != 0 && is_ellipsis && !expanded {
            // let mut rest_content = if title.is_empty() {
            //     children.to_string()
            // } else {
            //     title.to_string()
            // };
            // rest_content = rest_content[..ellipsis_text.len()];
            // let text_node = html! {
            //     <>
            //         {ellipsis_text}
            //         <span title=rest_content aria-hidden=true>
            //             {ELLIPSIS_STR}
            //         </span>
            //     </>
            // }
        // }
        text_node = self.wrap_decoration(text_node);
        let mut classes = Classes::from(get_prefix_concat_with(&prefix_class, color));
        if disabled {
            classes.push(get_prefix_concat_with(&prefix_class, "disabled").as_str());
        }
        classes.extend(class);

        html! {
            <Typography
                component=component.to_owned()
                id=id
                class=classes
                key=key
                node_ref=self.contend_ref
                aria_label=title
            >
                { text_node }
                { self.render_operation() }
            </Typography>
        }
    }
}

impl Component for Base {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props, link, 
            state: Default::default(),
            edit_icon_ref: Default::default(),
            contend_ref: Default::default(),
            job: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender { 
        use Msg::*;
        let mut res = false;

        match msg {
            // OnExpandClick(e) => {
            //     let EllipsisProps { on_expand, .. } = self.get_ellipsis(None);
            //     self.state.expanded = true;
            //     on_expand.emit(e);
            //     res = true;
            // },
            OnEditClick(e) => {
                e.map(|e| e.prevent_default());
                self.trigger_edit(true);
                res = true;
            },
            OnEditChange(val) => {
                self.get_editable().on_change.emit(val);
                self.trigger_edit(false);
                res = true;
            },
            OnEditCancel => {
                self.get_editable().on_cancel.emit(());
                self.trigger_edit(false);
                res = true;
            },
            OnCopyClick(e) => {
                e.prevent_default();
                let BaseProps { copyable, .. } = self.props.base_props;
                let copy_props = copyable.unwrap_or_default();

                if copy_props.text.is_empty() {
                    copy_props.text = self.props.children.to_string();
                }

                if let Ok(ctx) = ClipboardContext::new() {
                    ctx.set_contents(copy_props.text);
                    self.state.copied = true;
                    copy_props.on_copy.emit(());
                    self.job = Some(TimeoutService::spawn(Duration::from_millis(3000), self.link.callback(|_| Msg::OnCopied)));
                    res = true;
                }
            },
            OnCopied => {
                self.state.copied = false;
                res = true;
            }
        }
        res
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // let mut res = false;
        // if let Some(ellipsis) = props.ellipsis {
        //     if let Some(prev_ellipsis) = self.props.ellipsis {
        //         if self.props.children != props.children || ellipsis.rows != prev_ellipsis.rows {
        //             self.props = props;
        //             res = true;
        //         }
        //     }
        // }
        // res
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if self.get_editable().editing {
            self.render_edit_input()
        } else {
            self.render_content()
        }
    }

    fn destroy(&mut self) {
        if let Some(job) = self.job {
            drop(job);
        }
    } 
}

