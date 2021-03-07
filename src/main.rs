#[macro_use]
extern crate yew;
#[macro_use]
extern crate yew_router;
extern crate reqwest;
extern crate serde;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;
extern crate yew_material;
extern crate yew_services;
#[macro_use]
extern crate serde_json;
extern crate anyhow;
#[macro_use]
extern crate log;
extern crate wasm_logger;

mod api;
mod constants;
mod pages;
mod services;

use api::*;
use serde::{Deserialize, Serialize};
use services::{cookie, router};
use wasm_bindgen::prelude::*;
use yew::{
    format::{Json, Nothing},
    prelude::*,
};
use yew_material::drawer::*;
use yew_material::list::*;
use yew_material::text_inputs::*;
use yew_material::top_app_bar_fixed::*;
use yew_material::{MatButton, MatIcon, MatIconButton, MatList, MatTopAppBarFixed};
use yew_services::fetch::{FetchOptions, FetchService, FetchTask, Mode, Request, Response};

struct Root {
    link: ComponentLink<Self>,
    is_opened: bool,
    cookie: cookie::CookieService,
    fetch_task: FetchState<ResponseBlock<InfoResponse>>,
    search_text: String,
}

enum Msg {
    GetNavIconClick,
    GetOpen,
    GetClose,
    GetInfo,
    GetLogout,
    ReceiveInfo(FetchState<ResponseBlock<InfoResponse>>),
    UpdateSearchText(InputData),
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
            search_text: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetOpen => {
                self.is_opened = true;
                true
            }
            Msg::GetClose => {
                self.is_opened = false;
                true
            }
            Msg::GetNavIconClick => {
                self.is_opened ^= true;
                true
            }
            Msg::GetInfo => {
                let future = async move {
                    match get_info().await {
                        Ok(info) => Msg::ReceiveInfo(FetchState::Success(info)),
                        Err(_) => {
                            Msg::ReceiveInfo(FetchState::Failed(FetchError::from(JsValue::FALSE)))
                        } // TODO
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::ReceiveInfo(data) => {
                self.fetch_task = data;
                true
            }
            Msg::GetLogout => {
                self.cookie.remove("token");
                self.fetch_task = FetchState::NotFetching;
                self.link.send_message(Msg::GetInfo);
                false
            }
            Msg::UpdateSearchText(data) => {
                self.search_text = data.value;
                false
            }
            _ => false,
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
                FetchState::NotFetching => self.link.send_message(Msg::GetInfo),
                _ => {}
            }
            (false, String::new())
        };
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
                            <router::MainRouterAnchor route=router::MainRoute::Main><MatListItem graphic=GraphicType::Icon>{"Home"}<mwc-icon slot="graphic">{"home"}</mwc-icon></MatListItem></router::MainRouterAnchor>
                            <li divider=true></li>
                            {
                                if is_logined {
                                    html! {
                                        <>
                                            <router::MainRouterAnchor route=router::MainRoute::Dashboard><MatListItem graphic=GraphicType::Icon>{"Dashboard"}<mwc-icon slot="graphic">{"dashboard"}</mwc-icon></MatListItem></router::MainRouterAnchor>
                                            <span onclick=self.link.callback(|_| Msg::GetLogout)><MatListItem graphic=GraphicType::Icon>{"Log-out"} <mwc-icon slot="graphic">{"logout"}</mwc-icon></MatListItem></span>
                                        </>
                                    }
                                } else {
                                    html! {
                                        <>
                                            <router::MainRouterAnchor route=router::MainRoute::Login><MatListItem graphic=GraphicType::Icon>{"Login"}<mwc-icon slot="graphic">{"login"}</mwc-icon></MatListItem></router::MainRouterAnchor>
                                            <router::MainRouterAnchor route=router::MainRoute::Register><MatListItem graphic=GraphicType::Icon>{"Register"}<mwc-icon slot="graphic">{"person_add"}</mwc-icon></MatListItem></router::MainRouterAnchor>
                                        </>
                                    }
                                }
                            }
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
                            <MatTextField value=self.search_text.clone() placeholder="Search.." icon="search" oninput=self.link.callback(|data| Msg::UpdateSearchText(data))/>
                            <div class="fix-link"><router::MainRouterAnchor route=router::MainRoute::NewPost><MatIconButton icon="add_circle"/></router::MainRouterAnchor></div>
                            {
                                if is_logined {
                                    html! { <div class="fix-link"><span onclick=self.link.callback(|_| Msg::GetLogout)><MatIconButton icon="logout"/></span></div> }
                                } else {
                                    html! { <div class="fix-link"><router::MainRouterAnchor route=router::MainRoute::Login><MatIconButton icon="login"/></router::MainRouterAnchor></div> }
                                }
                            }
                        </MatTopAppBarActionItems>
                    </MatTopAppBarFixed>
                    <main id="router-outlet">
                        <router::MainRouter render=router::MainRouter::render(Self::switch)/>
                    </main>
                </MatDrawerAppContent>
            </MatDrawer>
        }
    }
}

impl Root {
    fn switch(route: router::MainRoute) -> Html {
        match route {
            router::MainRoute::Main => html! { <pages::main::Main/> },
            router::MainRoute::Login => html! { <pages::login::LoginPage/> },
            router::MainRoute::Register => html! { <pages::register::RegisterPage/> },
            router::MainRoute::NewPost => html! { <pages::new_post::NewPostPage/> },
            router::MainRoute::ViewPost(id) => html! { <pages::view_post::ViewPost id=id/> },
            _ => html! {{"test"}},
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Root>();
}
