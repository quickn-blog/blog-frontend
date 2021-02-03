use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::fmt::{Error, Formatter};
use std::future::Future;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

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

pub fn send_future<COMP: Component, F>(link: ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    spawn_local(async move {
        link.send_message(future.await);
    });
}