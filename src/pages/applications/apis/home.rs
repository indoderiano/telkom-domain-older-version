use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;

pub struct ApisHome {}

pub enum Msg {}

impl Component for ApisHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ApisHome {}
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
                class="mx-auto pt-5 pb-5 px-4"
                style="max-width: 1048px;"
            >
                <div
                    class="mb-5"
                >
                    <div
                        class="d-flex flex-row mb-3"
                    >
                        <div
                            class="flex-fill fs-3 fw-bold"
                        >
                            {"APIs"}
                        </div>
                        <div
            
                        >
                            <button
                                type="button"
                                class="btn btn-primary d-flex align-items-center"
                                data-bs-toggle="modal" data-bs-target="#exampleModal"
                            >
                                <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                <span>{"Create API"}</span>
                            </button>
                        </div>
                    </div>
                    <p>{"Define APIs that you can consume from your authorized applications."}</p>
                </div>



                <div>
                    <div
                        class="d-flex border-bottom border-1 list-hover"
                    >
                        <div
                            class="p-3 d-flex"
                            style="width: 40%;"
                        >
                            <div
                                style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                                class="d-flex justify-content-center align-items-center rounded me-3"
                            >
                                <i class="bi bi-gear"></i>
                            </div>

                            <div
                                class="d-grid"
                                style="min-width: 40px;"
                            >
                                <Anchor
                                    route=AppRoute::ApisSettings
                                    classes="text-decoration-none fw-bold mb-0"
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
                                        {"Auth0 Management API"}
                                    </span>
                                </Anchor>
                                <p
                                    class="mb-0 text-muted"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                    "
                                >
                                    {"System API"}
                                </p>
                            </div>
            
                        </div>

                        <div
                            class="p-3 d-flex flex-fill align-items-center text-muted"
                        >
                            <span
                                style="font-size: 14px; margin-right: 8px; white-space: nowrap;"
                            >
                                {"API Audience:"} 
                            </span>
                            <div
                                class="rounded"
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
                                {"https://dev-r5y8heyf.au.auth0.com/api/v2/"}
                            </div>
                            <i class="bi bi-files ms-1"></i>
                        </div>

                        <div
                            class="p-3 d-flex align-items-center dropdown"
                        >
                            <button
                                type="button"
                                style="flex: 0 0 auto; width: 30px; height: 30px;"
                                class="btn d-flex justify-content-center align-items-center rounded border"
                                role="button"
                                id="dropdownMenuButton1"
                                data-bs-toggle="dropdown"
                                aria-expanded="false"
                            >
                                <i class="bi bi-three-dots"></i>
                            </button>
                            <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                <li>
                                    <Anchor route=AppRoute::ApisSettings classes="dropdown-item fs-7">
                                        {"Settings"}
                                    </Anchor>
                                </li>
                            </ul>
                        </div>

                    </div>
                </div>


                // MODAL CREATE APIs
                <div
                    class="modal fade"
                    id="exampleModal"
                    tabindex="-1"
                    aria-labelledby="exampleModalLabel"
                    aria-hidden="true"
                >
                    <div class="modal-dialog modal-dialog-scrollable">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"New API"}</h5>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body" style="font-size: 14px;">
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Name"}</label>
                                    <div class="input-group mb-2">
                                        <input type="text" class="form-control" id="basic-url" aria-describedby="basic-addon3"/>
                                    </div>
                                    <label class="form-label text-muted">{"A friendly name for the API"}</label>
                                </div>
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Identifier"}</label>
                                    <div class="input-group mb-2">
                                        <input type="text" class="form-control" id="basic-url" aria-describedby="basic-addon3"/>
                                    </div>
                                    <label class="form-label text-muted">{"A logical identifier for this API. We recommend using a URL but note that this doesn’t have to be a publicly available URL, Auth0 will not call your API at all.This field cannot be modified."}</label>
                                </div>
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Signing Algorithm"}</label>
                                    <select class="form-select mb-2" aria-label="Default select example">
                                        <option value="1">{"RS256"}</option>
                                        <option value="2">{"HS256"}</option>
                                    </select>
                                    <label class="form-label text-muted">{"Algorithm to sign the tokens with. When selecting RS256 the token will be signed with Auth0's private key."}</label>
                                </div>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                                <button type="button" class="btn btn-primary">{"Create"}</button>
                            </div>
                        </div>
                    </div>
                </div>
                
            </div>
        }
    }
}
