use yew::{format::{Json, Nothing}, prelude::*};
use yew_material::{MatSnackbar, MatFormfield, MatTextField, MatButton, WeakComponentLink};
use yew_material::text_inputs::*;
use yew_router::prelude::*;
use yew_router::agent::RouteRequest;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use crate::services::cookie::CookieService;
use crate::api::*;
use crate::services::router;

pub struct LoginPage {
    props: Props,
    link: ComponentLink<Self>,
    username: String,
    password: String,
    cookie: CookieService,
    fetch: FetchState<ResponseBlock<LoginResponse>>,
    error_link: WeakComponentLink<MatSnackbar>,
}

pub enum Msg {
    UpdateUsername(InputData),
    UpdatePassword(InputData),
    GetLogin,
    ReceiveLoginResponse(FetchState<ResponseBlock<LoginResponse>>),
    ShowError(AccountError),
    GoMain,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub info: Option<InfoResponse>,
}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            username: String::new(),
            password: String::new(),
            cookie: CookieService::new(),
            fetch: FetchState::NotFetching,
            error_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateUsername(s) => {
                self.username = s.value;
                false
            },
            Msg::UpdatePassword(s) => {
                self.password = s.value;
                false
            },
            Msg::GetLogin => {
                let form = LoginForm {
                    username: self.username.clone(),
                    pass: self.password.clone(),
                };
                let future = async move {
                    let client = reqwest::Client::new();
                    let res = client
                        .post("http://localhost/api/account_service/login")
                        .json(&form)
                        .send()
                        .await
                        .unwrap();
                    let text = res.text().await.unwrap();
                    let info: ResponseBlock<LoginResponse> = serde_json::from_str(&text).unwrap();
                    Msg::ReceiveLoginResponse(FetchState::Success(info))
                };
                send_future(self.link.clone(), future);
                false
            },
            Msg::ReceiveLoginResponse(data) => {
                self.fetch = data;
                true
            },
            Msg::ShowError(e) => {
                self.error_link.show();
                false
            },
            Msg::GoMain => {
                let mut router = RouteAgentDispatcher::<()>::new();
                let route = Route::from(router::MainRoute::Main);
                router.send(RouteRequest::ChangeRoute(route));
                false
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
        if let Some(_) = self.props.info {
            self.link.send_message(Msg::GoMain);
        }
        if let FetchState::Success(resp) = self.fetch.clone() {
            if let Some(body) = resp.body {
                match body.result {
                    AccountError::Nothing => {
                        self.cookie.set("token", &body.token.unwrap());
                        self.link.send_message(Msg::GoMain);
                    },
                    _ => { self.link.send_message(Msg::ShowError(body.result)) },
                }
            } else {
                self.link.send_message(Msg::ShowError(AccountError::NetworkError))
            }
        }
        html!{
            <div class="container">
                <MatSnackbar label_text="Fail to login" snackbar_link=self.error_link.clone()/>
                <div class="form">
                    <div class="field">
                        <h3>{"Login to ANEP Research"}</h3>
                    </div>
                    <div class="field">
                        <MatFormfield>
                            <MatTextField label="Username" value=self.username.clone() oninput=self.link.callback(|s| Msg::UpdateUsername(s))/>
                        </MatFormfield>
                    </div>
                    <div class="field">
                        <MatFormfield>
                            <MatTextField label="Password" value=self.password.clone() field_type=TextFieldType::Password oninput=self.link.callback(|s| Msg::UpdatePassword(s))/>
                        </MatFormfield>
                    </div>
                    <div class="field">
                        <div onclick=self.link.callback(|_| Msg::GetLogin)><MatButton label="Sumbit"/></div>
                    </div>
                </div>
            </div>
        }
    }
}