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
                <div class="container"></div>
                    <div class="row">
                        <div class="col">
                            <div 
                                style=
                                "margin: 1cm 0cm 0cm 3cm;
                                font-weight: 700;
                                font-size: 48px
                                ">
                                {"We're protecting your digital life"}
                            </div>
                            <div 
                                style=
                                "margin: 1cm 0cm 0cm 3cm;
                                font-weight: 700;
                                font-size: 48px
                                ">
                                {"The cybersecurity at your hand itself"}
                            </div>
                        </div>
                        <div class="col">
                            <div class="landing-page-logo">
                            <a
                                class="app-logo"
                                target="_blank" 
                            >
                            </a>
                            </div>
                        </div>
                    </div>
            </>
        }
    }
}
