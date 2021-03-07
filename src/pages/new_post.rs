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

pub struct NewPostPage {
    link: ComponentLink<Self>,
    root_link: ComponentLink<crate::Root>,
    title: String,
    body: String,
    error_link: WeakComponentLink<MatSnackbar>,
    error_msg: BlogError,
    fetch_new_post: FetchState<ResponseBlock<NewPostResponse>>,
    fetch_info: FetchState<ResponseBlock<InfoResponse>>,
}

pub enum Msg {
    UpdateTitle(InputData),
    UpdateBody(InputData),
    GetInfo,
    GetNewPost,
    ReceiveNewPostResponse(FetchState<ResponseBlock<NewPostResponse>>),
    ReceiveInfoResponse(FetchState<ResponseBlock<InfoResponse>>),
    ShowError,
    GoMain,
    GoLogin,
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for NewPostPage {
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
            title: String::new(),
            body: String::new(),
            error_link: WeakComponentLink::default(),
            error_msg: BlogError::Nothing,
            fetch_new_post: FetchState::NotFetching,
            fetch_info: FetchState::NotFetching,
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
            Msg::GetNewPost => {
                let form = NewPostForm {
                    title: self.title.clone(),
                    body: self.body.clone(),
                    tag: vec![],
                };
                let future = async move {
                    match new_post(form).await {
                        Ok(info) => Msg::ReceiveNewPostResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveNewPostResponse(FetchState::Failed(
                            FetchError::from(JsValue::FALSE),
                        )),
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
            Msg::ReceiveNewPostResponse(data) => {
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
                self.fetch_new_post = data;
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
        if let FetchState::Success(r) = self.fetch_info.clone() {
            if !r.status {
                self.link.send_message(Msg::GoLogin);
            }
        } else if let FetchState::NotFetching = self.fetch_info.clone() {
            self.link.send_message(Msg::GetInfo);
        }
        if let FetchState::Success(resp) = self.fetch_new_post.clone() {
            if let Some(body) = resp.body {
                match body.error {
                    BlogError::Nothing => {
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
                <MatSnackbar label_text=&format!("Failed to create post: {}", self.error_msg) snackbar_link=self.error_link.clone()/>
                <div class="form-fill">
                    <div class="field">
                        <h3>{"New post to blog"}</h3>
                    </div>
                    <div class="field">
                      //  <MatFormfield>
                            <MatTextField fullwidth=true outlined=true label="Title" value=self.title.clone() oninput=self.link.callback(|s| Msg::UpdateTitle(s))/>
                        //</MatFormfield>
                    </div>

                    <div class="field">
                        //<MatFormfield>
                            <MatTextArea fullwidth=true outlined=true label="Body" value=self.body.clone() oninput=self.link.callback(|s| Msg::UpdateBody(s))/>
                        //</MatFormfield>
                    </div>
                    <div class="field">
                        <div onclick=self.link.callback(|_| Msg::GetNewPost)><MatButton label="Sumbit" raised=true/></div>
                    </div>
                </div>
            </div>
        }
    }
}
