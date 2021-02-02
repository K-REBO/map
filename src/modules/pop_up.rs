use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Button {
	LeftTopX,
	BottonNext,
	BottomText(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub button: Button,
	pub children: Children,
}

pub struct PopUp {
	link: ComponentLink<Self>,
	props: Props,
}
pub enum Msg {}
impl Component for PopUp {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { props, link }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		false
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		let w = 70;
		let h = 60;

		html! {
			<div class="flex justify-center">
				<div style={format!("width : {w}%; height : {h}%; background-color : red; ", w=w,h=h)} class="fixed ">
					<div>
						{
							match self.props.button.clone() {
								Button::LeftTopX => {
									html! {
										<button class="ml-auto">
											<svg xmlns="http://www.w3.org/2000/svg" width="49" height="49" viewBox="0 0 24 24" fill="none" stroke="#000000"
											stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
												<line x1="18" y1="6" x2="6" y2="18"/>
												<line x1="6" y1="6" x2="18" y2="18"/>
											</svg>
										</button>//X button
									}
								},
								Button::BottonNext => {
									html! {
										<div class="text-3xl ml-auto">
											<span>
												{"Next"}
											</span>
										</div>
									}
								},
								Button::BottomText(state) => {
									html! {
										<div class="text-3xl ml-auto">
											<span>
												{state}
											</span>
										</div>
									}
								},

							}
						}
						{ self.props.children.clone() }
					</div>
				</div>
			</div>
		}
	}
}
