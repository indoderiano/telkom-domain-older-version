use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;

pub struct EnterpriseGoogleCreate {}

pub enum Msg {}

impl Component for EnterpriseGoogleCreate {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        EnterpriseGoogleCreate {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
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
                    route=AppRoute::EnterpriseGoogle
                    classes="text-decoration-none domain-link-dark"
                >
                    <i class="bi bi-arrow-left me-2"></i>
                    {"Back to Google Workplace"}
                </Anchor>

                <div
                    class="d-flex mb-5 mt-3 align-items-center"
                >
                    <div
                        style="flex: 0 0 auto; width: 64px; height: 64px;"
                        class="d-flex justify-content-center align-items-center rounded me-3 border"
                    >
                        <img
                            src="/assets/icons/google-avatar.png"
                            class="w-75"
                            style="filter: grayscale(100%);"
                        />
                    </div>

                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{"New Google Workspace Connection"}</h2>
                    </div>
                </div>

                <div>
                    <div
                        class="container border rounded p-4 d-flex flex-column mb-5"
                        style="font-size: 14px;"
                    >
                        <div
                            class="row border-bottom"
                        >
                            <div
                                class="col-lg-5 text-color-primary fw-bold mb-4"
                            >
                                {"General"}
                            </div>
                            <div
                                class="col-lg-7"
                            >
                                <div
                                    class="mb-4"
                                >
                                    <p class="mb-2 fw-bold">
                                        {"Name"}
                                    </p>
                                    <div class="input-group mb-2">
                                        <input
                                            type="text"
                                            class="form-control bg-input-grey"
                                            aria-label="Dollar amount (with dot and two decimal places)"
                                            value="google-oauth2"
                                        />   
                                    </div>
                                    <p
                                        class="text-color-disabled"
                                    >
                                        {"If you are triggering a login manually, this is the identifier you would use on the connection parameter."}
                                    </p>
                                </div>
                
                                <div
                                    class="mb-4"
                                >
                                    <p class="mb-2 fw-bold">
                                        {"Client ID"}
                                    </p>
                                    <div class="input-group mb-2">
                                        <input
                                            type="text"
                                            class="form-control"
                                            aria-label="Dollar amount (with dot and two decimal places)"
                                            placeholder="Leave blank to use telkom-dev keys"
                                        />   
                                    </div>
                                    <a
                                        href="javascript: void(0)"
                                        class="text-decoration-none"
                                    >
                                        <span
                                            style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                                text-decoration: none;
                                            "
                                        >
                                            {"How to obtain a Client ID?"}
                                        </span>
                                    </a>
                                </div>
                
                                <div
                                    class="mb-5"
                                >
                                    <p class="mb-2 fw-bold">
                                        {"Client Secret"}
                                    </p>
                                    <div class="input-group mb-2">
                                        <input
                                            type="text"
                                            class="form-control"
                                            aria-label="Dollar amount (with dot and two decimal places)"
                                            placeholder="Leave blank to use telkom-dev keys"
                                        />
                                    </div>
                                    <p
                                        class="text-color-disabled"
                                    >
                                        {"For security purposes, we don’t show your existing Client Secret."}
                                    </p>
                                </div>
    
                                <div
                                    class="mb-5"
                                >
                                    <p class="mb-2 fw-bold">
                                        {"Allowed Mobile Client IDs"}
                                    </p>
                                    <div class="input-group mb-2">
                                        <textarea
                                            class="form-control"
                                            rows="4"
                                        ></textarea>
                                    </div>
                                    <p
                                        class="text-color-disabled"
                                    >
                                        {"You can specify multiple valid client IDs by comma-separating them"}
                                    </p>
                                </div>
    
                                <div
                                    class="mb-5"
                                >
                                    <p class="mb-2 fw-bold">
                                        {"Attributes"}
                                    </p>
                                    <div
                                        class="d-flex"
                                    >
                                        <div class="form-check mb-2 d-flex align-items-center w-50">
                                            <input
                                                class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"
                                                checked=true
                                                disabled=true
                                            />
                                            <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                                {"Basic Profile "}
                                                <i class="bi bi-question-circle"></i>
                                                <span class="badge bg-light text-dark" style="text-transform: uppercase; letter-spacing: 1px;">{"REQUIRED"}</span>
                                            </label>
                                        </div>
    
                                        <div class="form-check mb-2 d-flex align-items-center w-50">
                                            <input
                                                class="form-check-input me-2 mt-0"
                                                style="font-size: 16px;"
                                                type="checkbox"
                                                checked=true
                                                disabled=true
                                            />
                                            <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                                {"Extended Profile "}
                                                <i class="bi bi-question-circle"></i>
                                                <span class="badge bg-light text-dark" style="text-transform: uppercase; letter-spacing: 1px;">{"REQUIRED"}</span>
                                            </label>
                                        </div>
    
                                    </div>
                                </div>
    
                                <div
                                    class="mb-5"
                                >
                                    <p class="mb-2 fw-bold">
                                        {"Permissions"}
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
                                                {"Contacts"}
                                                <i class="bi bi-question-circle ms-1"></i>
                                            </label>
                                        </div>
    
                                        <div
                                            class="form-check mb-2 d-flex align-items-center w-50 d-inline-flex"
                                            style="width: 49%;"
                                        >
                                            <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                            <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                                {"Blogger"}
                                                <i class="bi bi-question-circle ms-1"></i>
                                            </label>
                                        </div>
    
                                        <div
                                            class="form-check mb-2 d-flex align-items-center w-50 d-inline-flex"
                                            style="width: 49%;"    
                                        >
                                            <input class="form-check-input me-2 mt-0" style="font-size: 16px;" type="checkbox"/>
                                            <label class="form-check-label" style="font-size: 14px;" for="flexCheckCheckedDisabled">
                                                {"Calendar"}
                                                <i class="bi bi-question-circle ms-1"></i>
                                            </label>
                                        </div>
    
                                    </div>
                                </div>
                
                            </div>
                        </div>    
                
                        <div
                            class="row border-bottom mt-5"
                        >
                            <div
                                class="col-lg-5 text-color-primary fw-bold mb-4"
                            >
                                {"Advanced"}
                            </div>
                            <div
                                class="col-lg-7"
                            >
                                <div
                                    class="mb-4"
                                >
                                    <p class="mb-2 fw-bold">
                                        {"Sync user profile attributes at each login"}
                                    </p>
                                    <div class="form-check form-switch fs-3 mb-4">
                                        <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault"/>
                                    </div>
                                </div>
                
                                <button type="button" class="btn btn-primary mb-5 mt-3">{"Create"}</button>
                
                
                            </div>
                        </div>
                
                    </div>
                </div>

            </div>


        }
    }
}
