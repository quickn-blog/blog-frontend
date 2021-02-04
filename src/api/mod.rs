use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::fmt::{Error, Formatter};
use std::future::Future;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::services::cookie::CookieService;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    pub err: JsValue,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}

impl std::error::Error for FetchError {}


impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

#[derive(Clone, PartialEq)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum AccountError {
    Nothing,
    PassNotMatched,
    UserNotExists,
    DatabaseError,
    UsernameAlreadyExists,
    EmailAlreadyExists,
    NetworkError,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub pass: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub result: AccountError,
    pub token: Option<String>, // JWT token
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountError::UsernameAlreadyExists => write!(f, "Username already exists."),
            AccountError::EmailAlreadyExists => write!(f, "E-mail already exists."),
            /*AccountError::NotValidEmail => {
                write!(f, "Your E-mail is not valid. Re-check about it.")
            }
            AccountError::PasswordTooWeak => write!(
                f,
                "Your password is too short. Please set at least length 8."
            ),*/
            AccountError::NetworkError => write!(f, "Some network error occurs."),
            AccountError::DatabaseError => write!(f, "Some database error occurs."),
            AccountError::UserNotExists => write!(f, "Your user not exists."),
            AccountError::PassNotMatched => write!(f, "Your password is not matched."),
            _ => write!(f, "Nothing."),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountLevel {
    Default = 0,
    Admin = 1,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseBlock<T> {
    pub status: bool,
    pub body: Option<T>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InfoResponse {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub level: AccountLevel,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub pass: String,
    pub email: String,
    pub nickname: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub result: AccountError,
}

pub fn send_future<COMP: Component, F>(link: ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    spawn_local(async move {
        link.send_message(future.await);
    });
}

pub async fn login(form: LoginForm) -> Result<ResponseBlock<LoginResponse>, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost/api/account_service/login")
        .json(&form)
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<LoginResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn register(form: RegisterForm) -> Result<ResponseBlock<RegisterResponse>, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost/api/account_service/register")
        .json(&form)
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<RegisterResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn get_info() -> Result<ResponseBlock<InfoResponse>, anyhow::Error> {
    let cookie = CookieService::new();
    let client = reqwest::Client::new();
    let res = client
        .get(&format!("http://localhost/api/account_service/info?token={}", cookie.get("token").unwrap_or(String::new())))
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<InfoResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}
