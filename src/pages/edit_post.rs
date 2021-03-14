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

pub struct EditPost {
    props: Props,
    link: ComponentLink<Self>,
    title: String,
    body: String,
    tags: String,
    error_link: WeakComponentLink<MatSnackbar>,
    error_msg: BlogError,
    fetch_edit_post: FetchState<ResponseBlock<EditPostResponse>>,
    fetch_info: FetchState<ResponseBlock<InfoResponse>>,
    fetch_view_post: FetchState<ResponseBlock<ViewPostResponse>>,
}

pub enum Msg {
    UpdateTitle(InputData),
    UpdateBody(InputData),
    UpdateTags(InputData),
    GetInfo,
    GetViewPost,
    GetEditPost,
    ReceiveEditPostResponse(FetchState<ResponseBlock<EditPostResponse>>),
    ReceiveInfoResponse(FetchState<ResponseBlock<InfoResponse>>),
    ReceiveViewPostResponse(FetchState<ResponseBlock<ViewPostResponse>>),
    ShowError,
    GoLogin,
    GoPost,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i64,
}

impl Component for EditPost {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            title: String::new(),
            body: String::new(),
            tags: String::new(),
            error_link: WeakComponentLink::default(),
            error_msg: BlogError::Nothing,
            fetch_edit_post: FetchState::NotFetching,
            fetch_info: FetchState::NotFetching,
            fetch_view_post: FetchState::NotFetching,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateTitle(s) => {
                self.title = s.value;
                false
            }
            Msg::UpdateBody(s) => {
                self.body = s.value;
                false
            }
            Msg::UpdateTags(s) => {
                self.tags = s.value;
                false
            }
            Msg::GetEditPost => {
                if self.title.len() == 0 {
                    self.error_msg = BlogError::TooShortTitle;
                    true
                } else if self.body.len() == 0 {
                    self.error_msg = BlogError::TooShortBody;
                    true
                } else if !self.tags.is_ascii() {
                    self.error_msg = BlogError::InvalidTags;
                    true
                } else {
                    let t = self.tags.replace(" ", "");
                    let tags_vec: Vec<String> = t.split(",").map(|s| s.to_string()).collect();
                    let form = EditPostForm {
                        pk: self.props.id,
                        title: self.title.clone(),
                        body: self.body.clone(),
                        tag: tags_vec,
                    };
                    let future = async move {
                        match edit_post(form).await {
                            Ok(info) => Msg::ReceiveEditPostResponse(FetchState::Success(info)),
                            Err(_) => Msg::ReceiveEditPostResponse(FetchState::Failed(
                                FetchError::from(JsValue::FALSE),
                            )),
                        }
                    };
                    send_future(self.link.clone(), future);
                    false
                }
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
            Msg::GetViewPost => {
                let id = self.props.id;
                self.fetch_view_post = FetchState::Fetching;
                let future = async move {
                    match view_post(id).await {
                        Ok(info) => Msg::ReceiveViewPostResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveViewPostResponse(FetchState::Failed(FetchError::from(
                            JsValue::FALSE,
                        ))),
                    }
                };
                send_future(self.link.clone(), future);
                false
            },
            Msg::ReceiveInfoResponse(data) => {
                self.fetch_info = data;
                true
            }
            Msg::ReceiveEditPostResponse(data) => {
                if let FetchState::Success(resp) = data.clone() {
                    if let Some(body) = resp.body {
                        match body.error {
                            BlogError::Nothing => {}
                            _ => {
                                self.error_msg = body.error;
                            }
                        }
                    } else {
                        self.error_msg = BlogError::NetworkError;
                    }
                }
                self.fetch_edit_post = data;
                true
            }
            Msg::ReceiveViewPostResponse(data) => {
                self.fetch_view_post = data.clone();
                if let FetchState::Success(post) = data {
                    if let Some(body) = post.body {
                        if let Some(p) = body.post {
                            self.title = p.title;
                            self.body = p.body;
                            self.tags = p.tags.join(",");
                        }
                    }
                }
                true
            }
            Msg::ShowError => {
                self.error_link.show();
                false
            }
            Msg::GoPost => {
                let mut router = RouteAgentDispatcher::<()>::new();
                let route = Route::from(router::MainRoute::ViewPost(self.props.id));
                router.send(RouteRequest::ChangeRoute(route));
                false
            }
            Msg::GoLogin => {
                let mut router = RouteAgentDispatcher::<()>::new();
                let route = Route::from(router::MainRoute::Login);
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
        let post = if let FetchState::NotFetching = self.fetch_view_post.clone() {
            self.link.send_message(Msg::GetViewPost);
            None
        } else if let FetchState::Success(post) = self.fetch_view_post.clone() {
            if let Some(body) = post.body {
                body.post
            } else {
                None
            }
        } else {
            None
        };
        if let FetchState::Success(r) = self.fetch_info.clone() {
            if !r.status {
                self.link.send_message(Msg::GoLogin);
            }
            if let Some(body) = post.clone() {
                if let Some(user) = r.body {
                    if body.author != user.pk as i32 {
                        self.link.send_message(Msg::GoLogin);
                    }
                } else {
                    self.link.send_message(Msg::GoLogin);
                }
            }
        } else if let FetchState::NotFetching = self.fetch_info.clone() {
            self.link.send_message(Msg::GetInfo);
        }
        if let FetchState::Success(resp) = self.fetch_edit_post.clone() {
            if let Some(body) = resp.body {
                match body.error {
                    BlogError::Nothing => {
                        self.link.send_message(Msg::GoPost);
                    }
                    _ => {}
                }
            }
        }
        if self.error_msg != BlogError::Nothing {
            self.link.send_message(Msg::ShowError);
        }
        html! {
            <div class="container">
                <MatSnackbar label_text=&format!("Failed to modify post: {}", self.error_msg) snackbar_link=self.error_link.clone()/>
                <div class="form-fill">
                    <div class="field">
                        <h3>{"Edit a post"}</h3>
                    </div>
                    <div class="field">
                        <MatTextField required=true fullwidth=true outlined=true label="Title" value=self.title.clone() oninput=self.link.callback(|s| Msg::UpdateTitle(s))/>
                    </div>
                    <div class="field">
                        <MatTextArea required=true fullwidth=true outlined=true label="Body" value=self.body.clone() oninput=self.link.callback(|s| Msg::UpdateBody(s))/>
                    </div>
                    <div class="field">
                        <MatTextField fullwidth=true outlined=true label="Tags" value=self.tags.clone() oninput=self.link.callback(|s| Msg::UpdateTags(s))/>
                    </div>
                    <div class="field">
                        <div onclick=self.link.callback(|_| Msg::GetEditPost)><MatButton label="Sumbit" raised=true/></div>
                    </div>
                </div>
            </div>
        }
    }
}
