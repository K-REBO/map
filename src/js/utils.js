export function clear_inner_by_id(ID) {
	ID = String(ID);
	let DOM = document.getElementById(ID);
	DOM.value = "";
}
const shareData = {
	title: 'MDN',
	text: 'MDN でウェブ開発を学びましょう。',
	url: 'https://developer.mozilla.org',
}

export async function share() {
	try {
		await navigator.share(
			{
				title : "Map",
				url : "https://K-REBO/Map",
			}
		);
		resultPara.textContent = "Map shared successfully";
	}
	catch (err) {
		resultPara.textContent = 'Error: ' + e;
	}
}