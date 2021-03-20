use crate::api::*;
use crate::constants::*;
use crate::services::cookie::CookieService;
use crate::services::router;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use wasm_bindgen::prelude::*;
use yew::{
    format::{Json, Nothing},
    prelude::*,
};
use yew_material::list::*;
use yew_material::text_inputs::*;
use yew_material::{
    MatButton, MatFormfield, MatList, MatSnackbar, MatTextField, WeakComponentLink,
};
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

pub struct ListPostsPage {
    link: ComponentLink<Self>,
    fetch: FetchState<ResponseBlock<PostsResponse>>,
    fetch_counts: FetchState<ResponseBlock<CountPostsResponse>>,
    list_link: WeakComponentLink<MatList>,
    page: i64,
    count: i64,
}

pub enum Msg {
    GetPostHeaders,
    GetCounts,
    ReceivePostHeadersResponse(FetchState<ResponseBlock<PostsResponse>>),
    ReceiveCountsResponse(FetchState<ResponseBlock<CountPostsResponse>>),
    NextPage,
    PreviousPage,
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for ListPostsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Props, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch: FetchState::NotFetching,
            fetch_counts: FetchState::NotFetching,
            list_link: WeakComponentLink::default(),
            page: 0,
            count: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetCounts => {
                let future = async move {
                    match get_post_counts().await {
                        Ok(info) => Msg::ReceiveCountsResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveCountsResponse(FetchState::Failed(FetchError::from(
                            JsValue::FALSE,
                        ))),
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::GetPostHeaders => {
                let page = self.page;
                let future = async move {
                    match posts(page * MAX_LIST_POSTS, MAX_LIST_POSTS).await {
                        Ok(info) => Msg::ReceivePostHeadersResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceivePostHeadersResponse(FetchState::Failed(
                            FetchError::from(JsValue::FALSE),
                        )), // TODO
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::ReceiveCountsResponse(data) => {
                self.fetch_counts = data;
                if let FetchState::Success(r) = self.fetch_counts.clone() {
                    if let Some(body) = r.body {
                        self.count = body.count;
                    }
                }
                true
            }
            Msg::ReceivePostHeadersResponse(data) => {
                self.fetch = data;
                true
            }
            Msg::NextPage => {
                if (self.page + 1) * MAX_LIST_POSTS < self.count {
                    self.page += 1;
                    self.fetch = FetchState::NotFetching;
                    true
                } else {
                    false
                }
            }
            Msg::PreviousPage => {
                if self.page > 0 {
                    self.page -= 1;
                    self.fetch = FetchState::NotFetching;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let FetchState::NotFetching = self.fetch_counts.clone() {
            self.link.send_message(Msg::GetCounts);
        }
        let list = if let FetchState::Success(r) = self.fetch.clone() {
            if let Some(body) = r.body {
                body.posts
            } else {
                vec![]
            }
        } else if let FetchState::NotFetching = self.fetch.clone() {
            self.link.send_message(Msg::GetPostHeaders);
            vec![]
        } else {
            vec![]
        };
        html! {
            <div class="container">
            <div class="block">
                <h3>{"Posts"}</h3>
                <MatList list_link=self.list_link.clone()>
                    {
                        for list.iter().map(|post_header| {
                            html! {
                                <router::MainRouterAnchor route=router::MainRoute::ViewPost(post_header.id as i64)><MatListItem>{&post_header.title}</MatListItem></router::MainRouterAnchor>
                            }
                        })
                    }
                </MatList>
                <div class="button-grid">
                    <span onclick=self.link.callback(|_| Msg::PreviousPage)><MatButton disabled=(self.page == 0) label="Previous"/></span>
                    <span onclick=self.link.callback(|_| Msg::NextPage)><MatButton disabled=((self.page+1)*MAX_LIST_POSTS >= self.count) label="Next"/></span>
                </div>
                </div>
            </div>
        }
    }
}
