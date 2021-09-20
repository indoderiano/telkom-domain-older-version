use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;
use super::quickstart::Quickstart;
use super::tab_settings::TabSettings;
use yew::services::ConsoleService;
use crate::types::api::{ ApiTitle, ApiDetails, ResponseApiDetails };


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ApisSettingsProps {
    pub tenant_id: String,
    pub api_id: String,
    // api_title: ApiTitle,
}

pub enum Content {
    Quickstart,
    Settings
}

pub struct ApisSettings {
    content: Content,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    error: Option<String>,
    api_details: ApiDetails,
}

pub enum Msg {
    ChangeContent(Content),
    RequestApiDetails,
    GetApiDetails(Result<ResponseApiDetails, anyhow::Error>),
}

impl Component for ApisSettings {
    type Message = Msg;
    type Properties = ApisSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Api Settings props, tenant id = {}", props.tenant_id));
        ConsoleService::info(&format!("Api Settings props, api id = {}", props.api_id));

        let api_details = ApiDetails {
            id: 1,
            name: String::from("default"),
            api_id: String::from("default"),
            api_type: String::from("default"),
            identifier: String::from("default"),
            token_exp: 1000,
            token_exp_browser: 1000,
            sign_algorithm: String::from("default"),
            rbac: false,
            permission_acc_token: false,
            allow_skip_user: false,
            allow_off_acc: false,
            tenant_id: String::from("default"),
        };

        ApisSettings {
            content: Content::Quickstart,
            link,
            fetch_task: None,
            error: None,
            api_details,
        }
    }

    fn rendered(&mut self, first_render: bool) {

        if first_render {
            ConsoleService::info("This is first render");
            
            self.link.send_message(Msg::RequestApiDetails);
        }

    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
            Msg::RequestApiDetails => {
                let request = Request::get("http://localhost:3000/api/dev-ofzd5p1b/apis/60daccd6dff9a6003e8ef6ef")
                    // .header("Content-Type", "application/json")
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<ResponseApiDetails, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::GetApiDetails(data)
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetApiDetails(response) => {
                match response {
                    Ok(data) => {
                        ConsoleService::info(&format!("{:?}", data));
                        self.api_details = data.data;
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
        html! {
            <div
                class="py-5 px-4 m-auto"
                style="max-width: 1048px; font-size:14px;"
            >
                <Anchor
                    route=AppRoute::ApisHome{ tenant_id: String::from("testing_id") }
                    classes="text-decoration-none domain-link-dark"
                >
                    <i class="bi bi-arrow-left me-2"></i>
                    {"Back to Apis"}
                </Anchor>

                <div
                    class="d-flex mb-5 mt-3"
                >
                    <div
                        style="flex: 0 0 auto; width: 64px; height: 64px; background-color: #eff0f2;"
                        class="d-flex justify-content-center align-items-center rounded me-4"
                    >
                        <i class="bi bi-server fs-3"></i>
                    </div>

                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{"Testing Name"}</h2>
                        <div
                            class="text-muted"
                        >
                            <span
                                class="me-4"
                            >
                                {"Custom API"}
                            </span>
                            <span>
                                {"Identifier"}
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
                                {"https://test-api/"}
                            </span>
                        </div>
                    </div>
                </div>

                <div
                    class="mb-4"
                >
                    <ul class="nav nav-tabs">
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Quickstart))
                            class="nav-item"
                        >
                        <a
                            // class="nav-link active"
                            class={
                                match self.content {
                                    Content::Quickstart => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            aria-current="page"
                            href="#"
                        >
                            {"Quick Start"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Settings))
                            class="nav-item">
                        <a
                            // class="nav-link"
                            class={
                                match self.content {
                                    Content::Settings => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Settings"}</a>
                        </li>
                        <li class="nav-item">
                        <a class="nav-link" href="#">{"Permissions"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Machine to Machine Applications"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Test"}</a>
                        </li>
                    </ul>
                </div>

                // <Quickstart/>
                // <TabSettings/>

                {
                    match self.content {
                        Content::Quickstart => html! { <Quickstart/> },
                        Content::Settings => html! { <TabSettings api_details=self.api_details.clone() /> }
                    }
                }
            </div>
        }
    }
}
