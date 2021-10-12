use yew::prelude::*;
// use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;

pub struct SettingsAdvanced {}

impl Component for SettingsAdvanced {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SettingsAdvanced {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >

                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Login and Logout"}
                        </div>
                        <div
                            class="col-lg-6"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Allowed Logout URLs"}
                                </p>
                                <div class="input-group mb-2">
                                    <textarea
                                        class="form-control"
                                        rows="4"
                                        placeholder="https://mycompany.org/logoutCallback"
                                    ></textarea>
                                </div>
                                <p
                                    class="mb-0"
                                >
                                    {"A set of URLs that are valid to redirect to after logout from Auth0 when no client_id is specified on the logout endpoint invocation. It's useful as a global list when SSO is enabled. Read more about this at"}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Tenant Login URI"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="https://mycompany.org/login"
                                    />
                                </div>
                                <p
                                    class="mb-0"
                                >
                                    {"In some scenarios, Auth0 will need to redirect to your tenant’s login page. This URI needs to point to a route in your application that should redirect to your tenant’s /authorize endpoint. Learn more."}
                                </p>
                            </div>

                            <div
                                class="mt-3 mb-4"
                            >
                                <button
                                    type="button"
                                    class="btn btn-primary position-relative"
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                // {
                                //     if self.error_update_settings.is_some() {
                                //     html! {
                                //         <div class="alert alert-warning mt-3" role="alert">
                                //             <i class="bi bi-exclamation-triangle me-2"></i>
                                //             { self.error_update_settings.clone().unwrap() }
                                //         </div>
                                //     }
                                //     } else {
                                //         html! {}
                                //     }
                                // }
                            </div>

