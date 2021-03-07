use yew::prelude::*;

use crate::api::*;
use crate::constants::*;
use crate::pages::post_preview::PostPreview;
use std::cmp::min;
use wasm_bindgen::prelude::*;
use yew_material::list::*;
use yew_material::{MatButton, MatList};

pub struct Main {
    link: ComponentLink<Self>,
    fetch: FetchState<ResponseBlock<CountPostsResponse>>,
}

pub enum Msg {
    GetCountPosts,
    ReceiveCountPosts(FetchState<ResponseBlock<CountPostsResponse>>),
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch: FetchState::NotFetching,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetCountPosts => {
                self.fetch = FetchState::Fetching;
                let future = async move {
                    match get_post_counts().await {
                        Ok(info) => Msg::ReceiveCountPosts(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveCountPosts(FetchState::Failed(FetchError::from(
                            JsValue::FALSE,
                        ))),
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::ReceiveCountPosts(info) => {
                self.fetch = info;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let FetchState::NotFetching = self.fetch.clone() {
            self.link.send_message(Msg::GetCountPosts);
        }
        if let FetchState::Success(resp) = self.fetch.clone() {
            if let Some(body) = resp.body {
                html! {
                    <>
                    <h2>{"Recent posts"}</h2>
                    {
                        for (1..=min(MAX_NUMBER_OF_POSTS_PREVIEW, body.count)).map(|i| {
                            html! {
                                <div class="top-padding">
                                <div class="container">
                                        <div class="block">
                                            <PostPreview id=min(MAX_NUMBER_OF_POSTS_PREVIEW, body.count)-i+1/>
                                        </div>
                                </div>
                                </div>
                            }
                        })
                    }
                    <div class="container">
                    <div class="top-padding">
                        <MatButton raised=true label="More..."/>
                    </div>
                    </div>
                    </>
                }
            } else {
                html! {
                    <h2>{"Some error occurred."}</h2>
                }
            }
        } else {
            html! {
                <h2>{"Some error occurred."}</h2>
            }
        }
    }
}
