use iced::{button::State as BtnState, Color, Element, Sandbox, Svg};
use vedas_core::*;

component!(VedasSDK: "Hello" => menu_state: bool, menu_btn: BtnState);
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

    f_ref_mut_self!(self, view, Element<VedasMsg>, {
        let header_container = container!(
            fill!(),
            units!(40),
            col!(fill!()).push(
                row!().push(
                    container!(
                        units!(40),
                        fill!(),
                        btn_svg!(&mut self.menu_btn, svg!("./sdk/assets/menu.svg"))
                            .on_press(VedasMsg::ToggleMenu)
                            .style(MenuBtnStyle)
                    )
                    .center_x()
                    .center_y()
                )
            )
        )
        .style(HeaderContainer);
        let lsb_container = container!(
            units!(300),
            fill!(),
            col!(fill!()).push(text!("Left sidebar"))
        )
        .style(MenuContainer);
        let rsb_container = container!(units!(300), fill!(), col!(fill!())).style(MenuContainer);
        let main_area = container!(fill!(), fill!(), col!(fill!()).push(text!("Main area")));
        let body_row = if self.menu_state {
            row!(fill!())
                .push(lsb_container)
                .push(main_area)
                .push(rsb_container)
        } else {
            row!(fill!()).push(main_area).push(rsb_container)
        };

        let main_column = col!().push(header_container).push(body_row);
        let main_container = container!(fill!(), main_column);
        main_container.into()
    });
}

style_container!(HeaderContainer {
    text_color: Some(Color::WHITE),
    background: Some(iced::Background::Color(Color::BLACK)),
    border_radius: 0.,
    border_width: 1.,
    border_color: Color::BLACK
});

style_container!(MenuContainer {
    text_color: Some(Color::BLACK),
    background: Some(iced::Background::Color(Color::from_rgb8(230, 230, 230))),
    border_radius: 0.,
    border_width: 1.,
    border_color: Color::from_rgb8(230, 230, 230)
});

style_btn!(MenuBtnStyle {
    shadow_offset: iced::Vector::new(0., 0.),
    background: None,
    border_radius: 0.,
    border_width: 0.,
    border_color: Color::BLACK,
    text_color: Color::WHITE
});
