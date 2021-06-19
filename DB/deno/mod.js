import "https://cdn.jsdelivr.net/npm/faunadb@latest/dist/faunadb-min.js";

const hashedColor = new Map();
hashedColor["a"] = 0;
const defaultHeader = {
	"status": 200,
	"content-type": "application/json; charset=UTF-8",
	"Access-Control-Allow-Origin": "*",
};

let token = "fnAEKzkv1_ACCc4OUHXIzFl9kmDmlciDssZ4QnsH";
// let token = Deno.env.get("FAUNA_TOKEN");
if(!token) throw new Error("No fauna token");

let client = new faunadb.Client({secret: token}), q = faunadb.query;
client.query(
	q.Create(
		q.Collection('test'),
		{
			data: {
				title: `Deno`,
			}
		},
	)
)
.then((ret) => console.log(ret));




// client.query(
	// q.Get(q.Ref(q.Collection('test'), '300398276528570890'))
// )
// .then((ret) => console.log(ret))



client.query(
	q.Get(
	  q.Match(q.Index('teee'), 'Deno')
	)
)
.then((ret) => console.log(ret))
  
  
  
  
  

async function handleRequest(request) {
	const { pathname } = new URL(request.url);

	if(pathname.startsWith("/getByColor")) {
		const json = await request.json();
		const res = JSON.stringify({
			node: hashedColor[String(json.color + json.floor)],
		});
		
		return new Response(res, {
			headers: defaultHeader,
		});
	} else if(pathname.startsWith("/menu")) {
		const res = JSON.stringify({

		});

		return new Response(res, {
			headers: defaultHeader,
		});
	}else if(pathname.startsWith("/place")) {
		const json = await request.json();


		const res = JSON.stringify({
			title: "Dream today, be sure to diee",
			place_name: "32HR",
			description: "Hello from cors",
			waitting_time_minute: 10,
		});

		return new Response(res, {
			headers: defaultHeader,
		});
	} else if(pathname.startsWith("/event")) {
	} else if(pathname.startsWith("/IS")) {
	} else if(pathname.startsWith("/review")) {

	} else if(pathname.startsWith("/popMusic")) {

	} else {
		const res = JSON.stringify({
			health: "OK",
		});

		return new Response(res, {
			headers: defaultHeader,
		});
	}
}

addEventListener("fetch", (event) => {
	event.respondWith(handleRequest(event.request));
});
