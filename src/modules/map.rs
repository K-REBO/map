use std::{iter::StepBy, u16};

use wasm_bindgen::prelude::*;

use super::route;
use super::utils;

pub struct Map {
	pub current_floor: u8,
	pub display_floor: u8,

	pub start: Option<u16>,
	pub goal: Option<u16>,
}

impl Map {
	pub fn init(&self) {
		unsafe {
			init();
		}
	}

	pub fn draw_route(&self) -> Option<String> {
		let start: u16;
		let goal: u16;

		match (self.start, self.goal) {
			(Some(s), Some(g)) => {
				start = s;
				goal = g;
			}
			_ => {
				let msg = "Err::start node or goal node is set".to_string();

				log::info!("{}", msg);
				return None;
			}
		}

		let (route, cost) = match route::dijkstra(start, goal) {
			Ok(i) => i,
			Err(text) => {
				log::info!("Err in function draw_map in impl Map{}", text);
				return None;
			}
		};
		log::info!("Dijkstra Result: COST: {}", cost);

		unsafe {
			drawRoute(route, self.display_floor);
		}

		Some("".into())
	}

	pub fn move_floor(&mut self, floor: u8) -> Option<u8> {
		match floor {
			1..=5 => {
				self.display_floor = floor;
				self.draw_map();
				Some(floor)
			}
			_ => None,
		}
	}

	pub fn draw_map(&self) {
		unsafe { drawMap(self.display_floor) }
		self.draw_route();

		if let Some(s) = self.start {
			if self.display_floor == utils::classify_floor_by_node(s).unwrap() {
				unsafe {
					drawPlaceByNode(s, false);
				}
			}
		}

		if let Some(g) = self.goal {
			if self.display_floor == utils::classify_floor_by_node(g).unwrap() {
				unsafe {
					drawPlaceByNode(g, true);
				}
			}
		}
	}

	pub fn color(x: i32, y: i32) -> Vec<u16> {
		unsafe {
			return get_node(x, y);
		}
	}

}

#[wasm_bindgen(module = "/src/js/canvas.js")]
extern "C" {
	fn init();
	fn drawMap(floor: u8);
	fn drawNode(node: u16, color: &str);
	fn drawRoute(route: Vec<u16>, floor: u8);
	fn drawConnectNode(node1: u16, node2: u16);
	fn drawPlaceByNode(node: u16, is_goal: bool);
}
#[wasm_bindgen(module = "/src/js/utils.js")]
extern "C" {
	fn get_node(x:i32, y:i32) -> Vec<u16>;
}
