let utils = {
	NodeFloor:(node) => {
		if(0 < node && node <= utils.FloorVariation(2)) {
			return 1;
		}
		else if(utils.FloorVariation(2) < node && node <= utils.FloorVariation(3)){
			return 2
		}
		else if(utils.FloorVariation(3) < node && node <= utils.FloorVariation(4)){
			return 3;
		}
		else if(utils.FloorVariation(4) < node && node <= utils.FloorVariation(5)){
			return 4;
		}
		else if(utils.FloorVariation(5) < node) {
			return 5;
		}

	},
	FloorVariation : (Floor) => {
		switch(Floor) {
			case 1:return 0;
			case 2:return 28;
			case 3:return utils.FloorVariation(2) + 23;
			case 4:return utils.FloorVariation(3) + 18;
			case 5:return utils.FloorVariation(4) + 17;
			default:return -1000;
		}
	},
};


function dijkstra(start, goal) {
	const SIZE = 1000;
	let via = new Array(SIZE);

	let dist = new Array();
	let cost = new Array(SIZE);

	let node_num;
	let used = new Array(SIZE);

	//initialize variables
	for(let i = 0; i < SIZE;i++) {
		dist.push(new Array(SIZE));
	}
	//end

	if(start === goal) {
		console.log("same point")
		return 0;
	}


	for(let i = 0; i < SIZE; i++) {
		cost[i] = Infinity;
		used[i] = false;
		via[i] = -1;
		for(let j = 0; j < SIZE; j++) {
			dist[i][j] = Infinity;
		}
	}

	node_num = 1000;

	// connect Floor
	dist[2][23 + utils.FloorVariation(2)] = 2;
	dist[23][16 + utils.FloorVariation(2)] = 2;

	dist[23 + utils.FloorVariation(2)][1 + utils.FloorVariation(3)] = 2;
	dist[16 + utils.FloorVariation(2)][18 + utils.FloorVariation(3)] = 2;

	dist[1 + utils.FloorVariation(3)][1 + utils.FloorVariation(4)] = 2;
	dist[18 + utils.FloorVariation(3)][17 + utils.FloorVariation(4)] = 2;

	dist[1 + utils.FloorVariation(4)][1 + utils.FloorVariation(5)] = 2;
	dist[17 + utils.FloorVariation(4)][16 + utils.FloorVariation(5)] = 2;


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
		let i = utils.FloorVariation(2);
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
		i = utils.FloorVariation(3);
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

	//Florr 4
		i = utils.FloorVariation(4);
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
		i = utils.FloorVariation(5);
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


	let min, target;
	let distance;
	cost[start] = 0;

	while(true) {
		min = Infinity;
		for(let i = 0; i < node_num; i++) {
			if(!used[i] && min > cost[i]) {
				min = cost[i];
				target = i;
			}
		}

		if(target == goal) {
			distance =  cost[goal];
			break;
		}

		for(let next = 0; next < node_num; next++) {
			if(cost[next] > dist[target][next] + cost[target]) {
				cost[next] = dist[target][next] + cost[target];
				via[next] = target;
			}
		}

		used[target] = true;
	}
	return via;
}