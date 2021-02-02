use std::vec;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
mod search;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub value: String,
	pub on_exit: Callback<()>,
	pub on_enter: Callback<u16>,
	pub start: Option<u16>,
}

pub struct SearchBar {
	link: ComponentLink<Self>,
	props: Props,
	text: String,
}

pub enum Msg {
	CleanText,
	SetText(String),
	BackPage,
	Submit,
}

impl Component for SearchBar {
	type Message = Msg;
	type Properties = Props;

	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			props,
			text: String::from(""),
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::CleanText => {
				unsafe {
					clear_inner_by_id((&"input").to_string());
				}
				true
			}
			Msg::SetText(text) => {
				self.text = text;
				true
			}
			Msg::BackPage => {
				unsafe {
					web_sys::console::log_1(&"BackPage".into());
				}
				self.props.on_exit.emit(());
				true
			},
			Msg::Submit => {
				let start = match self.props.start {
					Some(s) => s,
					None => unsafe {
						web_sys::console::log_1(&"self.props.start doesn't have a value. add popup for tell user moving to AR-Mode".into());
						0
					},
				};

				unsafe {
					web_sys::console::log_1(&format!("start: {}", start).into());
				}

				if let Some(goal) = search::search_destination(&self.text, start) {
					self.props.on_enter.emit(goal);
				}
				true
			},
		}
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		if self.props != props {
			self.props = props;
			self.text = self.props.value.clone();
			true
		} else {
			false
		}
	}

	fn view(&self) -> Html {
		html! {
			<div style="position : fixed; height : 100%; width : 100%; z-index : 100; background-color : white">
				<div style="font-size:200%; @media (min-width: 640px) {font-size: 400%;}" class="flex justify-center">
					<div
					class="flex relative mr-6 my-2 flex flex-row shadow rounded bg-white">

						<input
							id="input"
							type="search"
							class="w-full" placeholder="校長.. 11HR.."
							oninput=self.link.callback(|e: InputData| Msg::SetText(e.value))
						/>
						<button onclick=self.link.callback(|_| Msg::CleanText)>
							<svg xmlns="http://www.w3.org/2000/svg" class="w-14 h-14 sm:w-24 sm:h-24" viewBox="0 0 24 24" fill="none" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<line x1="18" y1="6" x2="6" y2="18"/>
								<line x1="6" y1="6" x2="18" y2="18"/>
							</svg>
						</button>
						<button type="search" onclick=self.link.callback(|_| Msg::Submit)>
							<svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12 sm:w-20 sm:h-20" viewBox="0 0 24 24" fill="none" stroke="royalblue" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
							</svg>
						</button>
					</div>
					<div style="position: fixed; bottom: 5%; z-index: 100; background-color: #5850eb;" class="h-32 w-32  rounded-full">
						<button onclick=self.link.callback(|_| Msg::BackPage)>
							<svg class="w-32 h-32" fill="white" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
								<path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
							</svg>
						</button>
					</div>

				</div>
				<div class="flex flex-row">
				</div>
			</div>
		}
	}
}

struct Card {
	bg_color: String,
	color: String,
	title: String,
	attr: String,
	distance: String,
	time: String,
}

fn view_card(card: Card) -> Html {
	html! {
		<section class="w-1/3 rounded-xl text-center mt-10 mx-auto" style=format!("background-color : {}; color :{}", card.bg_color, card.color)>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white" width="60px" height="60px" class="rounded-full bg-blue-700 -mt-7 ml-7 border-4">
				<path d="M0 0h24v24H0V0z" fill="none"/>
				<path d="M5.5 22v-7.5H4V9c0-1.1.9-2 2-2h3c1.1 0 2 .9 2 2v5.5H9.5V22h-4zM18 22v-6h3l-2.54-7.63C18.18 7.55 17.42 7 16.56 7h-.12c-.86 0-1.63.55-1.9 1.37L12 16h3v6h3zM7.5 6c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2zm9 0c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2z"/>
			</svg>

			<span class="text-3xl">
				{
					card.title
				}
			</span>
			<h5>
				{card.attr}
			</h5>
			<div class="flex flex-row">
				<div class="bg-gray-300 w-1/6 m-5 rounded-lg">
					{
						card.distance
					}
				</div>
				<div class="bg-gray-300 w-1/6 m-5 rounded-lg">
					{
						card.time
					}
				</div>
			</div>
		</section>
	}
}

#[wasm_bindgen(module = "/src/js/utils.js")]
extern "C" {
	fn clear_inner_by_id(ID: String);
}
