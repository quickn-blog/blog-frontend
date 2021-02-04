use yew::{format::{Json, Nothing}, prelude::*};
use yew_material::{MatSnackbar, MatFormfield, MatTextField, MatButton, WeakComponentLink};
use yew_material::text_inputs::*;
use yew_router::prelude::*;
use yew_router::agent::RouteRequest;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use wasm_bindgen::prelude::*;
use crate::services::cookie::CookieService;
use crate::api::*;
use crate::services::router;

pub struct RegisterPage {
    link: ComponentLink<Self>,
    username: String,
    password: String,
    password_verify: String,
    email: String,
    nickname: String,
    error_link: WeakComponentLink<MatSnackbar>,
    error_msg: AccountError,
    fetch_register: FetchState<ResponseBlock<RegisterResponse>>,
}

pub enum Msg {
    UpdateUsername(InputData),
    UpdatePassword(InputData),
    UpdatePasswordVerify(InputData),
    UpdateEmail(InputData),
    UpdateNickname(InputData),
    GetRegister,
    ReceiveRegisterResponse(FetchState<ResponseBlock<RegisterResponse>>),
    ShowError,
    GoLogin,
}

#[derive(Properties, Clone)]
pub struct Props {
}

impl Component for RegisterPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Props, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            username: String::new(),
            password: String::new(),
            password_verify: String::new(),
            email: String::new(),
            nickname: String::new(),
            error_link: WeakComponentLink::default(),
            error_msg: AccountError::Nothing,
            fetch_register: FetchState::NotFetching,
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
            Msg::UpdatePasswordVerify(s) => {
                self.password_verify = s.value;
                false
            },
            Msg::UpdateEmail(s) => {
                self.email = s.value;
                false
            },
            Msg::UpdateNickname(s) => {
                self.nickname = s.value;
                false
            },
            Msg::GetRegister => {
                let form = RegisterForm {
                    username: self.username.clone(),
                    email: self.email.clone(),
                    nickname: self.nickname.clone(),
                    pass: self.password.clone(),
                };
                let future = async move {
                    match register(form).await {
                        Ok(info) => Msg::ReceiveRegisterResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveRegisterResponse(FetchState::Failed(FetchError::from(JsValue::FALSE))),
                    }
                };
                send_future(self.link.clone(), future);
                false
            },
            Msg::ReceiveRegisterResponse(data) => {
                if let FetchState::Success(resp) = data.clone() {
                    if let Some(body) = resp.body {
                        match body.result {
                            AccountError::Nothing => {},
                            _ => { self.error_msg = body.result; },
                        }
                    } else {
                        self.error_msg = AccountError::NetworkError;
                    }
                }
                self.fetch_register = data;
                true
            },
            Msg::ShowError => {
                self.error_link.show();
                false
            },
            Msg::GoLogin => {
                let mut router = RouteAgentDispatcher::<()>::new();
                let route = Route::from(router::MainRoute::Login);
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
        let link = self.link.clone();
        let validity_transform = MatTextField::validity_transform(move |s, _| {
            let mut state = ValidityState::new();
            let comp = link.get_component().unwrap();
            let pass = comp.password.clone();
            if s == pass {
                state.set_valid(true).set_bad_input(false);
            } else {
                state.set_valid(false).set_bad_input(true);
            }
            state
        });
        if let FetchState::Success(resp) = self.fetch_register.clone() {
            if let Some(body) = resp.body {
                match body.result {
                    AccountError::Nothing => {
                        self.link.send_message(Msg::GoLogin);
                    },
                    _ => { self.link.send_message(Msg::ShowError) },
                }
            } else {
                self.link.send_message(Msg::ShowError)
            }
        }
        html!{
            <div class="container">
                <MatSnackbar label_text=&format!("Failed to register: {}", self.error_msg) snackbar_link=self.error_link.clone()/>
                <div class="form">
                    <div class="field">
                        <h3>{"Register to ANEP Research"}</h3>
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
                        <MatFormfield>
                            <MatTextField outlined=true icon="check_circle" label="Password Verify" auto_validate=true validation_message="Sorry, try again" validity_transform=validity_transform.clone() value=self.password_verify.clone() field_type=TextFieldType::Password oninput=self.link.callback(|s| Msg::UpdatePasswordVerify(s))/>
                        </MatFormfield>
                    </div>
                    <div class="field">
                        <MatFormfield>
                            <MatTextField outlined=true icon="email" label="Email" auto_validate=true value=self.email.clone() field_type=TextFieldType::Email oninput=self.link.callback(|s| Msg::UpdateEmail(s))/>
                        </MatFormfield>
                    </div>
                    <div class="field">
                        <MatFormfield>
                            <MatTextField outlined=true icon="badge" label="Nickname" value=self.nickname.clone() oninput=self.link.callback(|s| Msg::UpdateNickname(s))/>
                        </MatFormfield>
                    </div>
                    <div class="field">
                        <div onclick=self.link.callback(|_| Msg::GetRegister)><MatButton label="Sumbit"/></div>
                    </div>
                </div>
            </div>
        }
    }
}