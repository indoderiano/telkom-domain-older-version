use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;

pub struct SettingsSigningKeys {}

impl Component for SettingsSigningKeys {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsSigningKeys {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div
                    class="mb-5"
                >
                    <h5
                        class="mb-2"
                    >
                        {"Signing Keys"}
                    </h5>
                    <p>{"Securely manage signing keys used by the applications in your tenant. "}</p>

                </div>

                <div
                    class="mb-5"
                >
                    <div
                        class="mb-4"
                    >
                        <h5
                            class="mb-2"
                        >
                            {"Rotation Settings"}
                        </h5>
                        <p>{"The actions below allow you to rotate the signing key and certificate used to validate tokens. You can see the next in queue valid signing key in the table below for updating your application’s configuration before the rotation."}</p>
            
                    </div>

                    <div
                        class="border rounded p-4 mb-4"
                    >
                        <div
                            class="d-flex border-bottom pb-5"
                        >
                            <div
                                class="flex-fill"
                            >
                                <div
                                    class="fw-bolder mb-1"
                                    style="font-size: 16px;"
                                >
                                    {"Rotate Signing Keys"}
                                </div>
                                <div>
                                    {"This will only rotate the currently used signing key. All tokens signed with this key will continue to be valid."}
                                </div>
                            </div>

                            <button type="button" class="btn btn-light">{"Rotate Key"}</button>
                        </div>

                        <div
                            class="d-flex pt-5 pb-5"
                        >
                            <div
                                class="flex-fill"
                            >
                                <div
                                    class="fw-bolder mb-1"
                                    style="font-size: 16px;"
                                >
                                    {"Rotate & Revoke Signing Key"}
                                </div>
                                <div>
                                    {"This will rotate and additionally revoke the currently used signing key and all tokens signed with this key will no longer be valid."}
                                </div>
                            </div>

                            <button type="button" class="btn btn-danger">{"Rotate & Revoke Key"}</button>
                        </div>
                    </div>

                </div>

                <div
                    class="mb-5"
                >
                    <div
                        class="mb-4"
                    >
                        <h5
                            class="mb-2"
                        >
                            {"List of Valid Keys"}
                        </h5>
                        <p>{"This is a list of valid keys for your tenant. They are also available at the "}</p>

                    </div>

                    <table class="table">
                        <thead>
                        <tr>
                            <th scope="col" class="p-3">{"Status"}</th>
                            <th scope="col" class="p-3">{"Key ID"}</th>
                            <th scope="col"></th>
                        </tr>
                        </thead>
                        <tbody>

                            <tr>
                                <td
                                    class="p-3"
                                >
                                    <span
                                        class="badge bg-primary fw-bolder"
                                        style="text-transform: uppercase; letter-spacing: 1px; font-size: 10px;"
                                    >{"next in queue"}</span>
                                </td>
                                <td
                                    class="p-3 flex-fill"
                                >
                                    <span
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
                                        {"OlHlmtYtepG9mSCiEwurY"}
                                    </span>
                                </td>
                                <td
                                    class="p-3"
                                >
                                    <div
                                        class="d-flex justify-content-end align-items-center dropdown"
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
                                                <span class="dropdown-item fs-7">
                                                    {"Settings"}
                                                </span>
                                            </li>
                                        </ul>
                                    </div>
                                </td>
                            </tr>

                            <tr>
                                <td
                                    class="p-3"
                                >
                                    <span
                                        class="badge bg-success fw-bolder"
                                        style="text-transform: uppercase; letter-spacing: 1px; font-size: 10px;"
                                    >{"currently used"}</span>
                                </td>
                                <td
                                    class="p-3 flex-fill"
                                >
                                    <span
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
                                        {"4r7Fyp-RVC01VIgO5X6hc"}
                                    </span>
                                </td>
                                <td
                                    class="p-3"
                                >
                                    <div
                                        class="d-flex justify-content-end align-items-center dropdown"
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
                                                <span class="dropdown-item fs-7">
                                                    {"Settings"}
                                                </span>
                                            </li>
                                        </ul>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>



                </div>

                <div
                    class="mb-5"
                >
                    <div
                        class="mb-4"
                    >
                        <h5
                            class="mb-2"
                        >
                            {"List of Revoked Keys"}
                        </h5>
                        <p>{"This is a list of the last 3 revoked keys for your tenant. Further data for revocation is available via tenant logs."}</p>

                    </div>

                    <table class="table">
                        <thead>
                        <tr>
                            <th scope="col" class="p-3">{"Revoked on"}</th>
                            <th scope="col" class="p-3">{"Key ID"}</th>
                            <th scope="col"></th>
                        </tr>
                        </thead>
                        <tbody>
                        </tbody>
                    </table>
                    <div
                        class="mt-3 p-3 text-center"
                        style="background-color: #eff0f2;"
                    >
                        {"There are no items to display"}
                    </div>

                </div>

            </div>
        }
    }
}
