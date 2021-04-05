use serde_json::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use yew::{Component, ComponentLink, Html, ShouldRender};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

pub fn map() {
	let map = include_str!("map.json");
	let json : Value = serde_json::from_str(map).unwrap();
	let json = json.get("map1").unwrap().get("line");
	for jjj in json.into_iter() {
		unsafe {
			web_sys::console::log_1(&format!("{}",jjj).into());
		}	
	}


}

pub struct Map {
	canvas : Option<HtmlCanvasElement>,
	ctx : Option<CanvasRenderingContext2d>,
	canvas_magnification : Option<f32>,

	current_floor: u8,
	current_node: Option<u16>,

	display_floor: u8,

	start: Option<u16>,
	goal: Option<u16>,
}
impl Map {
	fn init_test(&mut self) {
		let document = web_sys::window().unwrap().document().unwrap();
		let board = document.get_element_by_id("canvas").unwrap();
		let canvas: web_sys::HtmlCanvasElement = board
		.dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
		.unwrap();
		let ctx = canvas
        .get_context("2d")
		.unwrap()
		.unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
		.unwrap();
	
		self.canvas = Some(canvas);
		self.ctx = Some(ctx);
	}
	fn draw_line(&self, start_x : i32, start_y : i32, end_x : i32, end_y : i32, breadth : i32) {
		let start_x = (start_x as f32 * self.canvas_magnification.unwrap() / 2f32) as f64;
		let start_y = (start_y as f32 * self.canvas_magnification.unwrap() / 2f32) as f64;
		let end_x = (end_x as f32 * self.canvas_magnification.unwrap() / 2f32) as f64;
		let end_y = (end_y as f32 * self.canvas_magnification.unwrap() / 2f32) as f64;
		let breadth = breadth as f64;

		let ctx = self.ctx.clone().unwrap();

		ctx.begin_path();
		ctx.move_to(start_x, start_y);
		ctx.line_to(end_x, end_y);
		ctx.line_to(end_x + breadth, end_y + breadth);
		ctx.line_to(start_x + breadth, start_y + breadth);
		ctx.close_path();
		ctx.fill();
	}
	fn connect_node(&self, map_json : Value, node1 : u16, node2 : u16) {
		if utils::classify_floor_by_node(node1) != utils::classify_floor_by_node(node2) {
			unsafe{web_sys::console::log_1(&"node1 and node2 is not in same floor".into())}
			return;
		}

		let floor_map : &Value;
		let floor = utils::classify_floor_by_node(node1).unwrap();
		match floor {
			1 => floor_map = map_json.get("map1").unwrap(),
			2 => floor_map = map_json.get("map2").unwrap(),
			3 => floor_map = map_json.get("map3").unwrap(),
			4 => floor_map = map_json.get("map4").unwrap(),
			5 => floor_map = map_json.get("map5").unwrap(),
			_ => return,
		}

		let node1 = node1 - utils::floor_bounds_node(floor).unwrap();
		let node2 = node2 - utils::floor_bounds_node(floor).unwrap();
		let mut pos1 : i32;

		for i in floor_map.get("Node").into_iter() {
			if i.get("NodeID").unwrap() == node1 {
				pos1 = i.get("pos").unwrap() as i32;
			}
		}



	}
	fn draw_node(&self) {

	}
}

mod utils {
	pub fn classify_floor_by_node(node : u16) -> Option<u8> {// from utils.NodeFloor in canvas.js

		if 0 < node && node <= classify_floor_by_node(2)? as u16 {
			Some(1)
		}
		else if (classify_floor_by_node(2)? as u16) < node && node <= (classify_floor_by_node(3)? as u16) {
			Some(2)
		}
		else if (classify_floor_by_node(3)? as u16) < node && node <= (classify_floor_by_node(4)? as u16) {
			Some(3)
		}
		else if (classify_floor_by_node(4)? as u16) < node && node <= (classify_floor_by_node(5)? as u16) {
			Some(4)
		}
		else if (classify_floor_by_node(5)? as u16) < node {
			Some(5)
		}
		else {
			None
		}

	}	

	pub fn floor_bounds_node(floor : u8) -> Option<u16> {// from utils.FloorVariation in canvas.js
		match floor {
			1 => Some(0),
			2 => Some(28),
			3 => {
				let floor_by_node = classify_floor_by_node(2)? as u16;
				Some(floor_by_node + 23)
			},
			4 => {
				let floor_by_node = classify_floor_by_node(3)? as u16;
				Some(floor_by_node + 18)
			},
			5 => {
				let floor_by_node = classify_floor_by_node(4)? as u16;
				Some(floor_by_node + 17)
			},
			_ => None,
		}
	}
}