use yew::prelude::*;
use yewdux::prelude::WithDispatch;
use crate::components::landing_page_navtop::LandingPageNavTop;

pub struct HomePage {}

pub enum Msg {}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        HomePage {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <WithDispatch<LandingPageNavTop>/>
                <div>
                    {"HOMEPAGE"}
                </div>
            </>
        }
    }
}
