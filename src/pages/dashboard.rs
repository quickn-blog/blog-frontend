use crate::api::*;
use crate::services::cookie::CookieService;
use crate::services::router;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use wasm_bindgen::prelude::*;
use yew::{
    format::{Json, Nothing},
    prelude::*,
};
use yew_material::text_inputs::*;
use yew_material::{MatButton, MatFormfield, MatSnackbar, MatTextField, WeakComponentLink};
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

pub struct DashboardPage {
    link: ComponentLink<Self>,
    root_link: ComponentLink<crate::Root>,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for DashboardPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Props, link: ComponentLink<Self>) -> Self {
        let mut any = link.get_parent().unwrap();
        while let Some(l) = any.get_parent() {
            any = l;
        }
        let root_link: ComponentLink<crate::Root> = any.clone().downcast();
        Self { link, root_link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let user = if let FetchState::Success(r) =
            self.root_link.get_component().unwrap().fetch_task.clone()
        {
            r.body
        } else {
            None
        };
        html! {
            <div class="container">
                <div class="block">
                    <h3>{"Dashboard"}</h3>
                    {
                        if let Some(body) = user {
                            match body.level {
                                AccountLevel::Admin => { html! { <p>{"User level: ADMIN"}</p> } },
                                _ => { html! { <p>{"User level: USER"}</p> } },
                            }
                        } else {
                            html!{}
                        }
                    }
                </div>
            </div>
        }
    }
}
