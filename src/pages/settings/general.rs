use yew::prelude::*;

pub struct SettingsGeneral {}

pub enum Msg {}

impl Component for SettingsGeneral {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SettingsGeneral {}
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
                            style="font-size: 16px;"
                        >
                            {"Settings"}
                        </div>
                        <div
                            class="w-50"
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
                                    />   
                                </div>
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
                                    />   
                                </div>
                            </div>

                            <button type="button" class="btn btn-primary mb-5 mt-3">{"Save"}</button>
            
                        </div>
                    </div>
            
            
                    
                    <div
                        class="d-flex flex-row border-bottom mt-5"
                    >
                        <div
                            class="w-50"
                        >
                            <div
                                class="text-color-primary fw-bold"
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
                            class="w-50"
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
                                    {"Default Audience"}
                                </p>
                                <div class="input-group mb-2">
                                    <input
                                        type="text"
                                        class="form-control bg-input-grey"
                                        aria-label="Dollar amount (with dot and two decimal places)"
                                        placeholder="https://your-default-endpoint/"
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
                        class="d-flex flex-row border-bottom mt-5"
                    >
                        <div
                            class="w-50"
                        >
                            <div
                                class="text-color-primary fw-bold"
                                style="font-size: 16px;"
                            >
                                {"Error Pages"}
                            </div>
                        </div>
                        <div
                            class="w-50"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Error Page"}
                                </p>
                                <div class="card card-hover mb-2" style="cursor: pointer;">
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <input class="form-check-input me-2" type="radio" name="flexRadioDefault" id="flexRadioDefault1"/>

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
                                <div class="card card-hover mb-2" style="cursor: pointer;">
                                    <div class="card-body p-3">

                                        <div
                                            class="d-flex"
                                        >
                                            <input class="form-check-input me-2" type="radio" name="flexRadioDefault" id="flexRadioDefault2"/>

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

                            </div>


                            <button type="button" class="btn btn-primary mb-5 mt-3">{"Save"}</button>
            
                        </div>
                    </div>

                    <div
                        class="d-flex flex-row border-bottom mt-5"
                    >
                        <div
                            class="w-50 text-color-primary fw-bold"
                            style="font-size: 16px;"
                        >
                            {"Languages"}
                        </div>
                        <div
                            class="w-50"
                        >
                            <div
                                class="mb-4"
                            >
                                <p class="mb-2 fw-bold">
                                    {"Default Language"}
                                </p>
                                <select class="form-select" aria-label="Default select example">
                                    <option selected=true>{"Open this select menu"}</option>
                                    <option value="1">{"One"}</option>
                                    <option value="2">{"Two"}</option>
                                    <option value="3">{"Three"}</option>
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
                                            {"Bosnian (bs)"}
                                        </label>
                                    </div>

                                    <div
                                        class="form-check mb-2 d-flex align-items-center w-50 d-inline-flex"
                                        style="width: 49%;"
                                    >
                                        <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                        <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                            {"Bulgarian (bg)"}
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