                        </div>
                    </div>
            
            
                </div>



                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Login Session Management"}
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Session Cookie Mode *"}
                                </p>
                                <div
                                    class="row"
                                >
                                    <div
                                        class="col-md-6 col-sm-12 mb-2"
                                    >
                                        <div class="card border border-primary border-2">
                                            <div class="card-body">
                                            <p class="card-title mb-2 fw-bold">{"Persistent Session"}</p>
                                            <p class="card-text text-color-disabled">{"Allows the user to retain their session cookie when re-opening the browser on the same device."}</p>
                                            </div>
                                        </div>
                                    </div>
                                    <div
                                        class="col-md-6 col-sm-12 mb-2"
                                    >
                                    <div class="card">
                                        <div class="card-body">
                                            <p class="card-title mb-2 fw-bold">{"Non-Persistent Session"}</p>
                                            <p class="card-text text-color-disabled">{"Invalidates the session cookie when the browser is closed."}</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <p
                                    class="mb-0"
                                >
                                    {"By default, users will not be asked to log in again and will have a persistent cookie stored locally. This will affect all sessions managed by Auth0."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Inactivity Timeout *"}
                                </p>
                                <div class="input-group mb-3">
                                    <input type="text" class="form-control"
                                    aria-label="Recipient's username" aria-describedby="basic-addon2"/>
                                    <span class="input-group-text" id="basic-addon2">{"minutes"}</span>
                                    </div>
                                <p
                                    class="mb-0"
                                >
                                    {"Users will be asked to log in again unless they are active within this period (max."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Require login after *"}
                                </p>
                                <div class="input-group mb-3">
                                    <input type="text" class="form-control"
                                    aria-label="Recipient's username" aria-describedby="basic-addon2"/>
                                    <span class="input-group-text" id="basic-addon2">{"minutes"}</span>
                                    </div>
                                <p
                                    class="mb-0"
                                >
                                    {"Regardless of activity, users will be forced to log in after the period (max."}
                                </p>
                            </div>

                            <div
                                class="mt-3 mb-4"
                            >
                                <button
                                    type="button"
                                    class="btn btn-primary position-relative"
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                // {
                                //     if self.error_update_settings.is_some() {
                                //     html! {
                                //         <div class="alert alert-warning mt-3" role="alert">
                                //             <i class="bi bi-exclamation-triangle me-2"></i>
                                //             { self.error_update_settings.clone().unwrap() }
                                //         </div>
                                //     }
                                //     } else {
                                //         html! {}
                                //     }
                                // }
                            </div>

                        </div>
                    </div>
            
            
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 text-color-primary fw-bold mb-4"
                            style="font-size: 16px;"
                        >
                            {"Device Flow User Code Format"}
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"User Code Character Set *"}
                                </p>
                                <select
                                    class="form-select mb-2"
                                    aria-label="Default select example"
                                >
                                    <option value="RS256">{"Base-20 (BCDFGHJKLMNPQRSTVWXZ)"}</option>
                                    <option value="HS256">{"Digits (0123456789)"}</option>
                                </select>
                                <p
                                    class="mb-0 text-color-disabled"
                                >
                                    {"The character set for generating a User Code."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"User Code Mask *"}
                                </p>
                                <div class="input-group mb-3">
                                    <input type="password" class="form-control"
                                    aria-label="Recipient's username" aria-describedby="basic-addon2"/>
                                    <span class="input-group-text" id="basic-addon2">{"e.g BCDF-GHJK"}</span>
                                    </div>
                                <p
                                    class="mb-0 text-color-disabled"
                                >
                                    {"The mask is used to define the length of the User Code and to format the randomly generated User Code to a friendly, readable value with possible spaces or hyphens for readability."}
                                </p>
                            </div>

                            <div
                                class="mt-3 mb-4"
                            >
                                <button
                                    type="button"
                                    class="btn btn-primary position-relative"
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                // {
                                //     if self.error_update_settings.is_some() {
                                //     html! {
                                //         <div class="alert alert-warning mt-3" role="alert">
                                //             <i class="bi bi-exclamation-triangle me-2"></i>
                                //             { self.error_update_settings.clone().unwrap() }
                                //         </div>
                                //     }
                                //     } else {
                                //         html! {}
                                //     }
                                // }
                            </div>

                        </div>
                    </div>
            
            
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 mb-4"
                        >
                            <div
                                class="text-color-primary fw-bold mb-1"
                                style="font-size: 16px;"
                            >
                                {"Global Client Information"}
                            </div>
                            <p
                                class="text-color-disabled"
                            >
                                {"The global client ID and secret are used to generate tokens for legacy Auth0 APIs. Typically, you will not need these values. If you need to have the global client secret changed, please contact support."}
                            </p>
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Global Client ID"}
                                </p>
                                <div class="input-group mb-3">
                                    <input type="text" class="form-control" placeholder="Recipient's username" aria-label="Recipient's username" aria-describedby="button-addon2"/>
                                    <button class="btn btn-outline-secondary" type="button" id="button-addon2">
                                        <i class="bi bi-files ms-1"></i>
                                    </button>
                                </div>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Global Client Secret"}
                                </p>
                                <div class="input-group mb-3">
                                    <input type="text" class="form-control" placeholder="Recipient's username" aria-label="Recipient's username" aria-describedby="button-addon2"/>
                                    <button class="btn btn-outline-secondary" type="button" id="button-addon2">
                                        <i class="bi bi-eye-slash"></i>
                                        <i class="bi bi-eye"></i>
                                    </button>
                                    <button class="btn btn-outline-secondary" type="button" id="button-addon2">
                                        <i class="bi bi-files ms-1"></i>
                                    </button>
                                </div>
                            </div>

                        </div>
                    </div>
            
            
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 mb-4"
                        >
                            <div
                                class="text-color-primary fw-bold mb-1"
                                style="font-size: 16px;"
                            >
                                {"Settings"}
                            </div>
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Change Password flow v2"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"Enables a new version of the Change Password flow. We've deprecated the previous alternative and we strongly recommend enabling this option. This flag is presented only for backwards compatibility and once enabled you won't be able to move it back. You can configure how the Change Password widget will look like at the Password Reset tab inside the Universal Login section."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"OIDC Dynamic Application Registration"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"Enables third-party developers to dynamically register applications for your APIs. Learn more"}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Enable Application Connections"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"This flag determines whether all current connections shall be enabled when a new Application is created."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Use a generic response in public signup API error message"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"If enabled, this will use a generic response in the public signup API which will prevent users from being able to find out if an e-mail address or username has previously registered"}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Enable email verification flow during login for Azure AD and ADFS connections"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"If enabled, users will be presented with an email verification prompt during their first login when using Azure AD or ADFS connections"}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Refresh Token Revocation Deletes Grant"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"Delete underlying grant when a Refresh Token is revoked via the Authentication API."}
                                </p>
                            </div>

