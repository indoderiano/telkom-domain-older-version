use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterAnchor;
// use yew::services::ConsoleService;
use yewdux::prelude::WithDispatch;
// use yewdux::prelude::*;
// use yewtil::NeqAssign;

use crate::pages::{
    home::Home,
    details::Details,
    reducer_global::ReducerGlobal,

    applications::apis::home::ApisHome,
};


#[derive(Switch, Clone)]
enum Route {
    #[to = "/details"]
    Details,
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
        });
        type Anchor = RouterAnchor<Route>;
        html! {
            <div>
                <p>{ "Hello world!" }</p>
                <Anchor route=Route::Home classes="item">
                  {"Home"}
                </Anchor>
                <Anchor route=Route::Details classes="item">
                  {"Details"}
                </Anchor>
                <main>
                    <Router<Route, ()> render=render/>
                </main>
                <p></p>
                <p>{"Reducer"}</p>
                <WithDispatch<ReducerGlobal>/>

                <ApisHome/>
            </div>
        }
    }
}
