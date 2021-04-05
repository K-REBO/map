use crate::modules::route::dijkstra;
use crate::modules::utils::{classify_floor_by_node, floor_bounds_node};
use std::collections::HashMap;

pub fn search_destination(input: &str, node_now: u16) -> Option<u16> {
	let mut map = HashMap::new();
	map.insert("食堂", 1);
	map.insert("生徒相談室", 18);
	map.insert("調整室", 16);
	map.insert("放送", 16);
	map.insert("パソコン", 4);
	map.insert("中庭", 17);
	map.insert("校長室", 26);
	map.insert("玄関", 3);
	map.insert("昇降口", 21);
	map.insert("事務室", 24);
	map.insert("生徒ホール", 27);
	map.insert("美術室", 12);
	map.insert("書道室", 14);
	map.insert("保健室", 16);
	map.insert("湯沸室", 5);
	map.insert("応接室", 11);

	map.insert("職員室", floor_bounds_node(2)? + 9);
	map.insert("教務室", floor_bounds_node(2)? + 9);
	map.insert("地学", floor_bounds_node(2)? + 1);
	map.insert("国際科", floor_bounds_node(2)? + 11);
	map.insert("英大", floor_bounds_node(2)? + 19);
	map.insert("英研", floor_bounds_node(2)? + 16);
	map.insert("数研", floor_bounds_node(2)? + 6);
	map.insert("家庭", floor_bounds_node(2)? + 4);
	map.insert("生徒会", floor_bounds_node(2)? + 15);
	map.insert("図書室", floor_bounds_node(2)? + 20);
	map.insert("国語", floor_bounds_node(2)? + 5);
	map.insert("視聴覚", floor_bounds_node(2)? + 17);

	map.insert("11", floor_bounds_node(5)? + 2);
	map.insert("12", floor_bounds_node(5)? + 3);
	map.insert("13", floor_bounds_node(5)? + 4);
	map.insert("14", floor_bounds_node(5)? + 6);
	map.insert("15", floor_bounds_node(5)? + 7);
	map.insert("16", floor_bounds_node(5)? + 8);
	map.insert("17", floor_bounds_node(5)? + 10);
	map.insert("18", floor_bounds_node(5)? + 11);
	map.insert("19", floor_bounds_node(5)? + 12);
	map.insert("10", floor_bounds_node(5)? + 13);

	map.insert("21", floor_bounds_node(4)? + 2);
	map.insert("22", floor_bounds_node(4)? + 3);
	map.insert("23", floor_bounds_node(4)? + 4);
	map.insert("24", floor_bounds_node(4)? + 6);
	map.insert("25", floor_bounds_node(4)? + 7);
	map.insert("26", floor_bounds_node(4)? + 8);
	map.insert("27", floor_bounds_node(4)? + 10);
	map.insert("28", floor_bounds_node(4)? + 11);
	map.insert("29", floor_bounds_node(4)? + 12);
	map.insert("20", floor_bounds_node(4)? + 13);

	map.insert("31", floor_bounds_node(3)? + 2);
	map.insert("32", floor_bounds_node(3)? + 3);
	map.insert("33", floor_bounds_node(3)? + 4);
	map.insert("34", floor_bounds_node(3)? + 6);
	map.insert("35", floor_bounds_node(3)? + 7);
	map.insert("36", floor_bounds_node(3)? + 8);
	map.insert("37", floor_bounds_node(3)? + 10);
	map.insert("38", floor_bounds_node(3)? + 11);
	map.insert("39", floor_bounds_node(3)? + 12);
	map.insert("30", floor_bounds_node(3)? + 13);

	let input: Vec<&str> = input.split_whitespace().collect();
	if input.contains(&"トイレ") {
		match classify_floor_by_node(node_now).unwrap() {
			1 => {
				if dijkstra(node_now, 3) >= dijkstra(node_now, 9) {
					return Some(9);
				} else {
					return Some(3);
				}
			}
			2 | 3 | 4 | 5 => {
				let floor = floor_bounds_node(classify_floor_by_node(node_now)?)?;
				if dijkstra(node_now, floor + 3) >= dijkstra(node_now, floor + 8) {
					return Some(floor + 3);
				} else {
					return Some(floor + 8);
				}
			}
			_ => (),
		}
	}

	for word in input.into_iter() {
		let word: &str = &(word.replace("HR", ""));
		for (k, v) in map.clone() {
			if k.contains(word) {
				return Some(v);
			}
		}
	}
	None
}
