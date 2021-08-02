use yew::prelude::*;
use yew_router::prelude::*;
// use yew_router::components::RouterAnchor;
use yew::services::ConsoleService;
// use yewdux::prelude::*;
use yewdux::prelude::WithDispatch;
use yewtil::NeqAssign;
// use yew_router::switch::{Permissive};
use yew_router::route::Route;
use yew_router::service::RouteService;

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
            dispatch,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {

        // let acc_ref = &account;
        let acc = self.dispatch.state().clone();
        // let route_service = RouteService::new();
        let render = Router::render(move |switch: AppRoute| {
            let mut route_service = RouteService::new();
            match switch {
                AppRoute::GettingStarted => html! {<GettingStarted/>},
                AppRoute::ApisHome if acc.username == None => {
                    ConsoleService::info("redirect");
                    route_service.set_route("/", ());
                    html! {<HomePage/>}
                },
                AppRoute::ApisHome => html! {<ApisHome/>},
                AppRoute::Settings => html! {<Settings/>},
                AppRoute::ApplicationHome => html! {<ApplicationHome/>},
                AppRoute::Home if acc.username == None => html!{<HomePage/>}, 
                AppRoute::Home => {
                    route_service.set_route("/manage", ());
                    html! {<GettingStarted/>}
                },
                // html! {<HomePage/>},
                AppRoute::LoginPage if acc.username == None => {html! {<WithDispatch<LoginPage>/>}},
                AppRoute::LoginPage => {
                    ConsoleService::info("redirect");
                    // self.route_service.set_route("/manage", ());
                    route_service.set_route("/manage", ());
                    html! {<GettingStarted/>}
                },
                AppRoute::RegisterPage => html!{<RegisterPage/>},
                AppRoute::RequestPassPage => html!{<RequestPassPage/>},
                // _ => html! {
                //     <GettingStarted/>
                // },
            }
        });

        let account = self.dispatch.state().clone();
        if account.username == None {
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
                    <WithDispatch<Navtop>/>
                    
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
