export function clear_inner_by_id(ID){
	ID = String(ID);
	let DOM = document.getElementById(ID);
	DOM.value = "";
}
export function get_node(x,y) {
	console.log(x,y);
	let canvas = document.querySelector("#canvas").getContext("2d");
	let data = canvas.getImageData(x, y, 1, 1).data;
	let rbga = [data[0], data[1], data[2], data[3] / 255];
	return rbga;
}