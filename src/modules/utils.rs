pub fn classify_floor_by_node(node: u16) -> Option<u8> {
	// from utils.NodeFloor in canvas.js

	if 0 < node && node <= floor_bounds_node(2)? as u16 {
		Some(1)
	} else if (floor_bounds_node(2)? as u16) < node && node <= (floor_bounds_node(3)? as u16) {
		Some(2)
	} else if (floor_bounds_node(3)? as u16) < node && node <= (floor_bounds_node(4)? as u16) {
		Some(3)
	} else if (floor_bounds_node(4)? as u16) < node && node <= (floor_bounds_node(5)? as u16) {
		Some(4)
	} else if (floor_bounds_node(5)? as u16) < node {
		Some(5)
	} else {
		None
	}
}

pub fn floor_bounds_node(floor: u8) -> Option<u16> {
	// from utils.FloorVariation in canvas.js
	match floor {
		1 => Some(0),
		2 => Some(28),
		3 => {
			let floor_by_node = floor_bounds_node(2)? as u16;
			Some(floor_by_node + 23)
		}
		4 => {
			let floor_by_node = floor_bounds_node(3)? as u16;
			Some(floor_by_node + 18)
		}
		5 => {
			let floor_by_node = floor_bounds_node(4)? as u16;
			Some(floor_by_node + 17)
		}
		_ => None,
	}
}
