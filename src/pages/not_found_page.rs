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

pub struct NotFoundPage {
    link: ComponentLink<Self>,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for NotFoundPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Props, link: ComponentLink<Self>) -> Self {
        Self { link }
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
        html! {
            <>
                <div class="container">
                    <h2>{"404 NOT FOUND"}</h2>
                </div>
                <div class="container">
                    <router::MainRouterAnchor route=router::MainRoute::Main><MatButton raised=true label="Main"/></router::MainRouterAnchor>
                </div>
            </>
        }
    }
}
