#![recursion_limit = "1024"]
#![allow(unused_imports)]

mod components;
mod modules;

use std::u16;
use wasm_bindgen::prelude::*;
use yew::html;
// use serde::{Deserialize, Serialize};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
	format::{Json, Nothing},
	prelude::*,
	services::StorageService,
};

use components::dialog::Dialog;
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
	ChangeDialogVisibility,
}
struct Model {
	link: ComponentLink<Self>,
	page: Page,
	map: Map,
	dialog_visible: bool,
	storage: StorageService,
}
impl Component for Model {
	type Message = Msg;
	type Properties = ();
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut storage = yew::services::StorageService::new(yew::services::storage::Area::Local)
			.expect("Can not start localStrage");

		let (mut start, mut goal): (Option<u16>, Option<u16>) = (None, None);
		let (mut current_floor, mut display_floor): (u8, u8) = (1, 1);

		// get current_floor from local storage
		match storage.restore("current_floor") {
			Ok(f) => {
				log::info!("current_floor from local storage;{}", f);
				match f.parse::<u8>() {
					Ok(floor) => {
						current_floor = floor;
					}
					Err(e) => {
						log::error!("Can't parse current_floor; {:#?}", e);
					}
				}
			}
			Err(e) => {
				log::error!("Can't get current_floor from LocalStorage error:{}", e);
				storage.store("current_floor", yew::format::Json(&1));
			}
		}

		// get display_floor from local storage
		match storage.restore("display_floor") {
			Ok(f) => {
				log::info!("display_floor from local storage;{}", f);
				match f.parse::<u8>() {
					Ok(floor) => {
						display_floor = floor;
					}
					Err(e) => {
						log::error!("Can't parse display; {:#?}", e);
					}
				}
			}
			Err(e) => {
				log::error!("Can't get display from LocalStorage error:{}", e);
				storage.store("display_floor", yew::format::Json(&1));
			}
		}

		// get start(node) from local storage
		match storage.restore("start") {
			Ok(s_node) => match s_node.parse::<u16>() {
				Ok(node) => {
					start = Some(node);
				}
				Err(e) => {
					log::error!("Can't parse start(node); {}", e);
				}
			},
			Err(e) => {
				log::error!("Can't get node from LocalStorage error:{}", e);
			}
		}

		// get goal from local storage
		match storage.restore("goal") {
			Ok(g_node) => match g_node.parse::<u16>() {
				Ok(node) => {
					goal = Some(node);
				}
				Err(e) => {
					log::error!("Can't parse goal(node); {}", e);
				}
			},
			Err(e) => {
				log::error!("Can't get node from LocalStorage error:{}", e);
			}
		}


		Self {
			link,
			page: Page::Main,
			map: Map {
				current_floor,
				display_floor,
				start,
				goal,
			},
			dialog_visible: false,
			storage,
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
						self.storage
							.store("display_floor", yew::format::Json(&floor))
					}
					None => msg = "failed to Up".into(),
				}
				log::info!("{}", msg);
			}
			Msg::DownFloor => {
				let msg: String;
				match self.map.move_floor(self.map.display_floor - 1) {
					Some(floor) => {
						msg = floor.to_string();
						self.storage
							.store("display_floor", yew::format::Json(&floor));
					}
					None => msg = "failed to Up".into(),
				}
				log::info!("{}", msg);
			}
			Msg::AddGoal(goal) => {
				self.map.goal = Some(goal);
				log::info!("goal: {}", &self.map.goal.unwrap());
				self.storage.store("goal", yew::format::Json(&goal));
				self.page = Page::Main;
				self.map.draw_map();
				self.dialog_visible = false;
			}
			Msg::CanvasClick(x, y) => {
				log::info!("x: {}, y:{}", x, y);
				log::info!("{:#?}", 
					modules::map::Map::color(x,y)
				);
				log::info!(
					"start: {:#?}, goal: {:#?}, current_floor: {}, display_floor: {}",
					self.map.start,
					self.map.goal,
					self.map.current_floor,
					self.map.display_floor
				);
				self.dialog_visible = true;
			}
			Msg::ChangeDialogVisibility => {
				self.dialog_visible = !self.dialog_visible;
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
						<SideBar on_exit=self.link.callback(|_| Msg::ChangePage(Page::Main))/>
					}
					},
				}
			}
			{
				match self.dialog_visible {
					true => html! {
						<Dialog
							on_close=self.link.callback(|_| Msg::ChangeDialogVisibility)
							on_add_goal=self.link.callback(|goal| {
								Msg::AddGoal(goal)
							}
							)
							goal=self.map.goal
							start=self.map.start
						/>
					},
					_ => html!{}
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

					<a class="camera_button_main" href="https://map.oberk.dev/ar">

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

	fn rendered(&mut self, first_render: bool) {
		if first_render {
			self.map.init();
			self.map.draw_route();
			self.map.draw_map();
		}
	}

	fn destroy(&mut self) {}
}

#[wasm_bindgen(start)]
pub fn run_app() {
	wasm_logger::init(wasm_logger::Config::default());
	yew::start_app::<Model>();
}
