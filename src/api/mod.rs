use crate::constants::*;
use crate::services::cookie::CookieService;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Error, Formatter};
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
    PasswordVerifyFailed,
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
            AccountError::PasswordVerifyFailed => {
                write!(f, "Please re-check your password in the verify field.")
            }
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InfoResponse {
    pub pk: i64,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum BlogError {
    Nothing,
    AuthError,
    DatabaseError,
    NetworkError,
    PermissionError,
    TooShortTitle,
    TooShortBody,
    InvalidTags,
}

impl fmt::Display for BlogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlogError::AuthError => write!(f, "Some account verify error occurs."),
            BlogError::NetworkError => write!(f, "Some network error occurs."),
            BlogError::DatabaseError => write!(f, "Some database error occurs."),
            BlogError::PermissionError => write!(f, "You have not permission."),
            BlogError::TooShortBody => write!(f, "Too short body length."),
            BlogError::TooShortTitle => write!(f, "Too short title length."),
            BlogError::InvalidTags => write!(f, "Tags must be in ascii area."),
            _ => write!(f, "Nothing."),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AsRequest<T> {
    pub token: String,
    pub body: T,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct NewPostForm {
    pub title: String,
    pub body: String,
    pub tag: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ViewPostForm {
    pub id: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewPostResponse {
    pub error: BlogError,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CountPostsResponse {
    pub error: BlogError,
    pub count: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicPost {
    pub title: String,
    pub body: String,
    pub author: i32,
    pub tags: Vec<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ViewPostResponse {
    pub error: BlogError,
    pub post: Option<PublicPost>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DeletePostForm {
    pub id: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeletePostResponse {
    pub error: BlogError,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecentPostsResponse {
    pub error: BlogError,
    pub posts: Vec<i64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct EditPostForm {
    pub pk: i64,
    pub title: String,
    pub body: String,
    pub tag: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditPostResponse {
    pub error: BlogError,
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

pub async fn register(
    form: RegisterForm,
) -> Result<ResponseBlock<RegisterResponse>, anyhow::Error> {
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
        .get(&format!(
            "http://localhost/api/account_service/info?token={}",
            cookie.get("token").unwrap_or(String::new())
        ))
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<InfoResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn get_info_by_pk(pk: i32) -> Result<ResponseBlock<InfoResponse>, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(&format!(
            "http://localhost/api/account_service/get_user?pk={}",
            pk
        ))
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<InfoResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn get_post_counts() -> Result<ResponseBlock<CountPostsResponse>, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost/api/blog/count_posts")
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<CountPostsResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn recent_posts() -> Result<ResponseBlock<RecentPostsResponse>, anyhow::Error> {
    let client = reqwest::Client::new();
    let res = client
        .get(&format!(
            "http://localhost/api/blog/recent_posts?count={}",
            MAX_NUMBER_OF_POSTS_PREVIEW
        ))
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<RecentPostsResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn view_post(id: i64) -> Result<ResponseBlock<ViewPostResponse>, anyhow::Error> {
    let cookie = CookieService::new();
    let client = reqwest::Client::new();
    let form = AsRequest {
        token: cookie.get("token").unwrap_or(String::new()),
        body: ViewPostForm { id },
    };
    let res = client
        .post("http://localhost/api/blog/view_post")
        .json(&form)
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<ViewPostResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn new_post(block: NewPostForm) -> Result<ResponseBlock<NewPostResponse>, anyhow::Error> {
    let cookie = CookieService::new();
    let client = reqwest::Client::new();
    let form = AsRequest {
        token: cookie.get("token").unwrap_or(String::new()),
        body: block,
    };
    let res = client
        .post("http://localhost/api/blog/new_post")
        .json(&form)
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<NewPostResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn edit_post(
    block: EditPostForm,
) -> Result<ResponseBlock<EditPostResponse>, anyhow::Error> {
    let cookie = CookieService::new();
    let client = reqwest::Client::new();
    let form = AsRequest {
        token: cookie.get("token").unwrap_or(String::new()),
        body: block,
    };
    let res = client
        .post("http://localhost/api/blog/edit_post")
        .json(&form)
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<EditPostResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}

pub async fn delete_post(id: i64) -> Result<ResponseBlock<DeletePostResponse>, anyhow::Error> {
    let cookie = CookieService::new();
    let client = reqwest::Client::new();
    let form = AsRequest {
        token: cookie.get("token").unwrap_or(String::new()),
        body: DeletePostForm { id },
    };
    let res = client
        .post("http://localhost/api/blog/delete_post")
        .json(&form)
        .send()
        .await?;
    let text = res.text().await?;
    let info: ResponseBlock<DeletePostResponse> = serde_json::from_str(&text).unwrap();
    Ok(info)
}