                        </div>
                    </div>
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 mb-4"
                            style="font-size: 16px;"
                        >
                            <div
                                class="text-color-primary fw-bold"
                            >
                                {"Extensibility"}
                            </div>
                            <p
                                class="mb-0 text-color-disabled"
                            >
                                {"Use custom scripts to extend parts of Auth0's functionality, such as Rules, Hooks and Database Connections."}
                            </p>
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Runtime *"}
                                </p>
                                <select
                                    class="form-select mb-2"
                                    aria-label="Default select example"
                                >
                                    <option value="RS256">{"Node 12"}</option>
                                </select>
                                <p
                                    class="mb-0 text-color-disabled"
                                >
                                    {"The NodeJS version environment used to execute your custom scripts."}
                                </p>
                            </div>

                            <div
                                class="mt-3 mb-4"
                            >
                                <button
                                    type="button"
                                    class="btn btn-primary position-relative"
                                >
                                    <div class="telkom-label">
                                        {"Save"}
                                    </div>
                                    <div class="telkom-spinner telkom-center">
                                        <div class="spinner-border spinner-border-sm" role="status"/>
                                    </div>
                                </button>

                                // {
                                //     if self.error_update_settings.is_some() {
                                //     html! {
                                //         <div class="alert alert-warning mt-3" role="alert">
                                //             <i class="bi bi-exclamation-triangle me-2"></i>
                                //             { self.error_update_settings.clone().unwrap() }
                                //         </div>
                                //     }
                                //     } else {
                                //         html! {}
                                //     }
                                // }
                            </div>

                        </div>
                    </div>
            
            
                </div>

                <div
                    class="container border rounded p-4 d-flex flex-column mb-4"
                    style="font-size: 14px;"
                >
                    <div
                        class="row"
                    >
                        <div
                            class="col-lg-6 mb-4"
                        >
                            <div
                                class="text-color-primary fw-bold mb-1"
                                style="font-size: 16px;"
                            >
                                {"Migrations"}
                            </div>
                        </div>
                        <div
                            class="col-lg-6"
                        >

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Disable clickjacking protection for Classic Universal Login"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"If enabled, additional HTTP security headers will not be included in the response to prevent embedding of the Universal Login prompts in an IFRAME."}
                                </p>
                            </div>

                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Fixed Length of Access Token & Authorization Code"}
                                </p>
                                <div class="form-check form-switch fs-4 mb-3">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckChecked" checked={true}/>
                                </div>
                                <p
                                    class="text-color-disabled mb-0"
                                >
                                    {"If enabled, the Auth0 platform issues Access Tokens and Authorization Codes of fixed length. By default, these are of variable length.Learn More"}
                                </p>
                            </div>

                        </div>
                    </div>
                </div>

                <div
                    style="font-size: 14px;"
                >
                    <p
                        class="fw-bold"
                    >
                        {"Danger Zone"}
                    </p>

                    <div class="alert alert-danger d-flex flex-row justify-content-between" role="alert">
                        <div>
                            <p
                                class="fw-bold"
                            >
                                {"Delete Connection"}
                            </p>
                            {"Once confirmed, this operation can't be undone!"}
                        </div>
                        <div>
                            <button
                                type="button"
                                class="btn btn-danger"
                            >{"Delete"}</button>
                        </div>
                    </div>
                </div>

            </div>
        }
    }
}
