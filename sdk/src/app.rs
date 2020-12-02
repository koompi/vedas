use iced::{button::State as BtnState, Color, Element, Sandbox, Svg};
use vedas_core::*;
#[derive(Debug, Clone, Default)]
pub struct VedasSDK {
    search_btn: BtnState,
    components_btn: BtnState,
    run_bun: BtnState,
    deploy_btn: BtnState,
    setting_btn: BtnState,
    user_btn: BtnState,
    menu_state: bool,
}

// component!(VedasSDK: "Hello" => menu_state: bool, menu_btn: BtnState);
message!(pub VedasMsg, ToggleMenu);

impl Sandbox for VedasSDK {
    type Message = VedasMsg;
    f!(new, Self, { Self::default() });
    f_ref_self!(self, title, String, { String::from("Hello") });
    f_ref_mut_self!(self, update, message: VedasMsg, {
        match message {
            VedasMsg::ToggleMenu => self.menu_state = !self.menu_state,
        }
    });

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let left_side_bar_column = col!(units!(40))
            .align_items(center!())
            .push(
                btn_svg!(&mut self.search_btn, svg!("sdk/assets/search.svg"))
                    .padding(8)
                    .width(units!(40))
                    .style(LSBBtnStyle),
            )
            .push(
                btn_svg!(&mut self.components_btn, svg!("sdk/assets/components.svg"))
                    .padding(8)
                    .width(units!(40))
                    .style(LSBBtnStyle),
            )
            .push(
                btn_svg!(&mut self.run_bun, svg!("sdk/assets/run.svg"))
                    .padding(8)
                    .width(units!(40))
                    .style(LSBBtnStyle),
            )
            .push(
                btn_svg!(&mut self.deploy_btn, svg!("sdk/assets/deploy.svg"))
                    .padding(8)
                    .width(units!(40))
                    .style(LSBBtnStyle),
            )
            .push(h_space!(fill!()))
            .push(
                btn_svg!(&mut self.user_btn, svg!("sdk/assets/user.svg"))
                    .padding(8)
                    .width(units!(40))
                    .style(LSBBtnStyle),
            )
            .push(
                btn_svg!(&mut self.setting_btn, svg!("sdk/assets/settings.svg"))
                    .padding(8)
                    .width(units!(40))
                    .style(LSBBtnStyle),
            );
        let left_side_bar_container = container!(units!(40), fill!(), left_side_bar_column)
            .style(LSBContainerStyle)
            .center_x();

        let main_area = col!().push(text!("Hi"));
        let main_area_container =
            container!(portion!(1), fill!(), main_area).style(MainAreaContainer);
        // Left pane
        let left_pane_column = col!();
        let left_pane_container =
            container!(units!(300), fill!(), left_pane_column).style(LeftPaneContainer);
        // Right pane
        let right_pane_column = col!();
        let right_pane_container =
            container!(units!(300), fill!(), right_pane_column).style(RightPaneContainer);

        let main_row = row!(units!(40), fill!())
            .push(left_side_bar_container)
            .push(left_pane_container)
            .push(main_area_container)
            .push(right_pane_container);

        let main_contaier = container!(fill!(), fill!(), main_row)
            .style(MainContainerStyle)
            .into();
        main_contaier
    }
}

style_container!(MainContainerStyle {
    text_color: Some(Color::WHITE),
    background: Some(iced::Background::Color(Color::from_rgb8(106, 106, 106))),
    border_radius: 0.,
    border_width: 0.,
    border_color: Color::BLACK
});

// Left sidebar ===============================================================

style_container!(LSBContainerStyle {
    text_color: Some(Color::BLACK),
    background: Some(iced::Background::Color(Color::from_rgb8(223, 223, 223))),
    border_radius: 0.,
    border_width: 0.,
    border_color: Color::BLACK
});

style_btn!(LSBBtnStyle {
    shadow_offset: iced::Vector::new(0., 0.),
    background: None,
    border_radius: 0.,
    border_width: 0.,
    border_color: Color::BLACK,
    text_color: Color::BLACK
});
// Left pane
style_container!(LeftPaneContainer {
    text_color: Some(Color::BLACK),
    background: Some(iced::Background::Color(Color::from_rgb8(196, 196, 196))),
    border_radius: 0.,
    border_width: 0.,
    border_color: Color::from_rgb8(230, 230, 230)
});
// Right pane
style_container!(RightPaneContainer {
    text_color: Some(Color::BLACK),
    background: Some(iced::Background::Color(Color::from_rgb8(196, 196, 196))),
    border_radius: 0.,
    border_width: 0.,
    border_color: Color::from_rgb8(230, 230, 230)
});

// Main area
style_container!(MainAreaContainer {
    text_color: Some(Color::BLACK),
    background: None,
    border_radius: 0.,
    border_width: 0.,
    border_color: Color::from_rgb8(250, 255, 255)
});

style_btn!(MenuBtnStyle {
    shadow_offset: iced::Vector::new(0., 0.),
    background: None,
    border_radius: 0.,
    border_width: 0.,
    border_color: Color::BLACK,
    text_color: Color::WHITE
});
