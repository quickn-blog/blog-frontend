use yew_router::prelude::*;

pub type MainRouter = Router<MainRoute>;
pub type MainRouterAnchor = RouterAnchor<MainRoute>;

#[derive(Switch, Debug, Clone)]
pub enum MainRoute {
    #[to = "/accounts/login"]
    Login,
    #[to = "/accounts/register"]
    Register,
    #[to = "/accounts/dashboard"]
    Dashboard,
    #[to = "/view_post/{id}"]
    ViewPost(i64),
    #[to = "/editor/{id}"]
    Editor(i64),
    #[to = "/about"]
    About,
    #[to = "/list"]
    ListPosts,
    #[to = "/"]
    Main,
}

impl Default for MainRoute {
    fn default() -> Self {
        Self::Main
    }
}
