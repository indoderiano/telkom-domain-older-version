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
    // details::Details,
    home_page::HomePage,
    getting_started::GettingStarted,
    // reducer_global::ReducerGlobal,

    applications::apis::{
        home::ApisHome,
        settings::Settings
    },

    outer::{
        login_page::LoginPage,
        register_page::RegisterPage,
        password_page::RequestPassPage,
    },
    // reducer_account_view::ReducerAccountView,
    testing_fetch::TestingFetch,

};

use crate::components::{
    navtop::Navtop,
    sidebar::Sidebar,
    // landing_page_navtop::LandingPageNavTop,
};

#[derive(Switch, Clone)]
pub enum RouteNonMember {
    #[to = "/login/password"]
    RequestPassPage,
    #[to = "/login"]
    LoginPage,
    #[to = "/register"]
    RegisterPage,
    #[to = "/"]
    Home,
}

#[derive(Switch, Clone)]
pub enum Route {
    #[to = "/apis/settings"]
    Settings,
    #[to = "/apis"]
    ApisHome,
    #[to = "/"]
    GettingStarted,
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

        let renderouter = Router::render(|switch: RouteNonMember| match switch {
            RouteNonMember::Home => html! {<HomePage/>},
            RouteNonMember::LoginPage => html! {<LoginPage/>},
            RouteNonMember::RegisterPage => html! {<RegisterPage/>},
            RouteNonMember::RequestPassPage => html! {<RequestPassPage/>}
        });

        let render = Router::render(|switch: Route| match switch {
            Route::GettingStarted => html! {<GettingStarted/>},
            Route::ApisHome => html! {<ApisHome/>},
            Route::Settings => html! {<Settings/>},
            // Route::LoginPage => html!{<LoginPage/>},
            // Route::RegisterPage => html!{<RegisterPage/>},
            // Route::RequestPassPage => html!{<RequestPassPage/>},
        });
        // type Anchor = RouterAnchor<Route>;
        let account = self.dispatch.state().clone();

        if account.name == None {
            html! {
                <>
                    // <WithDispatch<LandingPageNavTop>/>
                    <main>
                        <Router<RouteNonMember, ()> render=renderouter/>
                    </main>
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
                    <TestingFetch/>
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
