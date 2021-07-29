use yew::prelude::*;
use yew_router::prelude::*;
// use yew_router::components::RouterAnchor;
use yew::services::ConsoleService;
// use yewdux::prelude::*;
use yewdux::prelude::WithDispatch;
use yewtil::NeqAssign;
// use yew_router::switch::{Permissive};
use yew_router::route::Route;

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

    // applications::apis::{
    //     home::ApisHome,
    //     settings::Settings
    // },

    applications::{
        applications::home::ApplicationHome,
        apis::{
            home::ApisHome,
            settings::Settings,
        },
    },

    outer::{
        login_page::LoginPage,
        register_page::RegisterPage,
        password_page::RequestPassPage,
    },
    // reducer_account_view::ReducerAccountView,
    // testing_fetch::TestingFetch,

};

use crate::components::{
    navtop::Navtop,
    sidebar::Sidebar,
    // landing_page_navtop::LandingPageNavTop,
};

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/apis/settings"]
    Settings,
    #[to = "/apis"]
    ApisHome,
    #[to = "/applications"]
    ApplicationHome,
    
    #[to = "/login/password"]
    RequestPassPage,
    #[to = "/login"]
    LoginPage,
    #[to = "/register"]
    RegisterPage,

    #[to = "/"]
    Home,

    #[to = "/manage"]
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

        // let renderouter = Router::render(|switch: RouteNonMember| match switch {
        //     RouteNonMember::Home => html! {<HomePage/>},
        //     RouteNonMember::LoginPage => html! {<WithDispatch<LoginPage>/>},
        //     RouteNonMember::RegisterPage => html! {<RegisterPage/>},
        //     RouteNonMember::RequestPassPage => html! {<RequestPassPage/>}
        // });

        let render = Router::render(|switch: AppRoute| match switch {
            AppRoute::GettingStarted => html! {<GettingStarted/>},
            AppRoute::ApisHome => html! {<ApisHome/>},
            AppRoute::Settings => html! {<Settings/>},
            AppRoute::ApplicationHome => html! {<ApplicationHome/>},
            AppRoute::Home => html! {<HomePage/>},
            AppRoute::LoginPage => html!{<WithDispatch<LoginPage>/>},
            AppRoute::RegisterPage => html!{<RegisterPage/>},
            AppRoute::RequestPassPage => html!{<RequestPassPage/>},
            _ => html! {
                <GettingStarted/>
            },
        });
        let account = self.dispatch.state().clone();

        if account.name == None {
            html! {
                <>
                    <main>
                        <Router<AppRoute, ()>
                            render=render
                            redirect = Router::redirect(|route: Route| {
                                ConsoleService::info(&route.route);
                                AppRoute::LoginPage
                            })
                        />
                    </main>
                </>
            }
        } else {
            html! {
                <>
                    <Navtop/>
                    
                    <div
                        class="container-fluid"
                    >
                        <div
                            class="row flex-nowrap"
                        >
                            <Sidebar/>
                            <div 
                                class="col"
                            >
                                <Router<AppRoute, ()>
                                    render=render
                                    // https://github.com/yewstack/yew_router/blob/master/examples/router_component/src/main.rs#L88
                                    redirect = Router::redirect(|route: Route| {
                                        ConsoleService::info(&route.route);
                                        AppRoute::LoginPage
                                        // Route::PageNotFound(Permissive(Some(route.route)))
                                    })
                                />
                            </div>
                        </div>
                        
                    </div>
                    // <TestingFetch/>
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
