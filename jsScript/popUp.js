class popUp {
	/// ## usage example
	///
	/// ```javascript
	/// let Modal = [
	/// 			{
	/// 				title : "What's new",
	/// 				content : `
	/// 							<span class="text-black-4xl">
	/// 								we add chat and you able to talk with people!
	/// 							</span>`
	/// 			}
	/// 		];
	/// let PU = new popUp(Modal);
	/// PU.generatePage();
	/// PU.getDOM();
	///```


	//sorry for being too crude.
	constructor(page, variableName) {
		this.variableName = variableName;
		this.page = page;
		this.HTML = '';
		this.bgcolor = "white";
		this.pageNumNow = 0;
	}
	generatePage() {
		this.HTML =  `
			<div class="flex justify-center">
				<div class="flex justify-center fixed" style="bottom:0px;">
					<div class="rounded-t-lg bg-white" style="height:min-content">
						<button class="justify-self-start fixed top-auto" onclick=" ` + this.variableName + `.delete()">
							<svg class="w-12 h-12" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
								<path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 
								111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 
								1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
							</svg>
						</button>
						<div class="flex-row flex justify-center">
							<span class="justify-self-center text-4xl" id="popUp_title">` + this.page[0].title + `</span>
						</div>
						<div class="grid">
							<progress value="1" min="0" max="` +  this.page.length+ `" class="w-16 h-2 justify-self-end" style="background-color: yellow;"></progress>
						</div>
						<div id="popUp_content" class="text-3xl flex flex-col">`
						+
						this.page[0].content
						+
						`
						</div>
						<div class="grid">
							<button class="text-5xl justify-self-end" onclick="` + this.variableName + `.nextPage()">Next</button>
						</div>
					</div>
				</div>
			</div>`;
	}
	delete() {
		let DOM = document.getElementById("popUpPage");
		DOM.innerHTML = "";	
	}
	nextPage() {
		this.switchPage(++this.pageNumNow);
		console.log(this.pageNumNow);
	}
	switchPage(pageNum) {
		pageNum = Number(pageNum);
		console.log("pageNum",this.pageNumNow);
		if(this.pageNumNow === this.page.length) {
			this.delete();
			console.log("ll");
		}

		else if(0 <= pageNum && pageNum <= this.page.length) {
			let DOM = document.getElementById("popUpPage");

			
			let NextOrEndButton = "Next";

			if(this.pageNumNow + 1 === this.page.length) {
				NextOrEndButton = "End";
			}





			DOM.innerHTML = `
				<div class="flex justify-center">
					<div class="flex justify-center fixed" style="bottom:0px;">
						<div class="rounded-t-lg bg-white" style="height:min-content;">
							<button class="justify-self-start fixed top-auto" onclick=" ` + this.variableName + `.delete()">
								<svg class="w-12 h-12" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
									<path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 
									111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 
									1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
								</svg>
							</button>
							<div class="flex-row flex justify-center">
								<span class="justify-self-center text-4xl" id="popUp_title">` + this.page[pageNum].title + `</span>
							</div>
							<div class="grid">
								<progress value="` +  (pageNum+1) + `" min="0" max="` +  this.page.length + `" class="w-16 h-2 justify-self-end" style="background-color: yellow;"></progress>
							</div>
							<div id="popUp_content" class="text-3xl flex flex-col">`
							+
							this.page[pageNum].content
							+
							`
							</div>
							<div class="grid">
								<button class="text-5xl justify-self-end" onclick="` + this.variableName + `.nextPage()">` + NextOrEndButton + `</button>

							</div>
						</div>
					</div>
				</div>`;

		}
		else {
			console.log("err: In popUp.switchPage(); pageNum is too large or Negative");
		}
	}
	getDOM() {
		return this.HTML;
	}
};



	
let PP = new popUp(
	[
		{
			title : "　　ようこそ!　パブリックアルファテストへ　　",
			content : `
						<div class="grid justify-itmes-center">
							<span class="text-black-4xl">
									現在実装されているのは一階のみです。
							</span>
						</div>
						`

		},
		{
			title : "&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp&nbsp",
			content : `
						<span class="text-black-4xl flex-col justify-center">
							バグや提案がある場合、2228まで。
						</span>`
		}
	],"PP"
);

let DOM = document.getElementById("popUpPage");
DOM.setAttribute("class","display_default");
PP.generatePage();
DOM.innerHTML = PP.getDOM();