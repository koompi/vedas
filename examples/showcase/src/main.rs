#![recursion_limit="512"]
use yew::prelude::*;
use yew::services::ConsoleService;
use vedas_ui::{
    button, Button, ButtonGroup, 
	utils::{Color, Size},
};

pub enum Msg {
	Clicked(&'static str),
}

pub struct App {
	link: ComponentLink<Self>
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Clicked(text) => ConsoleService::log(text)
		}
		true
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender { false }	

	fn view(&self) -> Html {
		html! {
			<div id="app" style="padding: 10px; width: 50%">
				<h1>{ "Showcase" }</h1>
				<h2>{ "Button" }</h2>
				<Button 
					on_click=self.link.callback(|_| Msg::Clicked("Emited callback"))
				>{ "Default Button" }
				</Button>
				<h2>{ "Style" }</h2>
				<ButtonGroup>
					<Button style=button::Style::Fill>{ "Fill Button" }</Button>
					<Button style=button::Style::Dashed>{ "Dashed Button" }</Button>
					<Button style=button::Style::Text>{ "Text Button" }</Button>
					<Button style=button::Style::Link>{ "Link Button" }</Button>
				</ButtonGroup>
				<h2>{ "Color" }</h2>
				<ButtonGroup size=Size::Small>
					<Button color=Color::Primary>{ "Primary Button" }</Button>
					<Button color=Color::Danger>{ "Danger Button" }</Button>
				</ButtonGroup>
				<h2>{ "Shape" }</h2>
				<ButtonGroup>
					<Button shape=button::Shape::Round>{ "Round Button" }</Button>
					<Button shape=button::Shape::Circle>{ "$" }</Button>
				</ButtonGroup>
				<h2>{ "Size" }</h2>
				<ButtonGroup>
					<Button size=Size::Small>{ "Small Button" }</Button>
					<Button size=Size::Large>{ "Large Button" }</Button>
				</ButtonGroup>
				<h2>{ "Other" }</h2>
				<ButtonGroup>
					<Button disabled=true>{ "Disabled Button" }</Button>
					<Button loading=Some(500)>{ "Loading Button" }</Button>
					<Button block=true>{ "Block Button" }</Button>
				</ButtonGroup>
			</div>
		}
	}
}

fn main() {
	yew::start_app::<App>();
}