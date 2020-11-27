use iced::{button::State as BtnState, Color, Sandbox, Svg};
use vedas_core::*;
#[derive(Debug, Clone, Default)]
pub struct VedasSDK {
    menu_btn: BtnState,
    menu_state: bool,
}

#[derive(Debug, Clone)]
pub enum VedasMsg {
    ToggleMenu,
}

impl Sandbox for VedasSDK {
    type Message = VedasMsg;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Vedas SDK")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            VedasMsg::ToggleMenu => self.menu_state = !self.menu_state,
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        println!("{}", self.menu_state);
        let header_contnr = container!(
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
        let rsb_container = container!(
            units!(300),
            fill!(),
            col!(fill!()).push(text!("Right sidebar"))
        )
        .style(MenuContainer);
        let main_area = container!(fill!(), fill!(), col!(fill!()).push(text!("Main area")));
        let body_row = if self.menu_state {
            row!(fill!())
                .push(lsb_container)
                .push(main_area)
                .push(rsb_container)
        } else {
            row!(fill!()).push(main_area).push(rsb_container)
        };

        let main_column = col!().push(header_contnr).push(body_row);
        let main_container = container!(fill!(), main_column);
        main_container.into()
    }
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
