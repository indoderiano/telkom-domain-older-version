use yew::prelude::*;

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
                            <button type="button" class="btn btn-primary d-flex align-items-center">
                                <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                <span>{"Create API"}</span>
                            </button>
                        </div>
                    </div>
                    <p>{"Define APIs that you can consume from your authorized applications."}</p>
                </div>



                // <!-- LIST -->
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
                                <a
                                    class="fw-bold mb-0"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                        text-decoration: none;
                                    "
                                    href="#"
                                >
                                    {"Auth0 Management API"}
                                </a>
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
                            class="p-3 d-flex align-items-center"
                        >
                            <button
                                type="button"
                                style="flex: 0 0 auto; width: 30px; height: 30px;"
                                class="btn d-flex justify-content-center align-items-center rounded border"
                            >
                                <i class="bi bi-three-dots"></i>
                            </button>
                        </div>

                    </div>
                </div>
                
            </div>
        }
    }
}
