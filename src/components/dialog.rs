use crate::modules::route::dijkstra;
use std::u16;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::StorageService,
};
use serde::{Deserialize, Serialize};



#[derive(Properties, Clone, PartialEq)]
pub struct Props {
	pub on_close: Callback<()>,
	pub on_add_goal: Callback<u16>,

	pub goal: Option<u16>,
	pub start: Option<u16>,
}

#[derive(Debug)]
pub enum Page {
	Summary,
	Detail,
}
pub struct Dialog {
	link: ComponentLink<Self>,
	page: Page,
	props: Props,
	place_info: PlaceInfo,
	fetch_task: Option<FetchTask>,
	cost: Option<usize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlaceInfo {
	title: String,
	place_name: String,
	description: String,
	waitting_time_minute: u16,// use chrono or something to use time type? so sleepy
}

pub enum Msg {
	MovePage(Page),
	CloseSummary,
	AddGoal,
    ReceiveResponse(Result<PlaceInfo, anyhow::Error>),
}

impl Component for Dialog {
	type Message = Msg;
	type Properties = Props;
	fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
		let mut cost: Option<usize> = None;
		if props.start.is_some() && props.goal.is_some() {
			match dijkstra(props.start.unwrap(), props.goal.unwrap()) {
				Ok((_, c)) => {
					cost = Some(c);
				}
				Err(e) => {
					log::error!("Can't find in Dijkstra: {:#?}", e);
				}
			};
		};
		Self {
			link,
			page: Page::Summary,
			props,
			cost,
			place_info: PlaceInfo {
				title: "Title".to_string(),
				place_name: "11HR".to_string(),
				description: "Example: put here some description;".to_string(),
				waitting_time_minute: 10,
			},
			fetch_task: None,
		}
	}

	fn rendered(&mut self, first_render: bool) {
		if first_render {
			let request = Request::get("http://localhost:18080")
			.body(Nothing)
			.expect("Could not build that request");
			let callback =
				self.link
					.callback(|response: Response<Json<Result<PlaceInfo, anyhow::Error>>>| {
						let Json(data) = response.into_body();
						Msg::ReceiveResponse(data)
					});
			let task = FetchService::fetch(request, callback).expect("fail");
			self.fetch_task = Some(task);
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::MovePage(page) => {
				log::info!("MovePage");
				self.page = page;
			}
			Msg::CloseSummary => {
				log::info!("CloseSummary");
				self.props.on_close.emit(());
			}
			Msg::AddGoal => {
				if let Some(goal) = self.props.goal {
					self.props.on_add_goal.emit(goal);
				} else {
					log::error!("Wow, Goal is not set");
				}
			},
			Msg::ReceiveResponse(response) =>  match response {
				Ok(result) => {
					log::info!("{:#?}",result);
				},
				Err(error) => {
					log::error!("{}", &error);
				}

			}
		}
		true
	}

	fn change(&mut self, props: Self::Properties) -> ShouldRender {
		if props != self.props {
			self.props = props;
			true
		} else {
			false
		}
	}

