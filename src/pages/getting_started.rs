use yew::prelude::*;

pub struct GettingStarted {}

pub enum Msg {}

impl Component for GettingStarted {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        GettingStarted {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "GETTING STARTED" }</p>
        }
    }
}
