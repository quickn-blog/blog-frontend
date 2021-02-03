use yew::prelude::*;

pub struct Main {
    link: ComponentLink<Self>
}

pub enum Msg {

}

#[derive(Properties, Clone)]
pub struct Props {

}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html!{
            {"Welcome to the ANEP Research"}
        }
    }
}