	fn view(&self) -> Html {
		html! {
			<div class="flex justify-center flex-col">
			{
				match self.page {
					Page::Summary => html! {
						<div class="flex fixed justify-self-center" style="bottom: 23%; left: calc(50% - 95ex/2); z-index: 100;">
							<section class="fixed bg-black text-white rounded-xl" style="width:95ex; height:17ex;">
								<button
									style="float: right;"
									onclick=self.link.callback(|_| Msg::CloseSummary)
								>
									<svg xmlns="http://www.w3.org/2000/svg" style="width: 7ex; height: 7ex;" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="3.5" strokeLinecap="round" strokeLinejoin="round">
										<line x1="18" y1="6" x2="6" y2="18"/>
										<line x1="6" y1="6" x2="18" y2="18"/>
									</svg>
								</button>
								<button
									onclick=self.link.callback(|_| Msg::MovePage(Page::Detail))
									class="absolute"
									style="padding-top: 1ex; width: 92%; height: 100%;"
								>
									<h1 class="text-6xl">
										{self.place_info.place_name.clone()}
									</h1>
									<h2 class="float-right">
										<span class="text-3xl" style="color: hsl(141, 53%, 53%);">
											{
												match self.cost {
													Some(c) =>html!{
														<>
														{
															c / 5
														}
														{"秒"}
														</>

													},
													None => html!{"Not set start"}
												}
											}
										</span>
										<span class="text-3xl">
											{
												match self.cost {
													Some(c) =>html!{
														<>
														{
															c / 3
														}
														{"m"}
														</>

													},
													None => html!{"Not set start"}
												}
											}
										</span>
									</h2>
								</button>
							</section>
						</div>
					},
					Page::Detail => html! {
					<div class="fixed" style="bottom: 37%; z-index: 100;">
						<section class="text-center w-screen absolute" style="background-color: #3c3e5c; border-top-left-radius: 3rem; border-top-right-radius: 3rem;">
							<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="rounded-2xl bg-red-200 mx-auto border-4 -my-24 w-48 h-48">
								<path fill-rule="evenodd" d="M20.322.75a10.75 10.75 0 00-7.373 2.926l-1.304 1.23A23.743 23.743 0 0010.103 6.5H5.066a1.75 1.75 0 00-1.5.85l-2.71 4.514a.75.75 0 00.49 1.12l4.571.963c.039.049.082.096.129.14L8.04 15.96l1.872 1.994c.044.047.091.09.14.129l.963 4.572a.75.75 0 001.12.488l4.514-2.709a1.75 1.75 0 00.85-1.5v-5.038a23.741 23.741 0 001.596-1.542l1.228-1.304a10.75 10.75 0 002.925-7.374V2.499A1.75 1.75 0 0021.498.75h-1.177zM16 15.112c-.333.248-.672.487-1.018.718l-3.393 2.262.678 3.223 3.612-2.167a.25.25 0 00.121-.214v-3.822zm-10.092-2.7L8.17 9.017c.23-.346.47-.685.717-1.017H5.066a.25.25 0 00-.214.121l-2.167 3.612 3.223.679zm8.07-7.644a9.25 9.25 0 016.344-2.518h1.177a.25.25 0 01.25.25v1.176a9.25 9.25 0 01-2.517 6.346l-1.228 1.303a22.248 22.248 0 01-3.854 3.257l-3.288 2.192-1.743-1.858a.764.764 0 00-.034-.034l-1.859-1.744 2.193-3.29a22.248 22.248 0 013.255-3.851l1.304-1.23zM17.5 8a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zm-11 13c.9-.9.9-2.6 0-3.5-.9-.9-2.6-.9-3.5 0-1.209 1.209-1.445 3.901-1.49 4.743a.232.232 0 00.247.247c.842-.045 3.534-.281 4.743-1.49z"/>
							</svg>
							<br/>
							<div class="mt-24 text-white mr-10">
								<span  class="text-5xl">
									{self.place_info.place_name.clone()}
								</span>
								<br/>
								<br/>
								<span>
								{self.place_info.description.clone()}
								</span>
								<br/>
								<h2 class="float-right">
									<span class="text-3xl" style="color: hsl(141, 53%, 53%);">
										{
											match self.cost {
												Some(c) =>html!{
													<>
													{
														c / 5
													}
													{"秒"}
													</>

												},
												None => html!{"Not set start"}
											}
										}
									</span>
									<br/>
									<span class="text-3xl">
										{
											match self.cost {
												Some(c) =>html!{
													<>
													{
														c / 3
													}
													{"m"}
													</>

												},
												None => html!{"Not set start"}
											}
										}
									</span>
									<br/>
									<div class="text-white text-3xl">
									{"待ち時間: "}
									{
										self.place_info.waitting_time_minute
									}
									{"分"}
									</div>	
								</h2>
							</div>
							<br/>
							<br/>
							<br/>
							<button
								onclick=self.link.callback(|_| Msg::AddGoal)
								class="text-white text-4xl rounded-full"
								style="background-color: #02c695; width: 25rem; height: 4ex;"
							>
								{"Get Direction"}
							</button>
						</section>
					</div>

					},
				}
			}
			</div>
		}
	}
}
