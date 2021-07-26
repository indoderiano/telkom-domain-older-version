use yew::prelude::*;
use yew_router::prelude::*;
// use yew_router::components::RouterAnchor;
// use yew::services::ConsoleService;
// use yewdux::prelude::*;
// use yewdux::prelude::WithDispatch;
use yewtil::NeqAssign;

use crate::store::reducer_account::{
    AppDispatch,
    // DataAccountAction,
    // DataAccount
};

use crate::pages::{
    // home::Home,
    details::Details,
    home_page::HomePage,
    // reducer_global::ReducerGlobal,

    applications::apis::{
        home::ApisHome,
        settings::Settings
    },
    // reducer_account_view::ReducerAccountView,

};

use crate::components::{
    navtop::Navtop,
    sidebar::Sidebar,
    // landing_page_navtop::LandingPageNavTop,
};


#[derive(Switch, Clone)]
pub enum Route {
    #[to = "/details"]
    Details,
    #[to = "/apis/settings"]
    Settings,
    // #[to = "/apis"]
    // ApisHome,
    #[to = "/"]
    ApisHome,
}

pub struct App {
    dispatch: AppDispatch,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            dispatch
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            // Route::Home => html! {<Home/>},
            Route::Details => html! {<Details/>},
            Route::ApisHome => html! {<ApisHome/>},
            Route::Settings => html! {<Settings/>},
        });
        // type Anchor = RouterAnchor<Route>;
        let account = self.dispatch.state().clone();

        if account.name == None {
            html! {
                <>
                    <HomePage/>
                </>
            }
        } else {
            html! {
                <>
                    <Navtop/>
                    // <LandingPageNavTop/>
                    // { self.navtop(account) }
                    // <button onclick=update>{"update"}</button>

                    // <p>{ "Hello world!" }</p>
                    // <Anchor route=Route::Home classes="item">
                    //   {"Home"}
                    // </Anchor>
                    // <Anchor route=Route::Details classes="item">
                    //   {"Details"}
                    // </Anchor>
                    // <Anchor route=Route::ApisHome classes="item">
                    //   {"APIs"}
                    // </Anchor>
                    <div
                        style="display: flex;"
                    >
                        <div
                            class="sidebar"
                        >
                            <Sidebar/>
                        </div>
                        <main
                            style="flex: 1;"
                        >
                            <Router<Route, ()> render=render/>
                        </main>
                    </div>
                    // <p></p>
                    // <p>{"Reducer"}</p>
                    // <WithDispatch<ReducerGlobal>/>
                    // <WithDispatch<ReducerAccountView>/>
                </>
            }
        }

    }
}

// impl App {
//     fn navtop(&self, account: DataAccount) -> Html {

//         if account.name == None {
//             html! {
//                 <>
//                     <HomePage/>
//                 </>
//             }
//         } else {
//             html! {
//                 <Navtop/>
//             }
//         }
//     }
// }
