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
use crate::types::application::{ AppDetails, RefreshToken, 
    // SigningKeys,
    JwtConfiguration };
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
    app_id: String,
    tenant_id: String,
}

pub enum Msg {
    ChangeContent(Content),
    RequestAppDetails,
    GetAppDetails(Result<AppDetails, anyhow::Error>)
}

impl Component for ApplicationSettings {
    type Message = Msg;
    type Properties = AppsSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        // let refresh_token_default = RefreshToken {
        //     expiration_type: String::from("default"),
        //     leeway: 0,
        //     infinite_token_lifetime: false,
        //     infinite_idle_token_lifetime: false,
        //     token_lifetime: 0,
        //     idle_token_lifetime: 0,
        //     rotation_type: String::from("default"),
        // }

        let app_details = AppDetails {
            tenant: String::from("default"),
            global: false,
            is_token_endpoint_ip_header_trusted: false,
            name: String::from("default"),
            is_first_party: false,
            oidc_conformant: false,
            sso_disabled: false,
            cross_origin_auth: false,
            refresh_token: RefreshToken {
                expiration_type: String::from("default"),
                leeway: 0,
                infinite_token_lifetime: false,
                infinite_idle_token_lifetime: false,
                token_lifetime: 0,
                idle_token_lifetime: 0,
                rotation_type: String::from("default"),
            },
            encrypted: false,
            allowed_clients: String::from("default"),
            callbacks: String::from("default"),
            // signing_keys: SigningKeys {
            //     cert: String::from("default"),
            //     pkcs7: String::from("default"),
            //     subject: String::from("default"),
            // },
            client_id: String::from("default"),
            callback_url_template: false,
            client_secret: String::from("default"),
            jwt_configuration: JwtConfiguration {
                alg: String::from("default"),
                lifetime_in_seconds: 0,
                secret_encoded: false,
            },
            client_aliases: String::from("default"),
            token_endpoint_auth_method: String::from("default"),
            app_type: String::from("default"),
            grant_types: String::from("default"),
            custom_login_page_on: false,
        };
        ApplicationSettings {
            content: Content::Settings,
            link,
            fetch_task: None,
            error: None,
            app_details,
            app_id: props.app_id,
            tenant_id: props.tenant_id
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
                let request = Request::get(format!("http://127.0.0.1:8080/api/v1/1/clients/{}", self.app_id))
                    // .header("Content-Type", "application/json")
                    .header("access_token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhleWthbGxAZ21haWwuY29tIiwiZXhwIjoxNjQzMDk0MTA0fQ.G_kEzjOwrzI_qD8Tco_4HTgXctsz4kUccl4e92WNZb8")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<AppDetails, anyhow::Error>>>| {
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
                        self.app_details = data;
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
            tenant,
            global,
            is_token_endpoint_ip_header_trusted,
            name,
            is_first_party,
            oidc_conformant,
            sso_disabled,
            cross_origin_auth,
            refresh_token: RefreshToken {
                expiration_type,
                leeway,
                infinite_token_lifetime,
                infinite_idle_token_lifetime,
                token_lifetime,
                idle_token_lifetime,
                rotation_type,
            },
            encrypted,
            allowed_clients,
            callbacks,
            // signing_keys: SigningKeys {
            //     cert,
            //     pkcs7,
            //     subject,
            // },
            client_id,
            callback_url_template,
            client_secret,
            jwt_configuration: JwtConfiguration {
                alg,
                lifetime_in_seconds,
                secret_encoded,
            },
            client_aliases,
            token_endpoint_auth_method,
            app_type,
            grant_types,
            custom_login_page_on,
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