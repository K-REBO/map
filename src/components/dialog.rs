use crate::modules::route::dijkstra;
use serde::{Deserialize, Serialize};
use std::u16;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
	format::{Json, Nothing},
	prelude::*,
	services::StorageService,
};

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
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlaceInfo {
	title: String,
	title_ruby: String,
	classroom: String,
	place_node: Option<u32>,
	floor: Option<u8>,
	description: String,
	social_url: Option<String>
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
		Self {
			link,
			page: Page::Detail,
			props,
			place_info: PlaceInfo {
				title: "Title".to_string(),
				title_ruby: "Ruby".to_string(),
				classroom: "Network Error Occured".to_string(),
				floor: None,
				place_node:None,
				description: "Example: put here some description;".to_string(),
				social_url:None,
			},
			fetch_task: None,
		}
	}

	fn rendered(&mut self, first_render: bool) {
		if first_render {
			let request = Request::get("http://0.0.0.0:8080/test")
				.body(Nothing)
				.expect("Could not build that request");
			let callback = self.link.callback(
				|response: Response<Json<Result<PlaceInfo, anyhow::Error>>>| {
					let Json(data) = response.into_body();
					Msg::ReceiveResponse(data)
				},
			);
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
			}
			Msg::ReceiveResponse(response) => match response {
				Ok(result) => {
					self.place_info = result;
				}
				Err(error) => {
					log::error!("{}", &error);
				}
			},
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
					},
					Page::Detail => html! {
						<div class="flex justify-center" style="z-index:100;">
							<div style="width:90%;background-color: black; color:white; bottom: 0px;" class="rounded-2xl fixed text-center">
								<div class="text-2xl">
									<ruby style="color:hsl(141, 53%, 53%);">
										{self.place_info.title.clone()}
										<rt>{self.place_info.title_ruby.clone()}</rt>
									</ruby>
								</div>
								<div class="">{self.place_info.classroom.clone()}</div>
								<div>{self.place_info.description.clone()}</div>
								{
									match &self.place_info.social_url {
										Some(url) => html! {
											<div class="w-8 mx-auto bg-white rounded-2xl">
												<a href={format!("\"{}\"", url)}>
													<svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" ml-update="aware">
														<script xmlns=""/>
														<path d="M12 0C8.74 0 8.333.015 7.053.072 5.775.132 4.905.333 4.14.63c-.789.306-1.459.717-2.126 1.384S.935 3.35.63 4.14C.333 4.905.131 5.775.072 7.053.012 8.333 0 8.74 0 12s.015 3.667.072 4.947c.06 1.277.261 2.148.558 2.913.306.788.717 1.459 1.384 2.126.667.666 1.336 1.079 2.126 1.384.766.296 1.636.499 2.913.558C8.333 23.988 8.74 24 12 24s3.667-.015 4.947-.072c1.277-.06 2.148-.262 2.913-.558.788-.306 1.459-.718 2.126-1.384.666-.667 1.079-1.335 1.384-2.126.296-.765.499-1.636.558-2.913.06-1.28.072-1.687.072-4.947s-.015-3.667-.072-4.947c-.06-1.277-.262-2.149-.558-2.913-.306-.789-.718-1.459-1.384-2.126C21.319 1.347 20.651.935 19.86.63c-.765-.297-1.636-.499-2.913-.558C15.667.012 15.26 0 12 0zm0 2.16c3.203 0 3.585.016 4.85.071 1.17.055 1.805.249 2.227.415.562.217.96.477 1.382.896.419.42.679.819.896 1.381.164.422.36 1.057.413 2.227.057 1.266.07 1.646.07 4.85s-.015 3.585-.074 4.85c-.061 1.17-.256 1.805-.421 2.227-.224.562-.479.96-.899 1.382-.419.419-.824.679-1.38.896-.42.164-1.065.36-2.235.413-1.274.057-1.649.07-4.859.07-3.211 0-3.586-.015-4.859-.074-1.171-.061-1.816-.256-2.236-.421-.569-.224-.96-.479-1.379-.899-.421-.419-.69-.824-.9-1.38-.165-.42-.359-1.065-.42-2.235-.045-1.26-.061-1.649-.061-4.844 0-3.196.016-3.586.061-4.861.061-1.17.255-1.814.42-2.234.21-.57.479-.96.9-1.381.419-.419.81-.689 1.379-.898.42-.166 1.051-.361 2.221-.421 1.275-.045 1.65-.06 4.859-.06l.045.03zm0 3.678c-3.405 0-6.162 2.76-6.162 6.162 0 3.405 2.76 6.162 6.162 6.162 3.405 0 6.162-2.76 6.162-6.162 0-3.405-2.76-6.162-6.162-6.162zM12 16c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4zm7.846-10.405c0 .795-.646 1.44-1.44 1.44-.795 0-1.44-.646-1.44-1.44 0-.794.646-1.439 1.44-1.439.793-.001 1.44.645 1.44 1.439z"/>
													</svg>
												</a>
											</div>			
										},
										None => html!{}
									}
								}
								<div class="text-left">
									<span>{"評価とレビュー"}</span>
								//	<div>
								//		<span>{"総合"}</span>
								//		<div class="flex">
								//			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6">
								//				<path fill="none" d="M0 0h24v24H0z"/>
								//				<path d="M12 18.26l-7.053 3.948 1.575-7.928L.587
								//				8.792l8.027-.952L12 .5l3.386 7.34 8.027.952-5.935
								//				5.488 1.575 7.928L12 18.26zm0-2.292l4.247 2.377-.949-4.773
								//				3.573-3.305-4.833-.573L12 5.275l-2.038 4.42-4.833.572
								//				3.573 3.305-.949 4.773L12 15.968z"/>
								//			</svg>
								//			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6">
								//				<path fill="none" d="M0 0h24v24H0z"/>
								//				<path d="M12 15.968l4.247 2.377-.949-4.773 3.573-3.305-4.833-.573L12
								//				5.275v10.693zm0 2.292l-7.053 3.948 1.575-7.928L.587
								//				8.792l8.027-.952L12 .5l3.386 7.34 8.027.952-5.935 5.488 1.575
								//				7.928L12 18.26z" fill="rgba(0,0,0,1)"/>
								//			</svg>
								//		</div>
								//	</div>
								//	<div>
								//		<span>{"ストーリー"}</span>
								//		<div class="flex">
								//			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6">
								//				<path fill="none" d="M0 0h24v24H0z"/>
								//				<path d="M12 18.26l-7.053 3.948 1.575-7.928L.587
								//				8.792l8.027-.952L12 .5l3.386 7.34 8.027.952-5.935
								//				5.488 1.575 7.928L12 18.26zm0-2.292l4.247 2.377-.949-4.773
								//				3.573-3.305-4.833-.573L12 5.275l-2.038 4.42-4.833.572
								//				3.573 3.305-.949 4.773L12 15.968z"/>
								//			</svg>
								//		</div>
								//	</div>
								//	<div>
								//		<span>{"装飾"}</span>
								//		<div class="flex">
								//			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6">
								//				<path fill="none" d="M0 0h24v24H0z"/>
								//				<path d="M12 18.26l-7.053 3.948 1.575-7.928L.587
								//				8.792l8.027-.952L12 .5l3.386 7.34 8.027.952-5.935
								//				5.488 1.575 7.928L12 18.26zm0-2.292l4.247 2.377-.949-4.773
								//				3.573-3.305-4.833-.573L12 5.275l-2.038 4.42-4.833.572
								//				3.573 3.305-.949 4.773L12 15.968z"/>
								//			</svg>
								//		</div>
								//	</div>
								//	<div>
								//		<span>{"待ち時間"}</span>
								//		<div class="flex">
								//			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-6">
								//				<path fill="none" d="M0 0h24v24H0z"/>
								//				<path d="M12 18.26l-7.053 3.948 1.575-7.928L.587
								//				8.792l8.027-.952L12 .5l3.386 7.34 8.027.952-5.935
								//				5.488 1.575 7.928L12 18.26zm0-2.292l4.247 2.377-.949-4.773
								//				3.573-3.305-4.833-.573L12 5.275l-2.038 4.42-4.833.572
								//				3.573 3.305-.949 4.773L12 15.968z"/>
								//			</svg>
								//		</div>
								//	</div>
								//</div>
								//<div class="rounded text-center bg-gray-500" style="width: 80%; height: 10ex;">
									<div>{"今夜作ります"}</div>
								</div>
								<button
								// style="float: right;"
								onclick=self.link.callback(|_| Msg::CloseSummary)>
									<svg xmlns="http://www.w3.org/2000/svg" class="w-12" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="3.5" strokeLinecap="round" strokeLinejoin="round">
										<line x1="18" y1="6" x2="6" y2="18"/>
										<line x1="6" y1="6" x2="18" y2="18"/>
									</svg>
								</button>
							</div>
						</div>			
					},
				}
			}
			</div>
		}
	}
}
