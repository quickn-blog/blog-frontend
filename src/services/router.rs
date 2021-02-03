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
    #[to = "/posts/{id}"]
    ViewPost(u32),
    #[to = "/"]
    Main,
}

impl Default for MainRoute {
    fn default() -> Self {
        Self::Main
    }
}