#![recursion_limit="1024"]
use yew::prelude::*;
use yew::services::ConsoleService;
use vedas_ui::{
    button, text, Button, ButtonGroup, Title, Paragraph, Text, 
	utils::{Color, Size},
};

pub enum Msg {
	Clicked(&'static str),
	TextChanged(String),
}

pub struct App {
	link: ComponentLink<Self>,
	text: String,
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link, 
			text: String::from("This is an editable text.")
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Clicked(text) => ConsoleService::log(text),
			Msg::TextChanged(val) => self.text = val,
		}
		true
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender { false }	

	fn view(&self) -> Html {
		html! {
			<div style="padding: 20px;">
				<Title level=text::Level::H2>{ "Introduction" }</Title>
				<Paragraph>
					{ "In the process of internal desktop applications development, many different design specs and
					implementations would be involved, which might cause designers and developers difficulties and
					duplication and reduce the efficiency of development." }
				</Paragraph>
				<Paragraph>
					{ "After massive project practice and summaries, Ant Design, a design language for background
					applications, is refined by Ant UED Team, which aims to" }
				</Paragraph>
				<Text options=text::Options {
					bold: true,
					..text::Options::default()
				}>
					{ "uniform the user interface specs for internal background projects, lower the unnecessary
					cost of design differences and implementation and liberate the resources of design and
					front-end development." }
				</Text>
				<Title level=text::Level::H3>{ "Button" }</Title>
				<Button 
					on_click=self.link.callback(|_| Msg::Clicked("Emited callback"))
				>{ "Default Button" }
				</Button>
				<Title level=text::Level::H4>{ "Style" }</Title>
				<ButtonGroup>
					<Button style=button::Style::Fill>{ "Fill Button" }</Button>
					<Button style=button::Style::Dashed>{ "Dashed Button" }</Button>
					<Button style=button::Style::Text>{ "Text Button" }</Button>
					<Button style=button::Style::Link>{ "Link Button" }</Button>
				</ButtonGroup>
				<Title level=text::Level::H4>{ "Shape" }</Title>
				<div>
					<Button shape=button::Shape::Round>{ "Round Button" }</Button>
					<Button shape=button::Shape::Circle>{ "$" }</Button>
				</div>
				<Title level=text::Level::H4>{ "Size" }</Title>
				<ButtonGroup>
					<Button size=Size::Small>{ "Small Button" }</Button>
					<Button size=Size::Large>{ "Large Button" }</Button>
				</ButtonGroup>
				<Title level=text::Level::H4>{ "Other" }</Title>
				<ButtonGroup>
					<Button disabled=true>{ "Disabled Button" }</Button>
					<Button loading=Some(500)>{ "Loading Button" }</Button>
					<Button danger=true>{ "Danger Button" }</Button>
				</ButtonGroup>
				<Button block=true>{ "Block Button" }</Button>
				<Title level=text::Level::H3>{ "Typography" }</Title>
				<div>
					<Text>{"Plain Text (default)"}</Text>
					<Text options=text::Options {
						color: Color::Secondary,
						..text::Options::default()
					}>{"Plain Text (secondary)"}</Text>
					<Text options=text::Options {
						color: Color::Success,
						..text::Options::default()
					}>{"Plain Text (success)"}</Text>
					<Text options=text::Options {
						color: Color::Warning,
						..text::Options::default()
					}>{"Plain Text (warning)"}</Text>
					<Text options=text::Options {
						color: Color::Danger,
						..text::Options::default()
					}>{"Plain Text (danger)"}</Text>
					<Text options=text::Options {
						disabled: true,
						..text::Options::default()
					}>{"Plain Text (disabled)"}</Text>
					<Text options=text::Options {
						style: text::Style::Highlight,
						..text::Options::default()
					}>{"Plain Text (highlight)"}</Text>
					<Text options=text::Options {
						style: text::Style::Code,
						..text::Options::default()
					}>{"Plain Text (code)"}</Text>
					<Text options=text::Options {
						style: text::Style::Keyboard,
						..text::Options::default()
					}>{"Plain Text (keyboard)"}</Text>
					<Text options=text::Options {
						line_style: text::LineStyle::Underline,
						..text::Options::default()
					}>{"Plain Text (underline)"}</Text>
					<Text options=text::Options {
						line_style: text::LineStyle::Crossline,
						..text::Options::default()
					}>{"Plain Text (crossline)"}</Text>
					<Text options=text::Options {
						bold: true,
						..text::Options::default()
					}>{"Plain Text (bold)"}</Text>
				</div>
				<Title level=text::Level::H4>{ "Interactive Text" }</Title>
				<div>
					<Paragraph options=text::Options {
						editable: Some(text::EditProps {
							on_change: self.link.callback(Msg::TextChanged),
							..text::EditProps::default()
						}),
						..text::Options::default()
					}>
						{ self.text.to_string() }
					</Paragraph>
					<Paragraph options=text::Options {
						copyable: Some(text::CopyProps::default()),
						..text::Options::default()
					}>
						{ "This is a copyable text." }
					</Paragraph>
				</div>
			</div>
		}
	}
}

fn main() {
	yew::start_app::<App>();
}