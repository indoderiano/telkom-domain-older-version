use yew::prelude::*;

pub struct SingleSignOn {}

pub enum Msg {}

impl Component for SingleSignOn {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SingleSignOn {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{ "SingleSignOn" }</p>
        }
    }
}
