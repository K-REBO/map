export function clear_inner_by_id(ID){
	ID = String(ID);
	let DOM = document.getElementById(ID);
	DOM.value = "";
}