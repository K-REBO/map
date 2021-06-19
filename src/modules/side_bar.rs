use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub on_exit: Callback<()>,
}
pub struct SideBar {
	link: ComponentLink<Self>,
	props: Props,
}

pub enum Msg {
	Exit,
}

impl Component for SideBar {
	type Message = Msg;
	type Properties = Props;
	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self { link, props }
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Exit => {
				self.props.on_exit.emit(());
				true
			}
		};
		false
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		if self.props != props {
			self.props = props;
			true
		} else {
			false
		}
	}

	fn view(&self) -> Html {
		html! {
			<div class="h-screen fixed" style="z-index:1000; background-color : #dfdfdf; width: 40%;">
				<button onclick=self.link.callback(|_| Msg::Exit)>
				<svg xmlns="http://www.w3.org/2000/svg" class="w-14" viewBox="0 0 24 24"
					fill="none" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<line x1="3" y1="12" x2="21" y2="12"/>
						<line x1="3" y1="6" x2="21" y2="6"/>
						<line x1="3" y1="18" x2="21" y2="18"/>
					</svg>
				</button>

				<div class="flex flex-col text-3xl ml-10">
					<div class="flex flex-row">
						<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="black" class="mt-3 h-6">
							<path d="M0 0h24v24H0z" fill="none"/>
							<path d="M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm2 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>
						</svg>
						<span>{"About"}</span>
					</div>
				</div>

				<div style="bottom: 5%; left:5%"  class="fixed">
					<span>{"v0.2.1"}</span>
					<a href="https://github.com/K-REBO/map">
						<svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" style="height : 5rem;">
							<title>{"GitHub icon"}</title>
							<path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084
							1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 
							3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
						</svg>
					</a>
				</div>
			</div>
		}
	}
}
