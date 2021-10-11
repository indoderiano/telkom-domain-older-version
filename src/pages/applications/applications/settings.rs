use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;
use super::tab_connection::ConnectionTab;
use super::tab_settings::TabSettings;
use yew::services::ConsoleService;
use crate::types::application::{ AppDetails, ResponseAppDetails };
use crate::components::loading2::Loading2;


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct AppsSettingsProps {
    pub tenant_id: String,
    pub app_id: String,
}

pub enum Content {
    Settings,
    Connection
}

pub struct ApplicationSettings {
    content: Content,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    error: Option<String>,
    app_details: AppDetails,
}

pub enum Msg {
    ChangeContent(Content),
    RequestAppDetails,
    GetAppDetails(Result<ResponseAppDetails, anyhow::Error>)
}

impl Component for ApplicationSettings {
    type Message = Msg;
    type Properties = AppsSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let app_details = AppDetails {
            tenant: String::from("default"),
            name: String::from("default"),
            domain: String::from("default"),
            client_id: String::from("default"),
            client_secret: String::from("default"),
            description: String::from("default"),
            app_logo: String::from("default"),
            app_type: String::from("default"),
            authentication_method: String::from("default"),
            login_url: String::from("default"),
            allowed_urls : String::from("default"),
            allowed_logout_urls : String::from("default"),
            allowed_web_origins: String::from("default"),
            allowed_origins: String::from("default"),
            token_exp: 32000,
            refresh_token_rotation: false,
            refresh_token_rotation_interval: 32000,
            refesh_token_absolute_expiration: false,
            refesh_token_absolute_expiration_lifetime: 32000,
            refesh_token_inactivity_expiration: false,
            refesh_token_inactivity_expiration_lifetime: 32000,
        };


        ApplicationSettings {
            content: Content::Settings,
            link,
            fetch_task: None,
            error: None,
            app_details
        }
    }

    fn rendered(&mut self, first_render: bool) {

        if first_render {
            ConsoleService::info("This is first render");
            
            self.link.send_message(Msg::RequestAppDetails);
        }

    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
            Msg::RequestAppDetails => {
                let request = Request::get("http://localhost:3000/applications/tenantid/applications/dev-1wj84p4q")
                    // .header("Content-Type", "application/json")
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<ResponseAppDetails, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::GetAppDetails(data)
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetAppDetails(response) => {
                match response {
                    Ok(data) => {
                        ConsoleService::info(&format!("{:?}", data));
                        self.app_details = data.data;
                    }
                    Err(error) => {
                        ConsoleService::info(&error.to_string());
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        let tenant_id = self.app_details.tenant.clone();
        html! {
            <div
                class="py-5 px-4 m-auto"
                style="max-width: 1048px; font-size:14px;"
            >
                <Anchor
                    route=AppRoute::ApplicationHome { tenant_id }
                    classes="text-decoration-none domain-link-dark"
                >
                    <i class="bi bi-arrow-left me-2"></i>
                    {"Back to Applications"}
                </Anchor>

                {
                    if self.fetch_task.is_some() {
                        html! {
                            <div
                                style="
                                    position: relative;
                                    margin-top: 8rem;
                                "
                            >
                                <Loading2 width=45 />
                            </div>
                        }
                    } else {
                        html! {
                            { self.view_content() }
                        }
                    }
                }

            </div>
        }
    }
}

impl ApplicationSettings {
    fn view_content (&self) -> Html {
        let AppDetails {
            tenant: _,
            name,
            domain: _,
            client_id,
            client_secret: _,
            description: _,
            app_logo: _,
            app_type,
            authentication_method: _,
            login_url: _,
            allowed_urls : _,
            allowed_logout_urls : _,
            allowed_web_origins: _,
            allowed_origins: _,
            token_exp: _,
            refresh_token_rotation: _,
            refresh_token_rotation_interval: _,
            refesh_token_absolute_expiration: _,
            refesh_token_absolute_expiration_lifetime: _,
            refesh_token_inactivity_expiration: _,
            refesh_token_inactivity_expiration_lifetime: _,
        } = self.app_details.clone();

        html! {
            <>
                <div
                    class="d-flex mb-5 mt-3"
                >
                    <div
                        style="flex: 0 0 auto; width: 64px; height: 64px;"
                        class="d-flex justify-content-center align-items-center rounded me-4 border"
                    >
                        <img
                            src="https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/spa.svg"
                            class="w-50"
                        />
                    </div>

                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{name}</h2>
                        <div
                            class="text-muted"
                        >
                            <span
                                class="me-4"
                            >
                                {app_type}
                            </span>
                            <span>
                                {"Client ID"}
                            </span>
                            <span
                                class="rounded ms-2"
                                style="
                                    background-color: #eff0f2;
                                    white-space: nowrap;
                                    text-overflow: ellipsis;
                                    overflow: hidden;
                                    font-size: 14px;
                                    padding: 2px 6px;
                                    font-family: 'Roboto Mono', monospace;
                                "
                            >
                                {client_id}
                            </span>
                        </div>
                    </div>
                </div>

                <div
                    class="mb-4"
                >
                    <ul class="nav nav-tabs">
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Settings))
                            class="nav-item"
                        >
                        <a
                            // class="nav-link active"
                            class={
                                match self.content {
                                    Content::Settings => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            aria-current="page"
                            href="#"
                        >
                            {"Settings"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Connection))
                            class="nav-item">
                        <a
                            // class="nav-link"
                            class={
                                match self.content {
                                    Content::Connection => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Connection"}</a>
                        </li>
                    </ul>
                </div>

                // <Quickstart/>
                // <TabSettings/>

                {
                    match self.content {
                        Content::Connection => html! { <ConnectionTab/> },
                        Content::Settings => html! { <TabSettings app_details=self.app_details.clone()/> }
                    }
                }

            </>
        }
    }
}