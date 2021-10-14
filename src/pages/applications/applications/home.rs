use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use crate::app::AppRoute;
use crate::types::{
    application::{AppList, ResponseAppList},
    ResponseMessage,
};
use yew::services::ConsoleService;
use yew_router::components::RouterAnchor;

use crate::components::loading2::Loading2;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct AppProps {
    pub tenant_id: String,
}

pub enum StateError {
    AppList,
}

pub struct ApplicationHome {
    tenant_id: String,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    loading_get_app: bool,
    app_list: Vec<AppList>,
    error_app_list: Option<String>,
}

pub enum Msg {
    DefaultState,
    RequestAppList,
    GetAppList(Vec<AppList>),
    ResponseError(String, StateError),
}

impl ApplicationHome {
    fn view_app_list(&self) -> Vec<Html> {
        type Anchor = RouterAnchor<AppRoute>;
        let tenant_id = &self.tenant_id;
        self.app_list.iter().map(|app| {
            html! {
                <>
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
                                        src={app.logo_uri.clone()} style=" color: transparent;
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
                                    <Anchor route=AppRoute::ApplicationSettings { tenant_id: tenant_id.clone(), app_id: app.client_id.clone() } >
                                            <a
                                                class="fw-bold mb-0"
                                                style=" white-space: nowrap;
                                                        text-overflow: ellipsis;
                                                        overflow: hidden;
                                                        font-size: 14px;
                                                        text-decoration: none;" 
                                                href="#">
                                                { &app.name }
                                            </a>
                                        </Anchor>
                                    <p
                                        class="mb-0 text-muted"
                                        style=" white-space: nowrap;
                                                text-overflow: ellipsis;
                                                overflow: hidden;
                                                font-size: 14px;"
                                    >
                                        { &app.app_type }
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
                                    { &app.client_id }
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
                                        <Anchor route=AppRoute::ApplicationSettings { tenant_id: tenant_id.clone(), app_id: app.client_id.clone() }classes="dropdown-item fs-7">
                                            {"Settings"}
                                        </Anchor>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </>
            }
        })
        .collect()
    }

    fn view_app_list_empty(&self) -> Html {
        html! {
            <div style="
                display: flex;
                text-align: center;
                align-items: center;
                flex-direction: column;
                margin-top: 60px;
                padding: 40px;
                border-radius: 6px;
                border: 1px solid #e3e4e6;"
            >

                <img width="150" height=""
                    src="https://assets-global.website-files.com/60058af53d79fbd8e14841ea/602e971e34a1e12c00b8c9ab_sso.svg"
                />

                <h4
                    style="padding-top: 20px;"
                >
                    {"You don't have any App yet."}
                </h4>
                <button
                    type="button"
                    class="btn btn-primary mt-3 d-flex align-items-center"
                    // data-bs-toggle="modal"
                    // data-bs-target="#exampleModal"
                    // onclick=self.link.callback(|_| {Msg::ShowModalCreate(true)})
                >
                    <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                    <span>{"Create APP"}</span>
                </button>
            </div>
        }
    }
}

impl Component for ApplicationHome {
    type Message = Msg;
    type Properties = AppProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Apps home props, tenant id = {}", props.tenant_id));
        ApplicationHome {
            tenant_id: props.tenant_id,
            error_app_list: None,
            fetch_task: None,
            link,
            loading_get_app: false,
            app_list: Vec::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info("This is first render");
            self.link.send_message(Msg::RequestAppList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DefaultState => {
                // self.show_modal_create = false;
                self.loading_get_app = false;
                // self.error_api_list: None,
                // self.loading_create_app = false;
                // self.error_api_create: None,
                // self.api_create.name = String::from("");
                // self.api_create.identifier = String::from("");
                // self.api_create.sign_algorithm = String::from("");
                true
            }
            Msg::RequestAppList => {
                let request = Request::get("http://localhost:3000/applications/tenantid")
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<ResponseAppList, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("{:?}", dataok));
                                Msg::GetAppList(dataok.data)
                            } 
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::AppList)
                            }
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error_app_list = None;
                self.loading_get_app = true;
                true
            }
            Msg::GetAppList(data) => {
                self.app_list = data;
                self.loading_get_app = false;
                self.fetch_task = None;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::AppList => {
                        self.loading_get_app = false;
                        self.error_app_list = Some(message);
                    }
                }
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
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
                        
                        {
                    if self.loading_get_app {
                        html! {
                            <div
                                style="
                                    position: relative;
                                    margin-top: 8rem;
                                "
                            >
                                <Loading2 width=45 />
                            </div>
                        }
                    } else if self.error_app_list.is_some() {
                        html! {
                            <div class="alert alert-warning mb-5" role="alert">
                                <i class="bi bi-exclamation-triangle me-2"></i>
                                { self.error_app_list.clone().unwrap() }
                            </div>
                        }
                    } else if self.app_list.len() == 0 {
                        html! {
                            <>
                                { self.view_app_list_empty() }
                            </>
                        }
                    } else {
                        html! {
                            <>
                                { self.view_app_list() }
                            </>
                        }
                    }
                }




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
