use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{ FetchService, FetchTask, Request, Response },
        ConsoleService,
    },
};
use super::general::SettingsGeneral;
use super::tenant_members::SettingsTenantMembers;
use super::custom_domain::SettingsCustomDomain;
use super::signing_keys::SettingsSigningKeys;
use crate::types::settings::{
    TenantSettings,
};
use crate::configs::server::API_URL;
use crate::components::{
    loading2::Loading2,
};

pub enum Content {
    General,
    TenantMembers,
    CustomDomains,
    SigningKeys,
    Advanced,
}

pub enum StateError {
    GetSettings,
}

pub struct SettingsHome {
    content: Content,
    link: ComponentLink<Self>,
    tenant_settings: Option<TenantSettings>,
    loading_request_settings: bool,
    error_request_settings: Option<String>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    ChangeContent(Content),
    RequestSettingsDetails,
    GetSettingsDetails(TenantSettings),
    ResponseError(String, StateError),
}

impl Component for SettingsHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsHome {
            content: Content::General,
            link,
            tenant_settings: None,
            loading_request_settings: false,
            error_request_settings: None,
            fetch_task: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("first render in settings");
            self.link.send_message(Msg::RequestSettingsDetails);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
            Msg::RequestSettingsDetails => {
                let request = Request::get(format!("{}/tenant/v2/settings", API_URL))
                    // .header("Content-Type", "application/json")
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<TenantSettings, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::GetSettingsDetails(dataok)
                            }
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::GetSettings)
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_request_settings = None;
                self.loading_request_settings = true;
                true
            }
            Msg::GetSettingsDetails(data) => {
                self.tenant_settings = Some(data);
                self.loading_request_settings = false;
                self.fetch_task = None;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::GetSettings => {
                        self.loading_request_settings = false;
                        self.error_request_settings = Some(message);
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
        html! {
            <div
                class="py-5 px-4 m-auto"
                style="max-width: 1048px; font-size:14px;"
            >
                <div
                    class="d-flex mb-5 mt-3"
                >
                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{"Tenant Settings"}</h2>
                    </div>
                </div>

                <div
                    class="mb-4"
                >
                    <ul class="nav nav-tabs">
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::General))
                            class="nav-item"
                        >
                        <a
                            // class="nav-link active"
                            class={
                                match self.content {
                                    Content::General => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            aria-current="page"
                            href="#"
                        >
                            {"General"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::TenantMembers))
                            class="nav-item">
                        <a
                            class={
                                match self.content {
                                    Content::TenantMembers => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Tenant Members"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::CustomDomains))
                            class="nav-item">
                            <a
                                class={
                                    match self.content {
                                        Content::CustomDomains => "nav-link active",
                                        _ => "nav-link"
                                    }
                                }
                                href="#"
                            >
                                {"Custom Domain"}
                            </a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::SigningKeys))
                            class="nav-item">
                            <a
                                class={
                                    match self.content {
                                        Content::SigningKeys => "nav-link active",
                                        _ => "nav-link"
                                    }
                                }
                                href="#"
                            >
                                {"Signing Keys"}
                            </a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::Advanced))
                            class="nav-item">
                            <a
                                class={
                                    match self.content {
                                        Content::Advanced => "nav-link active",
                                        _ => "nav-link"
                                    }
                                }
                                href="#"
                            >
                                {"Advanced"}
                            </a>
                        </li>
                    </ul>
                </div>

                {
                    if self.loading_request_settings {
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
                    } else if self.error_request_settings.is_some() {
                        html! {
                            <div class="alert alert-warning mb-5" role="alert">
                                <i class="bi bi-exclamation-triangle me-2"></i>
                                { self.error_request_settings.clone().unwrap() }
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


impl SettingsHome {
    fn view_content (&self) -> Html {
        match self.content {
            Content::General => {
                if let Some(data) = &self.tenant_settings {
                    html! { <SettingsGeneral tenant_settings=data.clone() /> }
                } else {
                    html! {}
                }
            },
            Content::TenantMembers => html! { <SettingsTenantMembers/> },
            Content::CustomDomains => html! { <SettingsCustomDomain/> },
            Content::SigningKeys => html! { <SettingsSigningKeys/> },
            Content::Advanced => html! {},
        }
    }
}