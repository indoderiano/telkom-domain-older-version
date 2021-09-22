use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::components::RouterAnchor;
use yew::services::ConsoleService;
use crate::app::AppRoute;
<<<<<<< HEAD
use crate::types::api::{ ApiTitle, ResponseApiList, ApiCreate };
use crate::components::loading::Loading;
=======
use crate::types::api::ApiTitle;
use crate::components::{
    loading::Loading,
    loading2::Loading2,
};
>>>>>>> 8e7c514651531a1365ac6c75138f54f7b580bddf


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ApisProps {
    pub tenant_id: String,
}

pub enum DataApiCreate {
    Name,
    Identifier,
    SignAlg,
}

pub struct ApisHome {
    tenant_id: String,
    fetch_task: Option<FetchTask>,
    error: Option<String>,
    link: ComponentLink<Self>,
    api_list: Vec<ApiTitle>,
    api_create: ApiCreate,
}

pub enum Msg {
    RequestApiList,
    GetApiList(Result<ResponseApiList, anyhow::Error>),
    Input(String, DataApiCreate),
    Create,
}

impl ApisHome {
    fn view_api_list (&self) -> Vec<Html> {
        type Anchor = RouterAnchor<AppRoute>;
        let tenant_id = &self.tenant_id;
        self.api_list.iter().map(|api| {
            html! {
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
                                <p
                                    class="m-0"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                        text-decoration: none;
                                    "
                                >
                                    <Anchor
                                        route=AppRoute::ApisSettings { tenant_id: tenant_id.clone(), api_id: api.id.clone() }
                                        classes="text-decoration-none fw-bold mb-0"
                                    >
                                            // {"Auth0 Management API"}
                                            { &api.name }
                                    </Anchor>
                                </p>
                                <p
                                    class="mb-0 text-muted"
                                    style="
                                        white-space: nowrap;
                                        text-overflow: ellipsis;
                                        overflow: hidden;
                                        font-size: 14px;
                                    "
                                >
                                    // {"System API"}
                                    { &api.api_type }
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
                                // {"https://dev-r5y8heyf.au.auth0.com/api/v2/"}
                                { &api.identifier }
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
                                    <Anchor route=AppRoute::ApisSettings { tenant_id: tenant_id.clone(), api_id: api.id.clone() } classes="dropdown-item fs-7">
                                        {"Settings"}
                                    </Anchor>
                                </li>
                            </ul>
                        </div>

                    </div>
                </div>
            }
        })
        .collect()
    }
}

impl Component for ApisHome {
    type Message = Msg;
    type Properties = ApisProps;  

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Apis home props, tenant id = {}", props.tenant_id));

        let api_create = ApiCreate {
            name: String::from(""),
            identifier: String::from(""),
            sign_algorithm: String::from(""),
        };

        ApisHome {
            tenant_id: props.tenant_id,
            fetch_task: None,
            error: None,
            link,
            api_list: Vec::new(),
            api_create,
        }
    }

    fn rendered(&mut self, first_render: bool) {

        if first_render {
            ConsoleService::info("This is first render");
            
            self.link.send_message(Msg::RequestApiList);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestApiList => {
                let request = Request::get("http://localhost:3000/api/tenantid")
                    // .header("Content-Type", "application/json")
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<ResponseApiList, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::GetApiList(data)
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetApiList(response) => {
                match response {
                    Ok(data) => {
                        ConsoleService::info(&format!("{:?}", data));
                        self.api_list = data.data;
                    }
                    Err(error) => {
                        ConsoleService::info(&error.to_string());
                    }
                }
                self.fetch_task = None;
                true
            }
            Msg::Input(input, data) => {
                match data {
                    DataApiCreate::Name => {
                        self.api_create.name = input;
                    }
                    DataApiCreate::Identifier => {
                        self.api_create.identifier = input;
                    }
                    DataApiCreate::SignAlg => {
                        self.api_create.sign_algorithm = input;
                    }
                }
                true
            }
            Msg::Create => {
                ConsoleService::info(&format!("{:?}", self.api_create));
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // type Anchor = RouterAnchor<AppRoute>;
        // ConsoleService::info(&format!("fetch task: {:?}", &self.fetch_task));
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
                        <div>
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


                {
                    if self.fetch_task.is_some() {
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
                    } else {
                        html! {
                            <>
                                { self.view_api_list() }
                            </>
                        }
                    }
                }



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
                                        <input
                                            type="text"
                                            class="form-control"
                                            id="basic-url"
                                            aria-describedby="basic-addon3"
                                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, DataApiCreate::Name))
                                        />
                                    </div>
                                    <label class="form-label text-muted">{"A friendly name for the API"}</label>
                                </div>
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Identifier"}</label>
                                    <div class="input-group mb-2">
                                        <input
                                            type="text"
                                            class="form-control"
                                            id="basic-url"
                                            aria-describedby="basic-addon3"
                                            oninput=self.link.callback(|data: InputData| Msg::Input(data.value, DataApiCreate::Identifier))
                                        />
                                    </div>
                                    <label class="form-label text-muted">{"A logical identifier for this API. We recommend using a URL but note that this doesn’t have to be a publicly available URL, Auth0 will not call your API at all.This field cannot be modified."}</label>
                                </div>
                                <div
                                    class="mb-4"
                                >
                                    <label for="basic-url" class="form-label fw-bold">{"Signing Algorithm"}</label>
                                    <select
                                        class="form-select mb-2"
                                        aria-label="Default select example"
                                        onchange=self.link.callback(|e| {
                                            if let ChangeData::Select(select) = e {
                                                let value = select.value();
                                                Msg::Input(value, DataApiCreate::SignAlg)
                                            } else {
                                                Msg::Input(String::from("no value"), DataApiCreate::SignAlg)
                                            }
                                        })
                                    >
                                        <option value="RS256">{"RS256"}</option>
                                        <option value="HS256">{"HS256"}</option>
                                    </select>
                                    <label class="form-label text-muted">{"Algorithm to sign the tokens with. When selecting RS256 the token will be signed with Auth0's private key."}</label>
                                </div>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                                <button
                                    type="button"
                                    class="btn btn-primary"
                                    onclick=self.link.callback(|_| Msg::Create)
                                >
                                    {"Create"}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
                
            </div>
        }
    }
}
