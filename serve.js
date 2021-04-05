import { serve } from "https://deno.land/std@0.91.0/http/server.ts";
const s = serve({ port: 18080 });

console.log("http://localhost:18080");

for await (const req of s) {
	let rgba = req.url.slice(1).split("/");
	rgba = rgba.map(x => Number(x));
	let node = getNodeByRGBA(rgba);
	let info = getInfoByNode(node);
	console.log("node: ", node, "info: ", info);


	req.respond(
		{
			status: 200,
			headers: new Headers({
				"content-type": "application/json",
				"Access-Control-Allow-Origin": "*",
			}),
			body: {
				title: "TEST EXAMPLE",
				place_name: "校長室",
				description: "hEllO, wOrld!",
				waitting_time_minute: 10,		
			},
		}
	);
};

function getInfoByNode(node) {
	let HashInfo = new Map();
	HashInfo[1] = {
		title: "TEST EXAMPLE",
		place_name: "校長室",
		description: "hEllO, wOrld!",
		waitting_time_minute: 10,
	};

	return HashInfo[1];
}


function getNodeByRGBA(r,g,b,a) {
	let node = 1;
	let HashColor = new Map();
	
	let hashedKey = [255,0,0,1,1];
	HashColor[hashedKey[1]] = 1;

	return node;
}