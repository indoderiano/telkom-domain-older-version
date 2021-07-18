use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterAnchor;
// use yew::services::ConsoleService;
// use yewdux::prelude::WithDispatch;
// use yewdux::prelude::*;
// use yewtil::NeqAssign;

use crate::pages::{
    home::Home,
    details::Details,
    // reducer_global::ReducerGlobal,

    applications::apis::{
        home::ApisHome,
        settings::Settings
    },
};


#[derive(Switch, Clone)]
pub enum Route {
    #[to = "/details"]
    Details,
    #[to = "/apis/settings"]
    Settings,
    #[to = "/apis"]
    ApisHome,
    #[to = "/"]
    Home,
}

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            Route::Home => html! {<Home/>},
            Route::Details => html! {<Details/>},
            Route::ApisHome => html! {<ApisHome/>},
            Route::Settings => html! {<Settings/>},
        });
        type Anchor = RouterAnchor<Route>;
        html! {
            <div>
                // <p>{ "Hello world!" }</p>
                // <p>{"Reducer"}</p>
                // <WithDispatch<ReducerGlobal>/>
                <Anchor route=Route::Home classes="item">
                  {"Home"}
                </Anchor>
                <Anchor route=Route::Details classes="item">
                  {"Details"}
                </Anchor>
                <Anchor route=Route::ApisHome classes="item">
                  {"APIs"}
                </Anchor>
                <main>
                    <Router<Route, ()> render=render/>
                </main>
                // <p></p>

                // <ApisHome/>
            </div>
        }
    }
}
