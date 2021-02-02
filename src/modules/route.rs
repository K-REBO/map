use std::usize;

pub fn dijkstra(start: u16, goal: u16) -> Result<(Vec<u16>, usize), String> {
	if start == goal {
		return Err("input start and goal is same point.".into());
	}

	let virtual_infinity = 9999;
	let size = 1000;
	let mut via: Vec<i32> = vec![-1; size];
	let mut cost: Vec<u16> = vec![virtual_infinity; size];
	let mut used: Vec<bool> = vec![false; size];
	let mut dist: Vec<Vec<u16>> = vec![vec![virtual_infinity; size]; size];

	let mut min: u16;
	let mut target: usize = 0;
	let distance: u16;
	cost[start as usize] = 0;
	let floor2 = match node_max_by_floor(2) {
		Some(i) => i as usize,
		None => return Err("Floor is not".into()),
	};
	let floor3 = match node_max_by_floor(3) {
		Some(i) => i as usize,
		None => return Err("Floor is not".into()),
	};
	let floor4 = match node_max_by_floor(4) {
		Some(i) => i as usize,
		None => return Err("Floor is not".into()),
	};
	let floor5 = match node_max_by_floor(5) {
		Some(i) => i as usize,
		None => return Err("Floor is not".into()),
	};

	dist[2][23 + floor2] = 2;
	dist[2][23 + floor2] = 2;
	dist[23][16 + floor2] = 2;

	dist[23 + floor2][1 + floor3] = 2;
	dist[16 + floor2][18 + floor3] = 2;

	dist[1 + floor3][1 + floor4] = 2;
	dist[18 + floor3][17 + floor4] = 2;

	dist[1 + floor4][1 + floor5] = 2;
	dist[17 + floor4][16 + floor5] = 2;

	//Floor 1
	dist[1][2] = 2;
	dist[2][1] = 2;

	dist[1][3] = 2;
	dist[3][1] = 2;

	dist[3][4] = 1;
	dist[4][3] = 1;

	dist[4][5] = 2;
	dist[5][4] = 2;

	dist[5][6] = 2;
	dist[6][5] = 2;

	dist[6][7] = 1;
	dist[7][6] = 1;

	dist[7][8] = 2;
	dist[8][7] = 2;

	dist[8][9] = 1;
	dist[9][8] = 1;

	dist[9][28] = 1;
	dist[28][9] = 1;

	dist[28][10] = 1;
	dist[10][28] = 1;

	dist[10][19] = 3;
	dist[19][10] = 3;

	dist[19][20] = 5;
	dist[20][19] = 5;

	dist[20][21] = 2;
	dist[21][20] = 2;

	dist[21][23] = 1;
	dist[23][21] = 1;

	dist[21][22] = 3;
	dist[22][21] = 3;

	dist[20][24] = 3;
	dist[24][20] = 3;

	dist[24][25] = 2;
	dist[25][24] = 2;

	dist[25][27] = 3;
	dist[27][25] = 3;

	dist[25][26] = 3;
	dist[26][25] = 3;

	dist[27][11] = 3;
	dist[11][27] = 3;

	dist[10][11] = 4;
	dist[11][10] = 4;

	dist[11][12] = 4;
	dist[12][11] = 4;

	dist[13][12] = 4;
	dist[12][13] = 4;

	dist[13][14] = 2;
	dist[14][13] = 2;

	dist[14][15] = 2;
	dist[15][14] = 2;

	dist[15][16] = 3;
	dist[16][15] = 3;

	dist[15][17] = 1;
	dist[17][15] = 1;

	dist[18][17] = 1;
	dist[17][18] = 1;

	dist[18][1] = 5;
	dist[1][18] = 5;
	//
	//Floor 2
	let mut i = floor2;
	dist[1 + i][2 + i] = 1;
	dist[2 + i][1 + i] = 1;

	dist[2 + i][23 + i] = 2;
	dist[23 + i][2 + i] = 2;

	dist[2 + i][3 + i] = 2;
	dist[3 + i][2 + i] = 2;

	dist[3 + i][4 + i] = 1;
	dist[4 + i][3 + i] = 1;

	dist[4 + i][5 + i] = 3;
	dist[5 + i][4 + i] = 3;

	dist[5 + i][6 + i] = 2;
	dist[6 + i][5 + i] = 2;

	dist[7 + i][8 + i] = 2;
	dist[8 + i][7 + i] = 2;

	dist[8 + i][22 + i] = 1;
	dist[22 + i][8 + i] = 1;

	dist[22 + i][9 + i] = 4;
	dist[9 + i][22 + i] = 4;

	dist[10 + i][9 + i] = 7;
	dist[9 + i][10 + i] = 7;

	dist[11 + i][10 + i] = 3;
	dist[10 + i][11 + i] = 3;

	dist[12 + i][11 + i] = 2;
	dist[11 + i][12 + i] = 2;

	dist[13 + i][12 + i] = 2;
	dist[12 + i][13 + i] = 2;

	dist[12 + i][14 + i] = 2;
	dist[14 + i][12 + i] = 2;

	dist[14 + i][2 + i] = 5;
	dist[2 + i][14 + i] = 5;

	dist[22 + i][15 + i] = 8;
	dist[15 + i][22 + i] = 8;

	dist[15 + i][16 + i] = 3;
	dist[16 + i][15 + i] = 3;

	dist[16 + i][17 + i] = 2;
	dist[17 + i][16 + i] = 2;

	dist[17 + i][18 + i] = 1;
	dist[18 + i][17 + i] = 1;

	dist[18 + i][19 + i] = 2;
	dist[19 + i][18 + i] = 2;

	dist[15 + i][20 + i] = 3;
	dist[20 + i][15 + i] = 3;

	dist[20 + i][21 + i] = 5;
	dist[21 + i][20 + i] = 5;

	//Floor 3
	i = floor3;
	dist[1 + i][4 + i] = 2;
	dist[4 + i][1 + i] = 2;

	dist[2 + i][3 + i] = 2;
	dist[3 + i][2 + i] = 2;

	dist[3 + i][4 + i] = 2;
	dist[4 + i][3 + i] = 2;

	dist[4 + i][5 + i] = 2;
	dist[5 + i][4 + i] = 2;

	dist[5 + i][6 + i] = 1;
	dist[6 + i][5 + i] = 1;

	dist[6 + i][7 + i] = 3;
	dist[7 + i][6 + i] = 3;

	dist[7 + i][8 + i] = 2;
	dist[8 + i][7 + i] = 2;

	dist[8 + i][9 + i] = 2;
	dist[9 + i][8 + i] = 2;

	dist[9 + i][10 + i] = 3;
	dist[10 + i][9 + i] = 3;

	dist[10 + i][11 + i] = 2;
	dist[11 + i][10 + i] = 2;

	dist[11 + i][12 + i] = 2;
	dist[12 + i][11 + i] = 2;

	dist[12 + i][13 + i] = 1;
	dist[13 + i][12 + i] = 1;

	dist[12 + i][14 + i] = 1;
	dist[14 + i][12 + i] = 1;

	dist[14 + i][15 + i] = 1;
	dist[15 + i][14 + i] = 1;

	dist[15 + i][16 + i] = 1;
	dist[16 + i][15 + i] = 1;

	dist[16 + i][17 + i] = 3;
	dist[17 + i][16 + i] = 3;

	dist[15 + i][18 + i] = 2;
	dist[18 + i][15 + i] = 2;

	//Floor 4
	i = floor4;
	dist[1 + i][4 + i] = 2;
	dist[4 + i][1 + i] = 2;

	dist[2 + i][3 + i] = 2;
	dist[3 + i][2 + i] = 2;

	dist[3 + i][4 + i] = 2;
	dist[4 + i][3 + i] = 2;

	dist[4 + i][5 + i] = 2;
	dist[5 + i][4 + i] = 2;

	dist[5 + i][6 + i] = 1;
	dist[6 + i][5 + i] = 1;

	dist[6 + i][7 + i] = 3;
	dist[7 + i][6 + i] = 3;

	dist[7 + i][8 + i] = 2;
	dist[8 + i][7 + i] = 2;

	dist[8 + i][9 + i] = 2;
	dist[9 + i][8 + i] = 2;

	dist[9 + i][10 + i] = 3;
	dist[10 + i][9 + i] = 3;

	dist[10 + i][11 + i] = 2;
	dist[11 + i][10 + i] = 2;

	dist[11 + i][12 + i] = 2;
	dist[12 + i][11 + i] = 2;

	dist[12 + i][13 + i] = 1;
	dist[13 + i][12 + i] = 1;

	dist[12 + i][14 + i] = 1;
	dist[12 + i][14 + i] = 1;

	dist[14 + i][15 + i] = 1;
	dist[15 + i][14 + i] = 1;

	dist[15 + i][16 + i] = 3;
	dist[16 + i][15 + i] = 3;

	dist[14 + i][17 + i] = 2;
	dist[17 + i][14 + i] = 2;
	//Floor 5
	i = floor5;
	dist[1 + i][4 + i] = 2;
	dist[4 + i][1 + i] = 2;

	dist[2 + i][3 + i] = 2;
	dist[3 + i][2 + i] = 2;

	dist[3 + i][4 + i] = 2;
	dist[4 + i][3 + i] = 2;

	dist[4 + i][5 + i] = 2;
	dist[5 + i][4 + i] = 2;

	dist[5 + i][6 + i] = 1;
	dist[6 + i][5 + i] = 1;

	dist[6 + i][7 + i] = 3;
	dist[7 + i][6 + i] = 3;

	dist[7 + i][8 + i] = 2;
	dist[8 + i][7 + i] = 2;

	dist[8 + i][9 + i] = 2;
	dist[9 + i][8 + i] = 2;

	dist[9 + i][10 + i] = 3;
	dist[10 + i][9 + i] = 3;

	dist[10 + i][11 + i] = 2;
	dist[11 + i][10 + i] = 2;

	dist[11 + i][12 + i] = 2;
	dist[12 + i][11 + i] = 2;

	dist[12 + i][13 + i] = 1;
	dist[13 + i][12 + i] = 1;

	dist[12 + i][14 + i] = 1;
	dist[14 + i][12 + i] = 1;

	dist[14 + i][15 + i] = 1;
	dist[15 + i][14 + i] = 1;

	dist[14 + i][16 + i] = 2;
	dist[16 + i][14 + i] = 2;

	loop {
		min = virtual_infinity;

		for i in 0..used.len() {
			if !used[i] && min > cost[i] {
				min = cost[i];
				target = i;
			}
		}

		if target == goal as usize {
			distance = cost[goal as usize];
			break;
		}

		for next in 0..cost.len() {
			if cost[next] > dist[target][next] + cost[target] {
				cost[next] = dist[target][next] + cost[target];
				via[next] = target as i32;
			}
		}

		used[target] = true;
	}

	// let record = [];
	// for(let i = 0;true;i++) {
	// node = via[node];
	// record.push(node);
	// if(node === start) break;
	// }
	// record.reverse();
	// record.push(goal);

	let mut node: usize = target;
	let mut record: Vec<u16> = Vec::new();
	loop {
		node = via[node] as usize;
		record.push(node as u16);
		if node as u16 == start {
			break;
		}
	}
	record.reverse();
	record.push(goal as u16);

	Ok((record, distance as usize))
}

fn node_max_by_floor(floor: u8) -> Option<u16> {
	match floor {
		1 => Some(0),
		2 => Some(28),
		3 => Some(node_max_by_floor(2)? + 23),
		4 => Some(node_max_by_floor(3)? + 18),
		5 => Some(node_max_by_floor(4)? + 17),
		_ => None,
	}
}
fn node_floor(node: u16) -> Option<u8> {
	if node <= node_max_by_floor(2)? {
		Some(1)
	} else if node_max_by_floor(2)? <= node && node <= node_max_by_floor(3)? {
		Some(2)
	} else if node_max_by_floor(3)? <= node && node <= node_max_by_floor(4)? {
		Some(3)
	} else if node_max_by_floor(4)? <= node && node <= node_max_by_floor(5)? {
		Some(4)
	} else if node_max_by_floor(5)? <= node && node <= node_max_by_floor(5)? + 16 {
		Some(5)
	} else {
		None
	}
}
