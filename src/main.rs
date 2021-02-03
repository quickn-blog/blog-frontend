#[macro_use]
extern crate yew;
#[macro_use]
extern crate yew_router;
extern crate yew_services;
extern crate yew_material;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate reqwest;
extern crate web_sys;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate anyhow;
#[macro_use]
extern crate log;
extern crate wasm_logger;

mod services;
mod pages;
mod api;

use yew_services::fetch::{Request, Response, FetchService, FetchTask, FetchOptions, Mode};
use yew::{format::{Json, Nothing}, prelude::*};
use yew_material::{MatList, MatButton, MatTopAppBarFixed, MatIconButton};
use yew_material::top_app_bar_fixed::*;
use yew_material::list::*;
use yew_material::drawer::*;
use serde::{Serialize, Deserialize};
use services::{router, cookie};
use api::*;

struct Root {
    link: ComponentLink<Self>,
    is_opened: bool,
    cookie: cookie::CookieService,
    fetch_task: FetchState<ResponseBlock<InfoResponse>>,
    info: Option<InfoResponse>,
}

enum Msg {
    GetNavIconClick,
    GetOpen,
    GetClose,
    GetInfo,
    ReceiveInfo(FetchState<ResponseBlock<InfoResponse>>),
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            is_opened: false,
            cookie: cookie::CookieService::new(),
            fetch_task: FetchState::NotFetching,
            info: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetOpen => {
                self.is_opened = true;
                true
            },
            Msg::GetClose => {
                self.is_opened = false;
                true
            },
            Msg::GetNavIconClick => {
                self.is_opened ^= true;
                true
            }
            Msg::GetInfo => {
                let cookie = self.cookie;
                let future = async move {
                    let client = reqwest::Client::new();
                    let res = client
                        .get(&format!("http://localhost/api/account_service/info?token={}", cookie.get("token").unwrap_or(String::new())))
                        .send()
                        .await
                        .unwrap();
                    let text = res.text().await.unwrap();
                    let info: ResponseBlock<InfoResponse> = serde_json::from_str(&text).unwrap();
                    Msg::ReceiveInfo(FetchState::Success(info))
                };
                send_future(self.link.clone(), future);
                false
            },
            Msg::ReceiveInfo(data) => {
                if let FetchState::Success(r) = data.clone() {
                    self.info = r.body;
                }
                self.fetch_task = data;
                true
            },
            _ => {
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let (is_logined, nickname) = if let FetchState::Success(data) = self.fetch_task.clone() {
            if let Some(body) = data.body {
                (data.status, body.nickname)
            } else {
                (false, String::new())
            }
        } else {
            match self.fetch_task {
                FetchState::NotFetching =>
                self.link.send_message(Msg::GetInfo),
                _ => {}
            }
            (false, String::new())
        };
        let info = self.info.clone();
        html! {
            <MatDrawer open=self.is_opened drawer_type="modal" onopened=self.link.callback(|_| Msg::GetOpen) onclosed=self.link.callback(|_| Msg::GetClose)>
                <div class="drawer-content">
                    <div class="profile">
                        <svg xmlns="http://www.w3.org/2000/svg" height="80" viewBox="0 0 24 24" width="80"><path d="M0 0h24v24H0z" fill="none"/><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 3c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm0 14.2c-2.5 0-4.71-1.28-6-3.22.03-1.99 4-3.08 6-3.08 1.99 0 5.97 1.09 6 3.08-1.29 1.94-3.5 3.22-6 3.22z"/></svg>
                        {
                            if is_logined {
                                html! { <h3>{&nickname}</h3> }
                            } else {
                                html! { <h3>{"Please login"}</h3> }
                            }
                        }
                    </div>
                    <div class="navigate-menu">
                        <MatList>
                            <router::MainRouterAnchor route=router::MainRoute::Main><MatListItem>{"Home"}</MatListItem></router::MainRouterAnchor>
                        </MatList>
                    </div>
                </div>
                <MatDrawerAppContent>
                    <MatTopAppBarFixed onnavigationiconclick=self.link.callback(|_| Msg::GetNavIconClick)>
                        <MatTopAppBarNavigationIcon>
                            <MatIconButton icon="menu"></MatIconButton>
                        </MatTopAppBarNavigationIcon>
                        <MatTopAppBarTitle>
                            {"ANEP Research"}
                        </MatTopAppBarTitle>
                        <MatTopAppBarActionItems>
                            {
                                if is_logined {
                                    html! { <MatIconButton icon="user"/> }
                                } else {
                                    html! { <router::MainRouterAnchor route=router::MainRoute::Login><MatIconButton icon="login"/></router::MainRouterAnchor> }
                                }
                            }
                        </MatTopAppBarActionItems>
                    </MatTopAppBarFixed>
                    <main id="router-outlet">
                        <router::MainRouter render=router::MainRouter::render(|switch: router::MainRoute| {
                            match switch {
                                router::MainRoute::Main => html!{ <pages::main::Main/> },
                                router::MainRoute::Login => html!{ <pages::login::LoginPage info=None/> },
                                _ => html!{{"test"}}
                            }
                        })/>
                    </main>
                </MatDrawerAppContent>
            </MatDrawer>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Root>();
}
