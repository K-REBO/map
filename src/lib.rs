#![recursion_limit = "1024"]
#[warn(unused_imports)]
mod modules;

use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;

use modules::map::*;
use modules::search_bar::SearchBar;
use modules::side_bar::SideBar;

enum Page {
	Search,
	Main,
	SideBar,
}
enum Msg {
	ChangePage(Page),
	UpFloor,
	DownFloor,
	AddGoal(u16),
	CanvasClick(i32, i32),
	Test,
}
struct Model {
	link: ComponentLink<Self>,
	page: Page,
	map: Map,
}
impl Component for Model {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

		let (mut start, mut goal): (Option<u16>, Option<u16>) = (Some(1), Some(1));
		let (mut current_floor, mut display_floor): (u8, u8) = (1, 1);
		match storage.get_item("first").unwrap() {
			Some(a) => {
				if let Some(floor) = storage.get_item("current_floor").unwrap() {
					current_floor = floor.parse().unwrap();
				}
				if let Some(floor) = storage.get_item("display_floor").unwrap() {
					display_floor = floor.parse().unwrap();
				}
				let None_string: String = String::from("None");

				if let Some(node) = storage.get_item("start").unwrap() {
					match node {
						None_string => start = None,
						_ => {
							start = Some(node.parse().unwrap());
						}
					}
				}

				if let Some(node) = storage.get_item("goal").unwrap() {
					match node {
						None_string => goal = None,
						_ => {
							goal = Some(node.parse().unwrap());
						}
					}
				}
			}
			None => {
				storage.set_item("first", "Hi");
				storage.set_item("current_floor", "1");
				storage.set_item("display_floor", "1");
				storage.set_item("start", "1");
				storage.set_item("goal", "None");
			}
		}

		Self {
			link,
			page: Page::Main,
			map: Map {
				current_floor,
				display_floor,
				start : Some(1),
				goal : goal,
			},
		}
	}

	fn rendered(&mut self, first_render: bool) {
		if first_render {
			self.map.init();
			self.map.draw_route();
			self.map.draw_map();
		}
		unsafe {
			web_sys::console::log_1(
				&format!(
					"current: {} display: {} start: {:?} goal: {:?}",
					self.map.current_floor, self.map.display_floor, self.map.start, self.map.goal
				)
				.into(),
			);
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::ChangePage(page) => self.page = page,
			Msg::UpFloor => {
				let msg: String;
				match self.map.move_floor(self.map.display_floor + 1) {
					Some(floor) => {
						msg = floor.to_string();
						let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
						storage.set_item("display_floor", &floor.to_string());
					}
					None => msg = "failed to Up".into(),
				}
				unsafe { web_sys::console::log_1(&msg.into()) }
			}
			Msg::DownFloor => {
				let msg: String;
				match self.map.move_floor(self.map.display_floor - 1) {
					Some(floor) => {
						msg = floor.to_string();
						let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
						storage.set_item("display_floor", &floor.to_string());
					},
					None => msg = "failed to Up".into(),
				}
				unsafe { web_sys::console::log_1(&msg.into()) }
			}
			Msg::AddGoal(goal) => {
				self.map.goal = Some(goal);
				unsafe {
					web_sys::console::log_1(&format!("goal: {}", &self.map.goal.unwrap()).into());
				}
				let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
				storage.set_item("goal", &goal.to_string());
				self.page = Page::Main;
				self.map.draw_map();
			}
			Msg::Test => unsafe {
				web_sys::console::log_1(&"callBacked".into());
			},
			Msg::CanvasClick(x,y) => {
				unsafe{web_sys::console::log_1(&format!("x:{}, y:{}",x,y).into());}
			}
		}
		true
	}

	fn change(&mut self, _props: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		html! {
			<div>
			{
				match self.page {
					Page::Main => html!{},
					Page::Search => {
					 html! {
						 <SearchBar
							value=""
							start=self.map.start
							on_exit=self.link.callback(|_| Msg::ChangePage(Page::Main))
							on_enter=self.link.callback(Msg::AddGoal)
						 ></SearchBar>
					 }
					},
					Page::SideBar => {
					 html!{
						 <SideBar on_exit=self.link.callback(|_| Msg::ChangePage(Page::Main))></SideBar>
					 }
					},
					_ => html!{},
				}

			}

				<button style="position: fixed; z-index : 10;" onclick=self.link.callback(|_| Msg::ChangePage(Page::SideBar))>
					<svg xmlns="http://www.w3.org/2000/svg" class="w-20 h-20 md:w-32 md:h-32" viewBox="0 0 24 24"
					fill="none" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<line x1="3" y1="12" x2="21" y2="12"/>
						<line x1="3" y1="6" x2="21" y2="6"/>
						<line x1="3" y1="18" x2="21" y2="18"/>
					</svg>
				</button>

				<canvas id="canvas" width="2500px" height="1500px" style="touch-action: manipulation; z-index : 0;" onclick=self.link.callback(|e : web_sys::MouseEvent| Msg::CanvasClick(e.client_x(),e.client_y()))></canvas>


				<nav style="left: 0px; bottom: 20%; flex-direction: column;" class="fixed flex">
					<button style="background-color:#d7d7dc" onclick=self.link.callback(|_| Msg::UpFloor)>
						<svg xmlns="http://www.w3.org/2000/svg" class="w-20 h-20 md:w-32 md:h-32" viewBox="0 0 24 24"
						fill="none" stroke="#000000" strokeWidth="5" strokeLinecap="round" strokeLinejoin="round">
							<path d="M17 11l-5-5-5 5M17 18l-5-5-5 5"/>
						</svg>
					</button>

					<span style="text-align: center; font-size:xx-large;">
					{
						format!("{}F", self.map.display_floor)
					}
					</span>

					<button style="background-color:#d7d7dc" onclick=self.link.callback(|_| Msg::DownFloor)>
						<svg xmlns="http://www.w3.org/2000/svg" class="w-20 h-20 md:w-32 md:h-32" viewBox="0 0 24 24"
						fill="none" stroke="#000000" strokeWidth="3.5" strokeLinecap="round" strokeLinejoin="round">
							<path d="M7 13l5 5 5-5M7 6l5 5 5-5"/>
						</svg>
					</button>
				</nav>

				<nav  class="justify-evenly flex inset-x-0" style="position:fixed; bottom: 5%;">
					<button onclick=self.link.callback(|_| Msg::ChangePage(Page::Search)) class="search_button_main">
						<svg class="w-20 h-20 md:w-32 md:h-32" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
							<path fill-rule="evenodd"
							d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z"
							clip-rule="evenodd"/>
						</svg>
					</button>

					<a class="camera_button_main" onclick=self.link.callback(|_| Msg::Test)
						href="https://map.oberk.dev/ar"
					>

						<svg xmlns="http://www.w3.org/2000/svg" class="w-20 h-20 md:w-32 md:h-32" viewBox="0 0 24 24"
						fill="none" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<g transform="translate(2 3)">
								<path d="M20 16a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3l2-3h6l2 3h3a2 2 0 0 1 2 2v11z"/>
								<circle cx="10" cy="10" r="4"/>
							</g>
						</svg>
					</a>
				</nav>
			</div>
		}
	}

	fn destroy(&mut self) {}
}

#[wasm_bindgen(start)]
pub fn run_app() {
	App::<Model>::new().mount_to_body();
}
