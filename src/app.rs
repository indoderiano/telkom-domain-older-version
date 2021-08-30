use yew::prelude::*;
use yew_router::prelude::*;
// use yew_router::components::RouterAnchor;
use yew::services::{
    ConsoleService,
    storage::{ StorageService, Area },
};
use yew::format::{ Json, Text };
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

    outer::{
        login_page::LoginPage,
        register_page::RegisterPage,
        password_page::RequestPassPage,
    },
    
    home_page::HomePage,
    getting_started::GettingStarted,
    activity::Activity,

    applications::{
        applications::home::ApplicationHome,
        apis::{
            home::ApisHome,
            settings::ApisSettings,
        },
        sso::{
            home::SsoHome,
            create_sso::CreateSso,
        },
    },

    authentication::{
        social::{
            home::SocialHome,
            settings::SocialSettings,
            create::SocialCreate,
        },
        enterprise::{
            home::EnterpriseHome,
            google_apps::EnterpriseGoogle,
            google_apps_create::EnterpriseGoogleCreate,
        },
        passwordless::home::AuthPasswordLess,
    },

    settings::{
        home::SettingsHome,
    },

    // reducer_account_view::ReducerAccountView,
    // testing_fetch::TestingFetch,

};

use crate::components::{
    navtop::Navtop,
    sidebar::Sidebar,
};

use crate::store::types::LocalStorage;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/login/password"]
    RequestPassPage,
    #[to = "/login"]
    LoginPage,
    #[to = "/register"]
    RegisterPage,
    #[to = "/apis/settings"]
    ApisSettings,
    #[to = "/getting-started"]
    GettingStarted,
    #[to = "/apis"]
    ApisHome,
    #[to = "/activity"]
    Activity,
    #[to = "/applications"]
    ApplicationHome,
    #[to = "/authentication/passwordless"]
    AuthPasswordless,
    #[to = "/sso/create-sso"]
    CreateSso,
    #[to = "/sso"]
    SsoHome,
    #[to = "/social/create"]
    SocialCreate,
    #[to = "/social/settings"]
    SocialSettings,
    #[to = "/social"]
    SocialHome,
    #[to = "/enterprise/google-app/create"]
    EnterpriseGoogleCreate,
    #[to = "/enterprise/google-app"]
    EnterpriseGoogle,
    #[to = "/enterprise"]
    EnterpriseHome,
    #[to = "/tenant"]
    SettingsHome,
    #[to = "/"]
    Home,
}


const KEY: &str = "telkom-domain";

pub struct App {
    dispatch: AppDispatch,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _: ComponentLink<Self>) -> Self {
        
        let mut storage = StorageService::new(Area::Local).expect("storage was disabled");
        
        // SET LOCALSTORAGE
        // let user_data = LocalStorage{
        //     username: String::from("batman"),
        //     email: String::from("batman@mail.com"),
        // };
        // let localstorage_data = Json(&user_data);

        // // let localstorage_data: Result<String, anyhow::Error> = Ok(String::from("tokendata_telkomdomain"));
        // storage.store(KEY, localstorage_data);


        // GET LOCALSTORAGE
        let localstorage_data = {
            if let Json(Ok(data)) = storage.restore(KEY) {
                // ConsoleService::info(&token);
                data
            } else {
                ConsoleService::info("token does not exist");
                LocalStorage {
                    username: None,
                    email: None,
                    token: None,
                }
            }
        };

        ConsoleService::info(&format!("{:?}", localstorage_data));

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
            let is_logged_in = if acc.username == None {false} else {true};
            let mut route_service = RouteService::new();
            if is_logged_in {
                match switch {
                    AppRoute::Activity => html!{<Activity/>},
                    AppRoute::GettingStarted => html! {<GettingStarted/>},
                    AppRoute::ApisHome => html! {<ApisHome/>},
                    AppRoute::ApisSettings => html! {<ApisSettings/>},
                    AppRoute::ApplicationHome => html! {<ApplicationHome/>},
                    AppRoute::AuthPasswordless => html! {<AuthPasswordLess/>},
                    AppRoute::SsoHome => html! {<SsoHome/>},
                    AppRoute::CreateSso => html! {<CreateSso/>},
                    AppRoute::SocialHome => html! {<SocialHome/>},
                    AppRoute::SocialSettings => html! {<SocialSettings/>},
                    AppRoute::SocialCreate => html! {<SocialCreate/>},
                    AppRoute::EnterpriseHome => html! {<EnterpriseHome/>},
                    AppRoute::EnterpriseGoogle => html! {<EnterpriseGoogle/>},
                    AppRoute::EnterpriseGoogleCreate => html! {<EnterpriseGoogleCreate/>},
                    AppRoute::SettingsHome => html! {<SettingsHome/>},
                    _ => {
                        route_service.set_route("/manage", ());
                        html! {<GettingStarted/>}
                    },
                }
            } else {
                match switch {
                    AppRoute::Home => html! {<HomePage/>},
                    AppRoute::LoginPage => html! {<WithDispatch<LoginPage>/>},
                    AppRoute::RegisterPage => html!{<RegisterPage/>},
                    AppRoute::RequestPassPage => html!{<RequestPassPage/>},
                    _ => {
                        route_service.set_route("/", ());
                        html! {<HomePage/>}
                    },
                }
            }
            // match switch {
            //     AppRoute::GettingStarted => html! {<GettingStarted/>},
            //     AppRoute::ApisHome if !is_logged_in => {
            //         ConsoleService::info("redirect");
            //         route_service.set_route("/", ());
            //         html! {<HomePage/>}
            //     },
            //     AppRoute::ApisHome => html! {<ApisHome/>},
            //     AppRoute::Settings => html! {<Settings/>},
            //     AppRoute::ApplicationHome => html! {<ApplicationHome/>},
            //     AppRoute::Home if !is_logged_in => html!{<HomePage/>}, 
            //     AppRoute::Home => {
            //         route_service.set_route("/manage", ());
            //         html! {<GettingStarted/>}
            //     },
            //     // html! {<HomePage/>},
            //     AppRoute::LoginPage if !is_logged_in => {html! {<WithDispatch<LoginPage>/>}},
            //     AppRoute::LoginPage => {
            //         ConsoleService::info("redirect");
            //         // self.route_service.set_route("/manage", ());
            //         route_service.set_route("/manage", ());
            //         html! {<GettingStarted/>}
            //     },
            //     AppRoute::RegisterPage => html!{<RegisterPage/>},
            //     AppRoute::RequestPassPage => html!{<RequestPassPage/>},
            //     // _ => html! {
            //     //     <GettingStarted/>
            //     // },
            // }
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
