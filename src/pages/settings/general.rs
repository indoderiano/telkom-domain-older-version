use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use crate::types::settings::{
    TenantSettings,
};


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct SettingsTabGeneralProps {
    pub tenant_settings: TenantSettings,
}

pub struct SettingsGeneral {
    tenant_settings: TenantSettings,
    link: ComponentLink<Self>,
}

pub enum StateError {
    UpdateSettings,
    UpdateEnvironmentTag,
    UpdateAuthorization,
    UpdateErrorPage,
    UpdateLanguage,
}

pub enum Data {
    FriendlyName,
    PictureUrl,
    SupportEmail,
    SupportUrl,
    // environment tag
    DefaultAudience,
    DefaultDirectory,
    ErrorPage,
    // Language

}

pub enum Msg {
    InputString(String, Data),
    InputBool(bool, Data),
}

impl Component for SettingsGeneral {
    type Message = Msg;
    type Properties = SettingsTabGeneralProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Tenant settings = {:?}", props.tenant_settings));
        SettingsGeneral {
            tenant_settings: props.tenant_settings,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputString(value, data) => {
                match data {
                    Data::FriendlyName => {
                        self.tenant_settings.friendly_name = value;
                        true
                    }
                    Data::PictureUrl => {
                        self.tenant_settings.picture_url = value;
                        true
                    }
                    Data::SupportEmail => {
                        self.tenant_settings.support_email = value;
                        true
                    }
                    Data::SupportUrl => {
                        self.tenant_settings.support_url = value;
                        true
                    }
                    Data::DefaultAudience => {
                        self.tenant_settings.default_audience = value;
                        true
                    }
                    Data::DefaultDirectory => {
                        self.tenant_settings.default_directory = value;
                        true
                    }
                    Data::ErrorPage => {
                        self.tenant_settings.error_page.url = value;
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            Msg::InputBool(value, data) => {
                match data {
                    Data::ErrorPage => {
                        self.tenant_settings.error_page.show_log_link = value;
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let TenantSettings {
            change_password,
            guardian_mfa_page,
            default_audience,
            default_directory,
            error_page,
            device_flow,
            flags,
            friendly_name,
            picture_url,
            support_email,
            support_url,
            allowed_logout_urls,
            session_lifetime,
            idle_session_lifetime,
            sandbox_version,
            sandbox_versions_available,
            default_redirection_uri,
            enabled_locales,
            session_cookie,
        } = self.tenant_settings.clone();
        html! {
            <div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-5"
                    style="font-size: 14px;"
                >

                    <div
                        class="text-color-primary fw-bold mb-4"
                        style="font-size: 16px;"
                    >
                        {"Tenant Information"}
                    </div>
                    <div
                        class="row border-bottom mb-3"
                    >
                        <div
                            class="col-lg-6 text-muted mb-4"
                            style="font-size: 14px;"
                        >
                            {"Tenant Name"}
                        </div>
                        <div
                            class="col-lg-6 mb-3"
                        >
                            <span
                                class="text-color-primary"
                            >
                                {"dev-ofzd5p1b"}
                            </span>
                        </div>
                    </div>

                    <div
                        class="row border-bottom mb-3"
                    >
                        <div
                            class="col-lg-6 text-muted mb-4"
                            style="font-size: 14px;"
                        >
                            {"Region"}
                        </div>
                        <div
                            class="col-lg-6 mb-3"
                        >
                            <span
                                class="text-color-primary"
                            >
                                {"AU"}
                            </span>
                        </div>
                    </div>

                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 text-muted mb-4"
                            style="font-size: 14px;"
                        >
                            {"Environment"}
                        </div>
                        <div
                            class="col-lg-6 mb-3"
                        >
                            <span
                                class="text-color-primary"
                            >
                                {"Development"}
                            </span>
                        </div>
                    </div>
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-5"
                    style="font-size: 14px;"
                >

                    <div
                        class="row border-bottom"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Settings"}
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Friendly Name"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="My Company Inc."
                                        value={friendly_name.clone()}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::FriendlyName))
                                    />   
                                </div>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Logo URL"}
                                </p>
                                <div
                                    class="d-flex justify-content-center align-items-center border rounded-top"
                                    style="height: 120px;"
                                >
                                    <img
                                        src="https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/badge.png"
                                        style="height: 60px;"
                                    />
                                </div>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="Your logo URL"
                                        value={picture_url}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::PictureUrl))
                                    />   
                                </div>
                                <p
                                    class="mb-0"
                                >
                                    {"If a URL is not provided, the Auth0 logo will be used."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Support Email"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="support@my_company.com"
                                        value={support_email}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::SupportEmail))
                                    />   
                                </div>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Support URL"}
                                </p>
                                <div class="input-group mb-4">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="https://my-company.com"
                                        value={support_url}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::SupportUrl))
                                    />   
                                </div>
                            </div>

                            <button type="button" class="btn btn-primary mb-5 mt-3">{"Save"}</button>
            
                        </div>
                    </div>
            
            
                    
                    <div
                        class="row border-bottom mt-5"
                    >
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="text-color-primary fw-bold mb-4"
                                style="font-size: 16px;"
                            >
                                {"Environment Tag"}
                            </div>

                            <p
                                class="mt-2 text-color-disabled"
                            >
                                {"Assign an environment tag to your tenant to differentiate between development, staging and production environments."}
                            </p>
                            <p
                                class="text-color-disabled"
                            >
                                {"Higher rate limits are applied to tenants tagged as Production with a paid subscription."}
                            </p>
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Assign Environment Tag"}
                                </p>
                                <div class="card card-hover mb-2" style="cursor: pointer;">
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <div
                                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                                class="d-flex justify-content-center align-items-center rounded me-3 border bg-domain-secondary"
                                            >
                                                <i class="bi bi-code-slash fs-5 text-color-secondary"></i>
                                            </div>

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Development"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"The tenant is mainly used by engineers as a working environment to make configuration changes."}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>
                                <div class="card card-hover mb-2" style="cursor: pointer;">
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <div
                                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                                class="d-flex justify-content-center align-items-center rounded me-3 border bg-domain-secondary"
                                            >
                                                <i class="bi bi-search fs-5 text-color-secondary"></i>
                                            </div>

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Staging"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"The tenant is mainly used by your testing team and is used to test changes before releasing them to Production."}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>
                                <div class="card card-hover mb-2" style="cursor: pointer;">
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <div
                                                style="flex: 0 0 auto; width: 40px; height: 40px;"
                                                class="d-flex justify-content-center align-items-center rounded me-3 border bg-domain-secondary"
                                            >
                                                <i class="bi bi-check2 fs-4 text-color-secondary"></i>
                                            </div>

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Production"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"The tenant is pointed to a production instance used by your end users. This environment should be treated carefully since it could break your application."}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>

                            </div>


                            <button type="button" class="btn btn-primary mb-5 mt-3">{"Save"}</button>
            
                        </div>
                    </div>
            
            
                    <div
                        class="row border-bottom mt-5"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                        >
                            {"API Authorization Settings"}
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Audience"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="https://your-default-endpoint/"
                                        value={default_audience}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::DefaultAudience))
                                    />   
                                </div>
                                <p>
                                    {"API Audience to use by default for API Authorization flows . Note: This setting is equivalent to appending the audience to every authorization request made to the tenant for every application. This will cause new behavior that might result in breaking changes for some of your applications. If you require assistance, contact support."}
                                </p>
                            </div>
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Directory"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="Connection Name"
                                        value={default_directory}
                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::DefaultDirectory))
                                    />   
                                </div>
                                <p>
                                    {"Name of the connection to be use for Password Grant exchanges. The default_directory value should be the exact name of an existing connections of one of the following strategies: ad, auth0, email, sms, waad, adfs"}
                                </p>
                            </div>


                            <button type="button" class="btn btn-primary mb-5 mt-3">{"Save"}</button>
            
            
                        </div>
                    </div>


                    <div
                        class="row border-bottom mt-5"
                    >
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="text-color-primary fw-bold mb-4"
                                style="font-size: 16px;"
                            >
                                {"Error Pages"}
                            </div>
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Error Page"}
                                </p>
                                <div
                                    class="card card-hover mb-2"
                                    style="cursor: pointer;"
                                    onclick=self.link.callback(|_| Msg::InputBool(false, Data::ErrorPage))
                                >
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <input
                                                class="form-check-input me-2"
                                                type="radio"
                                                name="flexRadioDefault"
                                                id="flexRadioDefault1"
                                                checked={ if error_page.show_log_link { false } else { true } }
                                            />

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Generic"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"Use a generic error page generated from your account data"}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>
                                <div
                                    class="card card-hover mb-4"
                                    style="cursor: pointer;"
                                    onclick=self.link.callback(|_| Msg::InputBool(true, Data::ErrorPage))
                                >
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <input
                                                class="form-check-input me-2"
                                                type="radio"
                                                name="flexRadioDefault"
                                                id="flexRadioDefault2"
                                                checked={ if error_page.show_log_link { true } else { false } }
                                            />

                                            <div
                                                class="d-grid"
                                                style="min-width: 40px;"
                                            >
                                                <div
                                                    class="text-decoration-none fw-bold mb-0"
                                                >
                                                    <span
                                                        class="fw-bold"
                                                        style="
                                                            white-space: nowrap;
                                                            text-overflow: ellipsis;
                                                            overflow: hidden;
                                                            text-decoration: none;
                                                        "
                                                    >
                                                        {"Custom"}
                                                    </span>
                                                </div>
                                                <p
                                                    class="mb-0 text-muted"
                                                >
                                                    {"Redirect users to a specified URL instead of showing the default error page"}
                                                </p>
                                            </div>
                            
                                        </div>
                                    </div>
                                </div>

                                {
                                    if self.tenant_settings.error_page.show_log_link {
                                        html!{
                                            <div
                                                class="mb-4"
                                            >
                                                <p class="mb-2 fw-bold">
                                                    {"Custom error page URL *"}
                                                </p>
                                                <div class="input-group mb-2">
                                                    <input
                                                        type="text"
                                                        class="form-control bg-input-grey"
                                                        aria-label="Dollar amount (with dot and two decimal places)"
                                                        placeholder="http://mycompany.com/error/"
                                                        value={error_page.url}
                                                        oninput=self.link.callback(|data: InputData| Msg::InputString(data.value, Data::ErrorPage))
                                                    />   
                                                </div>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }

                            </div>


                            <button type="button" class="btn btn-primary mb-5 mt-3">{"Save"}</button>
            
                        </div>
                    </div>

                    <div
                        class="row mt-5"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Languages"}
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Language"}
                                </p>
                                <select class="form-select" aria-label="Default select example">
                                    <option selected=true>{"Open this select menu"}</option>
                                    <option value="1">{"English (en)"}</option>
                                    <option value="2">{"Indonesian (id)"}</option>
                                </select>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Support Languages"}
                                </p>
                                <div
                                    class="d-flexs"
                                >
                                    <div
                                        class="form-check mb-2 d-flex align-items-center d-inline-flex"
                                        style="width: 49%;"
                                    >
                                        <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                        <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                            {"English (en)"}
                                        </label>
                                    </div>

                                    <div
                                        class="form-check mb-2 d-flex align-items-center w-50 d-inline-flex"
                                        style="width: 49%;"
                                    >
                                        <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                        <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                            {"Indonesian (id)"}
                                        </label>
                                    </div>

                                    <div
                                        class="form-check mb-2 d-flex align-items-center w-50 d-inline-flex"
                                        style="width: 49%;"    
                                    >
                                        <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                        <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                            {"Croatian (hr)"}
                                        </label>
                                    </div>

                                </div>
                            </div>

                            <button type="button" class="btn btn-primary mb-5 mt-3">{"Save"}</button>
            
                        </div>
                    </div>
            
                </div>
            </div>

        }
    }
}
