use yew::prelude::*;

pub struct TabSettings {}

pub enum Msg {}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TabSettings {}
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
                    class="border rounded p-4 d-flex flex-column mb-5"
                    style="font-size: 14px;"
                >
                    <div
                        class="d-flex flex-row border-bottom"
                    >
                        <div
                            class="w-50 text-color-primary fw-bold"
                        >
                            {"General Settings"}
                        </div>
                        <div
                            class="w-50"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"ID"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        value="60ef247ffab77800401cca56"
                                    />   
                                </div>
                                <p>
                                    {"The API id on our system. Useful if you prefer to work directly with Auth0's Management API instead."}
                                </p>
                            </div>
            
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Name*"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        value="Testing Name"
                                    />   
                                </div>
                                <p>
                                    {"A friendly name for the API. The following characters are not allowed "}
                                    <code
                                        style="padding: 2px 6px; font-size: 11px;"
                                        class="bg-input-grey text-color-primary"
                                    >
                                        <i class="bi bi-chevron-left"></i>
                                    </code>
                                    <code
                                        style="padding: 2px 6px; font-size: 11px;"
                                        class="bg-input-grey text-color-primary"
                                    >
                                        <i class="bi bi-chevron-right"></i>
                                    </code>
                                </p>
                            </div>
            
                            <div
                                class="mb-5"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Identifier"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        value="https://test-api/"
                                    />
                                </div>
                                <p>
                                    {"Unique identifier for the API. This value will be used as the "}
                                    <code
                                        style="padding: 2px 6px; font-size: 14px;"
                                        class="bg-input-grey text-color-primary"
                                    >
                                        {"audience"}
                                    </code>
            
                                    {" parameter on authorization calls."}
                                </p>
                            </div>
            
                        </div>
                    </div>
            
            
                    
                    <div
                        class="d-flex flex-row border-bottom mt-5"
                    >
                        <div
                            class="w-50 text-color-primary fw-bold"
                        >
                            {"Tokens Settings"}
                        </div>
                        <div
                            class="w-50"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Token Expirations (Seconds)*"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        value="86400"
                                    />
                                </div>
                                <p>
                                    {"Expiration value (in seconds) for "}
                                    <code
                                        style="padding: 2px 6px; font-size: 11px;"
                                        class="bg-input-grey text-color-primary"
                                    >
                                        {"access tokens"}
                                    </code>
                                    {" issued for this API from the Token Endpoint."}
                                </p>
                            </div>
            
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Token Expirations For Browser Flows (Seconds)*"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        value="7200"
                                    />
                                </div>
                                <p>
                                    {"Expiration value (in seconds) for "}
                                    <code
                                        style="padding: 2px 6px; font-size: 11px;"
                                        class="bg-input-grey text-color-primary"
                                    >
                                        {"access tokens"}
                                    </code>
                                    {" issued for this API via Implicit or Hybrid Flows. Cannot be greater than the Token Lifetime value."}
                                </p>
                            </div>
            
                            <div
                                class="mb-5"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Signing Algorithm"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        value="RS256"
                                    />
                                </div>
                                <p>
                                    {"Algorithm to be used when signing the "}
                                    <code
                                        style="padding: 2px 6px; font-size: 11px;"
                                        class="bg-input-grey text-color-primary"
                                    >
                                        {"access tokens"}
                                    </code>
            
                                    {" for this API. You can find more information"}
                                </p>
                            </div>
            
                        </div>
                    </div>
            
            
                    <div
                        class="d-flex flex-row border-bottom mt-5"
                    >
                        <div
                            class="w-50 text-color-primary fw-bold"
                        >
                            {"RBAC Settings"}
                        </div>
                        <div
                            class="w-50"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Enable RBAC "}
                                </p>
                                <div class="form-check form-switch fs-3 mb-4">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault"/>
                                </div>
                                <p>
                                    {"If this setting is enabled, this API will skip user consent for applications flagged as First Party."}
                                </p>
                            </div>
            
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Allow Offline Access"}
                                </p>
                                <div class="form-check form-switch fs-3 mb-4">
                                    <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault"/>
                                </div>
                                <p>
                                    {"If this setting is enabled, Auth0 will allow applications to ask for Refresh Tokens for this API."}
                                </p>
                            </div>
            
                            <button type="button" class="btn btn-primary mb-5 mt-3">{"Save"}</button>
            
            
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
                                {"Delete This API"}
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
