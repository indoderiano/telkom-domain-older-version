use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;

pub struct SocialHome {}

pub enum Msg {}

impl Component for SocialHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SocialHome {}
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
                            {"Social Connections"}
                        </div>
                        <div
            
                        >
                            <button
                                type="button"
                                class="btn btn-primary d-flex align-items-center"
                                data-bs-toggle="modal" data-bs-target="#exampleModal"
                            >
                                <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                <span>{"Create Connection"}</span>
                            </button>
                        </div>
                    </div>
                    <p>
                        {"Configure social connections like Facebook, Twitter, Github and others so that you can let your users login with them."}
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
                                {"Learn more"}
                                <i
                                    class="bi bi-arrow-right-short fs-5"
                                    style="vertical-align: -3px; margin-left: -2px;"></i>
                            </span>
                        </a>
                    </p>
                    <div class="alert alert-secondary" role="alert">
                        <div
                            class="fw-bold mb-3 pb-2"
                            style="
                                font-size: 13px;
                                text-transform: uppercase;
                                letter-spacing: 1px;
                                border-bottom: 1px solid rgb(200, 200, 200);
                            "
                        >
                            {"With social connection you can"}
                        </div>
                        <div
                            class="d-inline-flex flex-row w-50"
                        >
                            <i class="bi bi-info-circle-fill me-4"></i>
                            <p
                                class="pe-5"
                            >
                                {"Associate user accounts with multiple connections such as database, enterprise or social with the same user on Auth0, allowing that user to authenticate with any of them."}
                            </p>
                        </div>
                        <div
                            class="d-inline-flex flex-row"
                            style="width: 49%;"
                        >
                            <i class="bi bi-info-circle-fill me-4"></i>
                            <p
                                class="pe-5"
                            >
                                {"Let your users login with social connections like Facebook, Twitter, Github and others."}
                            </p>
                        </div>
                        <div
                            class="d-inline-flex flex-row w-50"
                        >
                            <i class="bi bi-info-circle-fill me-4"></i>
                            <p
                                class="pe-5"
                            >
                                {"Use Auth0's beautiful Login Box to let your users choose how to authenticate."}
                            </p>
                        </div>
                    </div>
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
                                <img src="/assets/icons/google-avatar.png" />
                            </div>

                            <div
                                class="d-grid"
                                style="min-width: 40px;"
                            >
                                <Anchor
                                    route=AppRoute::Settings
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
                                        {"facebook"}
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
                                    {"Facebook"}
                                </p>
                            </div>
            
                        </div>

                        <div
                            class="p-3 d-flex flex-fill align-items-center text-muted"
                        >
                            <i class="bi bi-dot fs-2"></i>
                            {"No applications enabled"}
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
                                    <Anchor route=AppRoute::Settings classes="dropdown-item fs-7">
                                        {"Settings"}
                                    </Anchor>
                                </li>
                            </ul>
                        </div>

                    </div>

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
                                <img src="/assets/icons/google-avatar.png" />
                            </div>

                            <div
                                class="d-grid"
                                style="min-width: 40px;"
                            >
                                <Anchor
                                    route=AppRoute::Settings
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
                                        {"google-oauth2"}
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
                                    {"Google / Mail"}
                                </p>
                            </div>
            
                        </div>

                        <div
                            class="p-3 d-flex flex-fill align-items-center text-muted"
                        >
                            <i class="bi bi-dot fs-2"></i>
                            {"No applications enabled"}
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
                                    <Anchor route=AppRoute::Settings classes="dropdown-item fs-7">
                                        {"Settings"}
                                    </Anchor>
                                </li>
                            </ul>
                        </div>

                    </div>

                </div>
                
            </div>
        }
    }
}
