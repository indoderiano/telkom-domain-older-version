use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};



use crate::types::{
    users::{ UserDetails},
    ResponseMessage,
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

pub struct UserTabDetails {
    user_details: UserDetails,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    InputText(String, Data),
    GetUserDetails(UserDetails),
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
                ConsoleService::info(&format!("{:?}", data));
                self.fetch_task = None;
                self.user_details = data;
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let UserDetails {
            user_id,
            email,
            email_verified,
            username,
            phone_number,
            phone_verified,
            created_at,
            updated_at,
            identities,
            app_metadata,
            user_metadata,
            picture,
            name,
            nickname,
            multifactor,
            last_ip,
            last_login,
            logins_count,
            blocked,
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
                                    <p class="mb-1">{"yeskahaganta3838@gmail.com"}</p>
                                    <a href="">{"Edit"}</a>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1 ">{"Email"}</p>
                                    <p class="mb-1">{"yeskahaganta3838@gmail.com"}</p>
                                    <p class="text-muted mb-1">{"(verified)"}</p>
                                    <a href="">{"Edit"}</a>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Signed Up"}</p>
                                    <p class="mb-1">{"September 7th 2021, 2:17:53 PM"}</p>
                                </div>
                            </div>
                            <div class="row mt-3">
                                <div class="col-4 col-md-4 col-lg-4">
                                    <p class="text-muted mb-1">{"Primary Identity Provider"}</p>
                                    <p class="mb-1">{"Database"}</p>
                                </div>
                                <div class="col-4 col-md-4 col-lg-4 mb-1">
                                    <p class="text-muted mb-1">{"Latest Login"}</p>
                                    <p class="mb-1">{"Never"}</p>
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
                            value={created_at}
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
                            value={last_login} 
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
                            <button type="button" class="btn btn-danger">{"Delete"}</button>
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
                            <button type="button" class="btn btn-danger">{"Block"}</button>
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
