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

pub struct LoginPage {
    link: ComponentLink<Self>,
    root_link: ComponentLink<crate::Root>,
    username: String,
    password: String,
    cookie: CookieService,
    error_link: WeakComponentLink<MatSnackbar>,
    error_msg: AccountError,
    fetch_login: FetchState<ResponseBlock<LoginResponse>>,
    fetch_info: FetchState<ResponseBlock<InfoResponse>>,
}

pub enum Msg {
    UpdateUsername(InputData),
    UpdatePassword(InputData),
    GetLogin,
    GetInfo,
    ReceiveLoginResponse(FetchState<ResponseBlock<LoginResponse>>),
    ReceiveInfoResponse(FetchState<ResponseBlock<InfoResponse>>),
    ShowError,
    GoMain,
    GoRegister,
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Props, link: ComponentLink<Self>) -> Self {
        let mut any = link.get_parent().unwrap();
        while let Some(l) = any.get_parent() {
            any = l;
        }
        let root_link: ComponentLink<crate::Root> = any.clone().downcast();
        Self {
            link,
            root_link,
            username: String::new(),
            password: String::new(),
            cookie: CookieService::new(),
            error_link: WeakComponentLink::default(),
            error_msg: AccountError::Nothing,
            fetch_login: FetchState::NotFetching,
            fetch_info: FetchState::NotFetching,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateUsername(s) => {
                self.username = s.value;
                false
            }
            Msg::UpdatePassword(s) => {
                self.password = s.value;
                false
            }
            Msg::GetLogin => {
                let form = LoginForm {
                    username: self.username.clone(),
                    pass: self.password.clone(),
                };
                let future = async move {
                    match login(form).await {
                        Ok(info) => Msg::ReceiveLoginResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveLoginResponse(FetchState::Failed(FetchError::from(
                            JsValue::FALSE,
                        ))),
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::GetInfo => {
                let future = async move {
                    match get_info().await {
                        Ok(info) => Msg::ReceiveInfoResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveInfoResponse(FetchState::Failed(FetchError::from(
                            JsValue::FALSE,
                        ))), // TODO
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::ReceiveInfoResponse(data) => {
                self.fetch_info = data;
                true
            }
            Msg::ReceiveLoginResponse(data) => {
                if let FetchState::Success(resp) = data.clone() {
                    if let Some(body) = resp.body {
                        match body.result {
                            AccountError::Nothing => {}
                            _ => {
                                self.error_msg = body.result;
                            }
                        }
                    } else {
                        self.error_msg = AccountError::NetworkError;
                    }
                }
                self.fetch_login = data;
                true
            }
            Msg::ShowError => {
                self.error_link.show();
                false
            }
            Msg::GoMain => {
                let mut router = RouteAgentDispatcher::<()>::new();
                let route = Route::from(router::MainRoute::Main);
                router.send(RouteRequest::ChangeRoute(route));
                false
            }
            Msg::GoRegister => {
                let mut router = RouteAgentDispatcher::<()>::new();
                let route = Route::from(router::MainRoute::Register);
                router.send(RouteRequest::ChangeRoute(route));
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let FetchState::Success(r) = self.fetch_info.clone() {
            if r.status {
                self.link.send_message(Msg::GoMain);
            }
        } else if let FetchState::NotFetching = self.fetch_info.clone() {
            self.link.send_message(Msg::GetInfo);
        }
        if let FetchState::Success(resp) = self.fetch_login.clone() {
            if let Some(body) = resp.body {
                match body.result {
                    AccountError::Nothing => {
                        self.cookie.set("token", &body.token.unwrap());
                        self.root_link.send_message(crate::Msg::GetInfo);
                        self.link.send_message(Msg::GoMain);
                    }
                    _ => self.link.send_message(Msg::ShowError),
                }
            } else {
                self.link.send_message(Msg::ShowError)
            }
        }
        html! {
            <div class="container">
                <MatSnackbar label_text=&format!("Failed to login: {}", self.error_msg) snackbar_link=self.error_link.clone()/>
                <div class="form">
                    <div class="field">
                        <h3>{"Login to ANEP Research"}</h3>
                    </div>
                    <div class="field">
                        <MatFormfield>
                            <MatTextField outlined=true icon="person" label="Username" value=self.username.clone() oninput=self.link.callback(|s| Msg::UpdateUsername(s))/>
                        </MatFormfield>
                    </div>
                    <div class="field">
                        <MatFormfield>
                            <MatTextField outlined=true icon="lock" label="Password" value=self.password.clone() field_type=TextFieldType::Password oninput=self.link.callback(|s| Msg::UpdatePassword(s))/>
                        </MatFormfield>
                    </div>
                    <div class="field">
                        {"Haven't an account? "}<router::MainRouterAnchor route=router::MainRoute::Register>{"click here"}</router::MainRouterAnchor>
                    </div>
                    <div class="field">
                        <div onclick=self.link.callback(|_| Msg::GetLogin)><MatButton label="Sumbit" raised=true/></div>
                    </div>
                </div>
            </div>
        }
    }
}
