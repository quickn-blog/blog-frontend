use crate::api::*;
use crate::constants::*;
use crate::services::cookie::CookieService;
use crate::services::router;
use chrono::prelude::*;
use chrono_tz::Asia::Seoul;
use std::cmp::min;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_material::list::*;
use yew_material::{MatButton, MatList};

pub struct PostPreview {
    props: Props,
    link: ComponentLink<Self>,
    fetch: FetchState<ResponseBlock<ViewPostResponse>>,
    fetch_info: FetchState<ResponseBlock<InfoResponse>>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i64,
}

pub enum Msg {
    GetPreview,
    GetInfo(i32),
    ReceiveViewResponse(FetchState<ResponseBlock<ViewPostResponse>>),
    ReceiveInfoResponse(FetchState<ResponseBlock<InfoResponse>>)
}

impl Component for PostPreview {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            fetch: FetchState::NotFetching,
            fetch_info: FetchState::NotFetching,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::GetPreview => {
                let id = self.props.id;
                self.fetch = FetchState::Fetching;
                let future = async move {
                    match view_post(id).await {
                        Ok(info) => Msg::ReceiveViewResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveViewResponse(FetchState::Failed(FetchError::from(
                            JsValue::FALSE,
                        ))),
                    }
                };
                send_future(self.link.clone(), future);
                false
            },
            Msg::ReceiveViewResponse(info) => {
                self.fetch = info;
                true
            },
            Msg::GetInfo(user) => {
                let future = async move {
                    match get_info_by_pk(user).await {
                        Ok(info) => Msg::ReceiveInfoResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveInfoResponse(FetchState::Failed(FetchError::from(
                            JsValue::FALSE,
                        ))), // TODO
                    }
                };
                send_future(self.link.clone(), future);
                false
            },
            Msg::ReceiveInfoResponse(info) => {
                self.fetch_info = info;
                true
            }
        }
    }

    fn change(&mut self, _props: Props) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let FetchState::NotFetching = self.fetch.clone() {
            self.link.send_message(Msg::GetPreview);
        }
        let info = if let FetchState::Success(resp) = self.fetch.clone() {
            resp.body
        } else {
            None
        };
        let user =  if let Some(resp) = info.clone() {
            if let Some(post) = resp.post {
            if let FetchState::NotFetching = self.fetch_info.clone() {
                self.link.send_message(Msg::GetInfo(post.author));
                None
            } else if let FetchState::Success(res) = self.fetch_info.clone() {
                res.body
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
        if let Some(resp) = info {
            if let Some(post) = resp.post {
                let mut tags = String::new();
                for i in 0..post.tags.len() {
                    tags.push_str(&format!("#{}", post.tags[i]));
                    if i != post.tags.len() - 1 {
                        tags.push_str(", ");
                    }
                }
                if tags == String::from("#") {
                    tags = String::from("NO TAGS");
                }
                html! {
                <>
                    <h2>{post.title}</h2>
                    <MatList><li divider=true role="separator"></li></MatList>
                    <MatListItem graphic=GraphicType::Avatar twoline=true noninteractive=true>
                        <span>{if let Some(user_info) = user { user_info.nickname.clone() } else { String::from("ERROR") }}</span>
                        <span slot="secondary">{&format!("{}", Seoul.from_utc_datetime(&post.created_at))}</span>
                        <mwc-icon slot="graphic" class="inverted">{"tag_faces"}</mwc-icon>
                    </MatListItem>
                    <MatList><li divider=true role="separator"></li></MatList>
                    <MatListItem noninteractive=true><b>{tags}</b></MatListItem>
                    <router::MainRouterAnchor route=router::MainRoute::ViewPost(self.props.id)><MatButton label="Read more" raised=true/></router::MainRouterAnchor>
                </>
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        }
    }
}
