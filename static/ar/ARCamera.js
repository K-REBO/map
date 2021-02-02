
AFRAME.registerComponent('markerhandler', {
		init: function() {
			this.el.sceneEl.addEventListener('markerFound', () => {
				// get AR_code from HTML-data-property
				let [L_pos,R_pos] = this.el.dataset['location'].split(',');

				let ID = this.el.id;

				let destination = Number(localStorage.getItem("goal"));

				console.log("ID:",ID," X:",L_pos," Y:",R_pos," destination:",destination);
				if(!isNaN(destination)) {
					let Edge = (L_pos,R_pos);
					let cursor = Route_dir(Edge,destination);
					
					console.log("Route Direction",cursor);
					set(ID);
					rotateObj(cursor,ID);
				}

			})
		}
});
function rotateObj(cursor,ID) {
	let DOM = document.getElementsByName(ID);
	if(cursor === "Left")
		DOM.innerHTML = `<a-obj-model id="HIRO_obj" src="#arrow_obj" position="0 0 0" color="#e0ff45" rotation="0 180 0"></a-obj-model>`;
	if(cursor === "Right")
		DOM.innerHTML = `<a-obj-model id="HIRO_obj" src="#arrow_obj" position="0 0 0" color="#e0ff45" rotation="0 0 0"></a-obj-model>`;
}

function set(ID) {
	let content = `<a-obj-model id="HIRO_obj" src="#arrow_obj" position="0 0 0" color="#e0ff45" rotation="0 180 0"></a-obj-model>`;
	let DOM = document.getElementById(ID);
		DOM.innerHTML = content;
}

function moveTo() {
	window.location.href = "https://oberk.dev/map";
}
let Edges = [1,3];
let DIR = Route_dir(Edges,28)

function Route_dir(Edge,goal) {
	let [LNode, RNode] = Edge;
	{
		let Route_via = dijkstra(LNode, goal);
		let start = LNode;


		let node = goal;
		while(true) {
			node = Route_via[node];
			if(node === RNode) return "Right";
			if(node === start) break;

		}
	}
	{
		let Route_via = dijkstra(RNode, goal);
		let start = RNode;


		let node = goal;
		while(true) {
			node = Route_via[node];
			if(node === LNode) return "Left";
			if(node === start) break;
		}
	}

}

