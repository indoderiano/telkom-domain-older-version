use crate::app::AppRoute;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct UsersManagement {}

pub enum Msg {}

impl Component for UsersManagement {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        UsersManagement {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // type Anchor = RouterAnchor<AppRoute>;
        html! {
            <div>
           
                <div class="mx-auto pt-5 pb-5 px-4" style="max-width: 1048px;">

                    
                            <p class="fs-2 fw-bold">{"Users"}</p>
    
                </div>





            </div>
        }
    }
}
