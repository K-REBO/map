function addToForm(num) {//This code is good?
	let DOM,value;
	if(num === 0)//clear form
	{
		value = "";
	}
	else
	{
		DOM = document.getElementById("suggest" + num);
		value = DOM.dataset.val;
	}
	DOM = document.getElementById("debug_input")
	DOM.value = String(value);
}


console.log(localStorage.getItem("destination"));



function debug_enter() {
	let value = convertWordToNord();
	// let value = Number(document.getElementById("debug_input").value);
	if(value > 28  || value < 0 || isNaN(value)) {
		return "ERROR";
	}
	ctx.clearRect(0,0,canvas_width,canvas_height);
	draw.drawMap(ctx, map, FloorNow);
	
	console.log(whrereNode,value);

	localStorage.setItem("destination",value);

	draw.drawRoute(ctx, map, Number(whrereNode),Number(value), FloorNow);
	SPA("search","main");
}

function getNodeByPlace(place) {
	let Node;
	let Nearest = (word) => {
		switch(word) {
			case "トイレ":
		}
	};

	switch(place) {
		case "トイレ":Node = 3;break;
		case "食堂": Node = 1;break;
		case "生徒相談室":Node = 18;break;
		case "調整室":Node = 16;break;
		case "放送室":Node = 16;break;
		case "パソコン室":Node = 4;break;
		case "中庭":Node = 15;break;
		case "校長室":Node = 26;break;
		case "玄関":Node = 20;break;
		case "昇降口":Node = 22;break;
		case "事務室":Node = 24;break;
		case "生徒ホール":Node = 27;break;
		case "美術室":Node = 12;break;
		case "書道室":Node = 14;break;
		case "保健室":Node = 16;break;
		case "湯沸室":Node = 5;break;
		default: {
			console.log("can't match any place");
		}break;
	}
	return Node;
}

function convertWordToNord() {
	let word = document.getElementById("debug_input").value;
	let Node = getNodeByPlace(word);
	return Node;
}

