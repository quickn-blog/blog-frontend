use crate::api::*;
use crate::constants::*;
use crate::services::cookie::CookieService;
use crate::services::render;
use crate::services::router;
use chrono::prelude::*;
use chrono_tz::Asia::Seoul;
use std::cmp::min;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_material::dialog::*;
use yew_material::list::*;
use yew_material::{MatButton, MatList, MatSnackbar, WeakComponentLink};
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

pub struct ViewPost {
    props: Props,
    link: ComponentLink<Self>,
    error_link: WeakComponentLink<MatSnackbar>,
    error_msg: BlogError,
    fetch: FetchState<ResponseBlock<ViewPostResponse>>,
    fetch_info: FetchState<ResponseBlock<InfoResponse>>,
    fetch_info2: FetchState<ResponseBlock<InfoResponse>>,
    fetch_delete: FetchState<ResponseBlock<DeletePostResponse>>,
    delete_dialog: WeakComponentLink<MatDialog>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i64,
}

pub enum Msg {
    GoMain,
    GetPreview,
    GetInfo(i32),
    GetInfo2,
    GetDelete,
    ShowError,
    ReceiveViewResponse(FetchState<ResponseBlock<ViewPostResponse>>),
    ReceiveInfoResponse(FetchState<ResponseBlock<InfoResponse>>),
    ReceiveInfo2Response(FetchState<ResponseBlock<InfoResponse>>),
    ReceiveDeletePostResponse(FetchState<ResponseBlock<DeletePostResponse>>),
    ShowDeleteDialog,
    Dummy,
}

impl Component for ViewPost {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Props, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            error_link: WeakComponentLink::default(),
            error_msg: BlogError::Nothing,
            fetch: FetchState::NotFetching,
            fetch_info: FetchState::NotFetching,
            fetch_info2: FetchState::NotFetching,
            fetch_delete: FetchState::NotFetching,
            delete_dialog: WeakComponentLink::default(),
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
            }
            Msg::ReceiveViewResponse(info) => {
                self.fetch = info;
                true
            }
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
            }
            Msg::GetInfo2 => {
                let future = async move {
                    match get_info().await {
                        Ok(info) => Msg::ReceiveInfo2Response(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveInfo2Response(FetchState::Failed(FetchError::from(
                            JsValue::FALSE,
                        ))), // TODO
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::GetDelete => {
                let id = self.props.id;
                self.fetch = FetchState::Fetching;
                let future = async move {
                    match delete_post(id).await {
                        Ok(info) => Msg::ReceiveDeletePostResponse(FetchState::Success(info)),
                        Err(_) => Msg::ReceiveDeletePostResponse(FetchState::Failed(
                            FetchError::from(JsValue::FALSE),
                        )),
                    }
                };
                send_future(self.link.clone(), future);
                false
            }
            Msg::GoMain => {
                let mut router = RouteAgentDispatcher::<()>::new();
                let route = Route::from(router::MainRoute::Main);
                router.send(RouteRequest::ChangeRoute(route));
                false
            }
            Msg::ReceiveInfoResponse(info) => {
                self.fetch_info = info;
                true
            }
            Msg::ReceiveInfo2Response(info2) => {
                self.fetch_info2 = info2;
                true
            }
            Msg::ReceiveDeletePostResponse(info) => {
                self.fetch_delete = info.clone();
                if let FetchState::Success(resp) = info {
                    if let Some(body) = resp.body {
                        self.error_msg = body.error;
                    }
                }
                true
            }
            Msg::ShowError => {
                self.error_link.show();
                false
            }
            Msg::ShowDeleteDialog => {
                self.delete_dialog.show();
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Props) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let FetchState::NotFetching = self.fetch.clone() {
            self.link.send_message(Msg::GetPreview);
        }
        let user_logined = if let FetchState::NotFetching = self.fetch_info2.clone() {
            self.link.send_message(Msg::GetInfo2);
            None
        } else if let FetchState::Success(user) = self.fetch_info2.clone() {
            user.body
        } else {
            None
        };
        let info = if let FetchState::Success(resp) = self.fetch.clone() {
            resp.body
        } else {
            None
        };
        let user = if let Some(resp) = info.clone() {
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
        if let FetchState::Success(resp) = self.fetch_delete.clone() {
            if let Some(body) = resp.body {
                match body.error {
                    BlogError::Nothing => {
                        self.link.send_message(Msg::GoMain);
                    }
                    _ => {
                        self.link.send_message(Msg::ShowError);
                    }
                }
            }
        }
        if let Some(resp) = info {
            if let Some(post) = resp.post {
                let html_output = render::render(post.body);
                let mut tags = String::new();
                for i in 0..post.tags.len() {
                    tags.push_str(&format!("#{}", post.tags[i].to_uppercase()));
                    if i != post.tags.len() - 1 {
                        tags.push_str(", ");
                    }
                }
                if tags == String::from("#") {
                    tags = String::from("NO TAGS");
                }
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
                let render: web_sys::HtmlDivElement = html_document
                    .create_element("div")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlDivElement>()
                    .unwrap();
                render.set_inner_html(&html_output);
                html! {
                    <div class="container">
                    <MatDialog heading="Are you sure?" dialog_link=self.delete_dialog.clone() onclosing=self.link.callback(|action: String| match action.as_str() { "yes" => Msg::GetDelete, _ => Msg::Dummy })>
                    {"Are you sure to delete this post?"}
                    <MatDialogAction action_type=ActionType::Secondary action="no">
                        <MatButton label="No" />
                    </MatDialogAction>
                    <MatDialogAction action_type=ActionType::Primary action="yes">
                        <MatButton label="Yes" />
                    </MatDialogAction>
                </MatDialog>
                        <MatSnackbar label_text=&format!("Failed to create post: {}", self.error_msg) snackbar_link=self.error_link.clone()/>
                        <div class="block">
                        <h2>{post.title}</h2>
                        <MatList><li divider=true role="separator"></li></MatList>
                        <MatListItem graphic=GraphicType::Avatar twoline=true noninteractive=true>
                            <span>{if let Some(user_info) = user.clone() { user_info.nickname.clone() } else { String::from("ERROR") }}</span>
                            <span slot="secondary">{&format!("{}", Seoul.from_utc_datetime(&post.created_at))}</span>
                            <mwc-icon slot="graphic" class="inverted">{"tag_faces"}</mwc-icon>
                        </MatListItem>
                        <MatList><li divider=true role="separator"></li></MatList>
                            <div class="markdown-body">
                        {if let Ok(node) = web_sys::Node::try_from(render) {
                            let vnode = VNode::VRef(node);
                            vnode
                          } else {
                            html! {
                              <div class="error">{"error"}</div>
                            }
                          }}
                            </div>
                        <MatList><li divider=true role="separator"></li></MatList>
                        <MatListItem noninteractive=true><b>{tags}</b></MatListItem>
                        {
                            if let Some(user1) = user_logined.clone() {
                                if let Some(user2) = user.clone() {
                                    if user1 == user2 {
                                        html!{
                                            <div class="button-grid">
                                                <router::MainRouterAnchor route=router::MainRoute::Editor(self.props.id)><MatButton label="EDIT" raised=true/></router::MainRouterAnchor>
                                                <span onclick=self.link.callback(|_| Msg::ShowDeleteDialog)><MatButton label="DELETE" raised=true/></span>
                                            </div>
                                        }
                                    } else {
                                        html!{}
                                    }
                                } else {
                                    html!{}
                                }
                            } else {
                                html!{}
                            }
                        }
                    </div>
                    </div>
                    }
            } else {
                html! {}
            }
        } else {
            html! {}
        }
    }
}
