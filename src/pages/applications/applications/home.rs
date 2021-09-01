use yew::prelude::*;
// use yew_router::components::RouterAnchor;
// use crate::app::Route;

pub struct ApplicationHome {}

pub enum Msg {}

impl Component for ApplicationHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ApplicationHome {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // type Anchor = RouterAnchor<Route>;
        html! {
        <>
                    <div 
                        class="col py-3"
                    >
                        <div>
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
                                        {"Applications"}
                                    </div>
                                    <div>
                                        <button 
                                            type="button" 
                                            class="btn btn-primary d-flex align-items-center" 
                                            data-bs-toggle="modal"
                                            data-bs-target="#exampleModal"
                                        >
                                            <i  
                                                class="bi bi-plus me-2" 
                                                style="margin-left: -5px;"
                                            >
                                            </i>
                                            <span>{"Create Application"}</span>
                                        </button>
                                    </div>
                                    </div>
                                    <p>{"Setup a application to use for Authentication."}</p>
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
                                        <img
                                            src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg" style=" color: transparent;
                                            width: 100%;
                                            height: 100%;
                                            object-fit: cover;
                                            text-align: center;
                                            text-indent: 10000px;"
                                        />
                                    </div>

                                    <div 
                                        class="d-grid" 
                                        style="min-width: 40px;"
                                    >
                                        <a
                                            class="fw-bold mb-0" 
                                            style=" white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 14px;
                                                    text-decoration: none;" 
                                            href="#">
                                            {"API Explorer Application"}
                                        </a>
                                        <p 
                                            class="mb-0 text-muted" 
                                            style=" white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 14px;"
                                        >
                                            {"Machine to Machine"}
                                        </p>
                                    </div>
                                </div>

                                <div 
                                    class="p-3 d-flex flex-fill align-items-center text-muted"
                                >
                                    <span 
                                        style="font-size: 14px; margin-right: 8px; white-space: nowrap;"
                                    >
                                    {"Client ID:"}
                                    </span>
                                    <div 
                                        class="rounded" 
                                        style=" background-color: #eff0f2;
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                                padding: 2px 6px;
                                                font-family: 'Roboto Mono', monospace;"
                                    >
                                        {"AunM1dD5rf3p6xALjnssbrVWSiYiFBcy"}
                                    </div>
                                    <i 
                                        class="bi bi-files ms-1"
                                    >
                                    </i>
                                </div>

                                <div 
                                    class="p-3 d-flex align-items-center dropdown"
                                >
                                    <button 
                                        type="button" 
                                        style="flex: 0 0 auto; width: 30px; height: 30px;"
                                        class="btn d-flex justify-content-center align-items-center rounded border" role="button"
                                        id="dropdownMenuButton1" 
                                        data-bs-toggle="dropdown" 
                                        aria-expanded="false"
                                    >
                                        <i 
                                            class="bi bi-three-dots"
                                        >
                                        </i>
                                    </button>
                                    <ul 
                                        class="dropdown-menu" 
                                        aria-labelledby="dropdownMenuButton1"
                                    >
                                        <li>
                                            <a 
                                                class="dropdown-item fs-7" 
                                                href="#"
                                            >
                                                {"Quickstart"}
                                            </a>
                                        </li>
                                        <li>
                                            <a 
                                                class="dropdown-item fs-7" 
                                                href="#"
                                            >
                                                {"Settings"}
                                            </a>
                                        </li>
                                        <li>
                                            <a 
                                                class="dropdown-item fs-7" 
                                                href="#"
                                            >
                                                {"API"}
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <div>
                            <div 
                                class="d-flex border-bottom border-1 list-hover"
                            >
                                <div class="p-3 d-flex" style="width: 40%;">
                                    <div style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                                    class="d-flex justify-content-center align-items-center rounded me-3">
                                    <img
                                        src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg" style=" color: transparent;
                                        width: 100%;
                                        height: 100%;
                                        object-fit: cover;
                                        text-align: center;
                                        text-indent: 10000px;"/>
                                    </div>

                                    <div class="d-grid" style="min-width: 40px;">
                                    <a class="fw-bold mb-0" style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                                text-decoration: none;
                                            " href="#">
                                        {"Management API (Test Application)"}
                                    </a>
                                    <p class="mb-0 text-muted" style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                            ">
                                        {"Machine to Machine"}
                                    </p>
                                </div>

                                </div>

                                <div class="p-3 d-flex flex-fill align-items-center text-muted">
                                    <span style="font-size: 14px; margin-right: 8px; white-space: nowrap;">
                                        {"Client ID:"}
                                    </span>
                                    <div class="rounded" style="
                                            background-color: #eff0f2;
                                            white-space: nowrap;
                                            text-overflow: ellipsis;
                                            overflow: hidden;
                                            font-size: 14px;
                                            padding: 2px 6px;
                                            font-family: 'Roboto Mono', monospace;"
                                    >
                                        {"AunM1dD5rf3p6xALjnssbrVWSiYiFBcy"}
                                    </div>
                                    <i class="bi bi-files ms-1"></i>
                                </div>

                                <div class="p-3 d-flex align-items-center dropdown">
                                    <button type="button" style="flex: 0 0 auto; width: 30px; height: 30px;"
                                    class="btn d-flex justify-content-center align-items-center rounded border" role="button"
                                    id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false">
                                    <i class="bi bi-three-dots"></i>
                                    </button>
                                    <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                    <li><a class="dropdown-item fs-7" href="#">{"Quickstart"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Settings"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"API"}</a></li>
                                    </ul>
                                </div>

                            </div>
                        </div>

                        <div>
                            <div class="d-flex border-bottom border-1 list-hover">
                                <div class="p-3 d-flex" style="width: 40%;">
                                    <div style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                                    class="d-flex justify-content-center align-items-center rounded me-3">
                                    <img src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/none.svg"
                                        style=" color: transparent;
                                        width: 100%;
                                        height: 100%;
                                        object-fit: cover;
                                        text-align: center;
                                        text-indent: 10000px;"/>
                                    </div>

                                    <div class="d-grid" style="min-width: 40px;">
                                    <a class="fw-bold mb-0" style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                                text-decoration: none;
                                            " href="#">
                                        {"Default App"}
                                    </a>
                                    <p class="mb-0 text-muted" style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                            ">
                                        {"Generic"}
                                    </p>
                                    </div>

                                </div>

                                <div class="p-3 d-flex flex-fill align-items-center text-muted">
                                    <span style="font-size: 14px; margin-right: 8px; white-space: nowrap;">
                                    {"Client ID:"}
                                    </span>
                                    <div class="rounded" style="
                                        background-color: #eff0f2;
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                        padding: 2px 6px;
                                        font-family: 'Roboto Mono', monospace;
                                    ">
                                    {"AunM1dD5rf3p6xALjnssbrVWSiYiFBcy"}
                                    </div>
                                    <i class="bi bi-files ms-1"></i>
                                </div>

                                <div class="p-3 d-flex align-items-center dropdown">
                                    <button type="button" style="flex: 0 0 auto; width: 30px; height: 30px;"
                                    class="btn d-flex justify-content-center align-items-center rounded border" role="button"
                                    id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false">
                                    <i class="bi bi-three-dots"></i>
                                    </button>
                                    <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                    <li><a class="dropdown-item fs-7" href="#">{"Quickstart"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Settings"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Addons"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Connections"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Organizations"}</a></li>
                                    </ul>
                                </div>

                            </div>
                        </div>

                        <div>
                            <div class="d-flex border-bottom border-1 list-hover">
                                <div class="p-3 d-flex" style="width: 40%;">
                                    <div style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                                    class="d-flex justify-content-center align-items-center rounded me-3">
                                    <img src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/spa.svg"
                                        style=" color: transparent;
                                        width: 100%;
                                        height: 100%;
                                        object-fit: cover;
                                        text-align: center;
                                        text-indent: 10000px;"/>
                                    </div>

                                    <div class="d-grid" style="min-width: 40px;">
                                    <a class="fw-bold mb-0" style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                                text-decoration: none;
                                            " href="#">
                                        {"My App"}
                                    </a>
                                    <p class="mb-0 text-muted" style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                            ">
                                        {"Single Page Application"}
                                    </p>
                                    </div>
                                </div>

                                <div class="p-3 d-flex flex-fill align-items-center text-muted">
                                    <span style="font-size: 14px; margin-right: 8px; white-space: nowrap;">
                                    {"Client ID:"}
                                    </span>
                                    <div class="rounded" style="
                                        background-color: #eff0f2;
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                        padding: 2px 6px;
                                        font-family: 'Roboto Mono', monospace;
                                    ">
                                        {"AunM1dD5rf3p6xALjnssbrVWSiYiFBcy"}
                                    </div>
                                    <i class="bi bi-files ms-1"></i>
                                </div>

                                <div class="p-3 d-flex align-items-center dropdown">
                                    <button type="button" style="flex: 0 0 auto; width: 30px; height: 30px;"
                                    class="btn d-flex justify-content-center align-items-center rounded border" role="button"
                                    id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false">
                                    <i class="bi bi-three-dots"></i>
                                    </button>
                                    <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                    <li><a class="dropdown-item fs-7" href="#">{"Quickstart"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Settings"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Addons"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Connections"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Organizations"}</a></li>
                                    </ul>
                                </div>
                            </div>
                        </div>

                        <div>
                            <div class="d-flex border-bottom border-1 list-hover">
                                <div class="p-3 d-flex" style="width: 40%;">
                                    <div style="flex: 0 0 auto; width: 40px; height: 40px; background-color: #eff0f2;"
                                    class="d-flex justify-content-center align-items-center rounded me-3">
                                    <img
                                        src="https://cdn.auth0.com/manhattan/versions/1.3226.0/assets/non_interactive.svg" style=" color: transparent;
                                        width: 100%;
                                        height: 100%;
                                        object-fit: cover;
                                        text-align: center;
                                        text-indent: 10000px;"/>
                                    </div>

                                <div class="d-grid" style="min-width: 40px;">
                                    <a class="fw-bold mb-0" style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                                text-decoration: none;
                                            " href="#">
                                        {"Test (Test Application)"}
                                    </a>
                                    <p class="mb-0 text-muted" style="
                                                white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;
                                            ">
                                        {"Machine To Machine"}
                                    </p>
                                </div>
                            </div>

                            <div class="p-3 d-flex flex-fill align-items-center text-muted">
                                <span style="font-size: 14px; margin-right: 8px; white-space: nowrap;">
                                    {"Client ID:"}
                                </span>
                                <div class="rounded" style="
                                        background-color: #eff0f2;
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                        padding: 2px 6px;
                                        font-family: 'Roboto Mono', monospace;"
                                >
                                    {"AunM1dD5rf3p6xALjnssbrVWSiYiFBcy"}
                                </div>
                                <i class="bi bi-files ms-1"></i>
                            </div>

                            <div class="p-3 d-flex align-items-center dropdown">
                                    <button type="button" style="flex: 0 0 auto; width: 30px; height: 30px;"
                                    class="btn d-flex justify-content-center align-items-center rounded border" role="button"
                                    id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false">
                                    <i class="bi bi-three-dots"></i>
                                    </button>
                                    <ul class="dropdown-menu" aria-labelledby="dropdownMenuButton1">
                                    <li><a class="dropdown-item fs-7" href="#">{"Quickstart"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"Settings"}</a></li>
                                    <li><a class="dropdown-item fs-7" href="#">{"API"}</a></li>
                                    </ul>
                            </div>
                            </div>
                        </div>


                        // <!-- Modal -->
                        <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                            <div class="modal-dialog modal-dialog-scrollable">
                                <div class="modal-content">
                                <div class="modal-header">
                                    <h5 class="modal-title" id="exampleModalLabel">{"Create Application"}</h5>
                                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                                </div>
                                <div class="modal-body" style="font-size: 14px;">
                                    <div class="mb-4">
                                    <label for="basic-url" class="form-label fw-bold">{"Name"}</label>
                                    <div class="input-group mb-2">
                                        <input type="text" class="form-control" placeholder="My App" id="basic-url" aria-describedby="basic-addon3" />
                                    </div>
                                    <label class="form-label text-muted">{"This is your application name"}</label>
                                    </div>
                                    <div class="mb-4">
                                    <label for="basic-url" class="form-label fw-bold">{"Application type"}</label>
                                    <div class="input-group mb-2">
                                        <input type="text" class="form-control" id="basic-url" aria-describedby="basic-addon3" />
                                    </div>
                                    <label class="form-label text-muted">{"Ex: Web Application"}</label>
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
                    </div>
                </div>
        </>
        }
    }
}
