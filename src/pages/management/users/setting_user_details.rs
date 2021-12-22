use yew::{
    prelude::*,
    format::{Json, Nothing},
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
        storage::{ StorageService, Area }
    },
};
use yew_router::service::RouteService;
use serde::{
    Deserialize,
    Serialize,
};

use crate::types::{
    users::{ UserDetails, ResponseUserDetails},
    ResponseMessage,
    LocalStorage,
    LOCALSTORAGE_KEY,
};

use crate::configs::server::API_URL;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct UsersTabSettingProps {
    pub user_details: UserDetails,
}

pub enum Data{
    UserId,
    Picture,
    CreatedAt,
    UpdatedAt,
    Blocked,
    LastIp,
    LastLogin,
    LoginsCount,
}

pub enum StateError{
    Blocked,
    Delete,
}

pub struct UserTabDetails {
    user_details: UserDetails,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_update_user: bool,
    error_update_user: Option<String>,
    loading_delete_user: bool,
    error_delete_user: Option<String>,
    route_service: RouteService,
}

pub enum Msg {
    InputText(String, Data),
    GetUserDetails(UserDetails),
    ResponseError(String, StateError),
    Delete,
    RedirectToUser,
    Block,
}

impl Component for UserTabDetails {
    type Message = Msg;
    type Properties = UsersTabSettingProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("this s fn create {:?}", props.user_details));
        UserTabDetails {
            user_details: props.user_details,
            link,
            fetch_task: None,
            loading_update_user: false,
            error_update_user: None,
            loading_delete_user: false,
            error_delete_user: None,
            route_service: RouteService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputText(input, data) => {
                match data {
                    Data::UserId => {
                        self.user_details.user_id = input;
                    }
                    Data::Picture => {
                        self.user_details.picture = input;   
                    }
                    Data::CreatedAt => {
                        self.user_details.created_at = input;
                       
                    }
                    Data::UpdatedAt => {
                        self.user_details.updated_at = input;
                    }
                    Data::Blocked => {
                        self.user_details.blocked = !self.user_details.blocked;
                    }
                    Data::LastIp => {
                        self.user_details.last_ip = input;
                    }
                    Data::LastLogin => {
                        self.user_details.last_login = input;
                    }
                    Data::LoginsCount =>{
                       if input.is_empty() {
                           self.user_details.logins_count = 0;
                       } else {
                           self.user_details.logins_count = input.parse::<u32>().unwrap();
                       }
                    }

                }
                true
            }
            Msg::GetUserDetails(data) => {
                ConsoleService::info(&format!("user details = {:?}", data));
                self.fetch_task = None;
                self.user_details = data;
                self.loading_update_user = false;
                self.error_update_user = None;
                true
            },
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::Blocked => {
                        self.fetch_task = None;
                        self.loading_update_user = false;
                        self.error_update_user = Some(message);
                    }
                    StateError::Delete => {
                        self.fetch_task = None;
                        self.loading_delete_user = false;
                        self.error_delete_user = Some(message);
                    }
                }
                true
            }
            Msg::Delete => {
                let request = Request::delete(format!("http://127.0.0.1:8080/api/v2/users/{}", self.user_details.user_id.clone()))
                    .header("access_token", "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhleWthbGxAZ21haWwuY29tIiwiZXhwIjoxNjQzMDk0MTA0fQ.G_kEzjOwrzI_qD8Tco_4HTgXctsz4kUccl4e92WNZb8")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(|response: Response<Json<Result<()
                    // StatusCode
                    , anyhow::Error>>>| {

                    let (meta, Json(data)) = response.into_parts();

                    let status_number = meta.status.as_u16();

                    match status_number {
                        204 => {
                            ConsoleService::info("status code is 204");
                            ConsoleService::info("api is deleted");
                            Msg::RedirectToUser
                        }
                        _ => {
                            ConsoleService::info("status code is not 204");
                            match data {
                                Ok(dataok) => {
                                    ConsoleService::info(&format!("{:?}", dataok));
                                    Msg::RedirectToUser
                                }
                                Err(error) => {
                                    ConsoleService::info(&error.to_string());
                                    Msg::ResponseError(error.to_string(), StateError::Delete)
                                }
                            }
                        }
                    }

                    // let Json(data) = response.into_body();
                    // match data {
                    //     Ok(dataok) => {
                    //         ConsoleService::info(&format!("{:?}", dataok));
                    //         Msg::RedirectToUser
                    //     }
                    //     Err(error) => {
                    //         ConsoleService::info(&error.to_string());
                    //         Msg::ResponseError(error.to_string(), StateError::Delete)
                    //     }
                    // }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_delete_user = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::RedirectToUser => {
                self.loading_delete_user = false;
                self.fetch_task = None;
                self.route_service.set_route(&format!("/{}/users", "tenant_id_not_from_reducer"), ());
                true
            }
            Msg::Block => {
                #[derive(Serialize, Debug, Clone)]
                struct BlockedUser {
                    blocked : bool
                }
                let blocked_user = BlockedUser{
                    blocked : true
                };
                ConsoleService::info(&format!("{:?}", blocked_user.clone()));

                let request = Request::patch(format!("{}/users/dev-ofzd5p1b/users/auth0|7CYXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI", API_URL))
                    .header("Content-Type", "application/json")
                    .header("access_token","tokenidtelkomdomain")
                    .body(Json(&blocked_user))
                    .expect("Could not build request.");
                let callback = self.link.callback(|response: Response<Json<Result<UserDetails, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    match data {
                        Ok(dataok) => {
                            ConsoleService::info(&format!("{:?}", dataok));
                            Msg::GetUserDetails(dataok)
                        }
                        Err(error) => {
                            Msg::ResponseError(error.to_string(), StateError::Blocked)
                        }
                    }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_update_user = true;
                self.fetch_task = Some(task);
                true

            }
            
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let UserDetails {
            user_id,
            email,
            email_verified: _,
            username: _,
            phone_number: _,
            phone_verified: _,
            created_at,
            updated_at,
            identities: _,
            app_metadata: _,
            user_metadata: _,
            picture,
            name:_,
            nickname: _,
            multifactor: _,
            last_ip,
            last_login,
            logins_count,
            blocked: _,
            given_name,
            family_name,
        } = self.user_details.clone();
        html! {
            <>
            <div class="mt-4">
                <div class="card">
                    <div class="card-body">
                        <div class="container">
                            <div class="row">
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Name"}</p>
                                    <p class="mb-1">{given_name}<span>{family_name}</span></p>
                                    <a href="">{"Edit"}</a>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1 ">{"Email"}</p>
                                    <p class="mb-1">{email}</p>
                                    <p class="text-muted mb-1">{"(verified)"}</p>
                                    <a href="">{"Edit"}</a>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Signed Up"}</p>
                                    <p class="mb-1">{created_at.clone()}</p>
                                </div>
                            </div>
                            <div class="row mt-3">
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Primary Identity Provider"}</p>
                                    <p class="mb-1">{"Database"}</p>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4 mb-1">
                                    <p class="text-muted mb-1">{"Latest Login"}</p>
                                    <p class="mb-1">{last_login.clone()}</p>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Accounts Associated"}</p>
                                    <p>{"None"}</p>
                                </div>
                            </div>
                            <div class="row mt-3">
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Browser"}</p>
                                    <p class="mb-1">{"Chrome 91.0.4472/ Linux 0.0.0"}</p>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                </div>
                            </div>


                        </div>
                    </div>
                </div>
        </div>
        
        <div class="mt-4">
            <div class="card">
                <div class="card-body">
                    <p class="fw-bold">{"Multi-Factor Authentication"}</p>
                    <div class="p-4" style="background-color: rgb(239,240,242)">
                        <p class="text-center mb-0">{"MFA is enabled for this user. "} <a href="">{"Send and enrollment invitation."}</a></p>
                    </div>
                </div>
            </div>
        </div>

        <div class="mt-4">
            <div class="card">
                <div class="card-body p-4">
                    <p class="fw-bold fs-4">{"Identity Provider Attributes"}</p>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"user_id"}</label>
                        <input 
                            type="text" 
                            class="form-control" 
                            aria-label="readonly input example"
                            value={user_id} 
                            readonly=true
                            oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::UserId))
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"picture"}</label>
                        <input 
                            type="text" 
                            class="form-control"
                            aria-label="readonly input example" 
                            value={picture} 
                            readonly=true
                            oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::Picture))
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"created_at"}</label>
                        <input 
                            type="text" 
                            class="form-control"
                            aria-label="readonly input example" 
                            value={created_at.clone()}
                            readonly=true
                            oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::CreatedAt))
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"updated_at"}</label>
                        <input 
                            type="text" 
                            class="form-control" 
                            aria-label="readonly input example"
                            value={updated_at} 
                            readonly=true
                            oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::UpdatedAt))
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"blocked_for"}</label>
                        <input class="form-control" type="text" value="[]" aria-label="readonly input example" readonly=true/>
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"blocked"}</label>
                        <input 
                            type="text" 
                            class="form-control" 
                            value="false" 
                            aria-label="readonly input example" 
                            readonly=true
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"last_password_reset"}</label>
                        <input 
                            class="form-control" 
                            type="text" 
                            value="2021-10-09T04:43:28.300Z" 
                            aria-label="readonly input example" 
                            readonly=true
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"last_ip"}</label>
                        <input 
                            type="text" 
                            class="form-control"
                            value={last_ip} 
                            aria-label="readonly input example" 
                            readonly=true
                            oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::LastIp))
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"last_logins"}</label>
                        <input 
                            type="text" 
                            class="form-control" 
                            value={last_login.clone()} 
                            aria-label="readonly input example" 
                            readonly=true
                            oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::LastLogin))
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label fw-bold">{"logins_count"}</label>
                        <input 
                            type="text" 
                            class="form-control" 
                            value={logins_count.to_string()} 
                            aria-label="readonly input example" 
                            readonly=true
                            oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::LoginsCount))
                        />
                    </div>
                </div>
            </div>


            <div class="mt-4">
                <div class="alert alert-danger" role="alert">
                    <div class="row">
                        <div class="col-10 col-sm-10">
                            <p class="p-0 m-0 fw-bold">{"Delete user"}</p>
                            <p class="p-0 m-0">{"The user will be removed and it will no longer have access to your applications"}</p>
                        </div>
                        <div class="col-2 col-sm-2 p-0 d-flex align-items-center justify-content-center">
                            
                            <button
                                type="button"
                                class=format!("btn {} btn-danger position-relative", if self.loading_delete_user {"loading"} else {""} )
                                onclick=self.link.callback(|_|Msg::Delete)
                                disabled={ self.loading_delete_user }
                            >
                                <div class="telkom-label">
                                    {"Delete"}
                                </div>
                                <div class="telkom-spinner telkom-center">
                                    <div class="spinner-border spinner-border-sm" role="status"/>
                                </div>
                            </button>
                            {
                                if self.error_delete_user.is_some() {
                                    html! {
                                        <div class="alert alert-warning" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_delete_user.clone().unwrap() }
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>


            <div class="mt-4">
                <div class="alert alert-danger" role="alert">
                    <div class="row">
                        <div class="col-10 col-sm-10">
                            <p class="p-0 m-0 fw-bold">{"Block user"}</p>
                            <p class="p-0 m-0">{"The user will be blocked for logging into your applications."}</p>
                        </div>
                        <div class="col-2 col-sm-2 p-0 d-flex align-items-center justify-content-center">
                            
                            <button
                                type="button"
                                class=format!("btn {} btn-danger position-relative", if self.loading_update_user {"loading"} else {""} )
                                onclick=self.link.callback(|_| Msg::Block)
                                disabled={ self.loading_update_user }
                            >
                                <div class="telkom-label">
                                    {"Block"}
                                </div>
                                <div class="telkom-spinner telkom-center">
                                    <div class="spinner-border spinner-border-sm" role="status"/>
                                </div>
                            </button>
                                {
                                    if self.error_update_user.is_some() {
                                    html! {
                                        <div class="alert alert-warning mt-3" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_update_user.clone().unwrap() }
                                        </div>
                                    }
                                    } else {
                                        html! {}
                                    }
                                }
                        </div>
                    </div>
                </div>
            </div>


            <div class="mt-4">
                <div class="alert alert-danger" role="alert">
                    <div class="row">
                        <div class="col-10 col-sm-10">
                            <p class="p-0 m-0 fw-bold">{"Change Password"}</p>
                            <p class="p-0 m-0">{"Once you change it, the user will not be able to log in using their previous password."}</p>
                        </div>
                        <div class="col-2 col-sm-2 p-0 d-flex align-items-center justify-content-center">
                            <button type="button" class="btn btn-danger">{"Change"}</button>
                        </div>
                    </div>
                </div>
            </div>




        </div>
            </>
        }
    }


}
