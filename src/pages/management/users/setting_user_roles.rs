use crate::components::loading2::Loading2;
use crate::configs::server::API_URL;
use crate::types::{
    users::{UserRoles},
    ResponseMessage,
};
use yew::services::ConsoleService;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};  
use yew_router::service::RouteService;
pub struct UserTabRoles {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    user_roles: UserRoles,
    loading_get_user_roles: bool,
    error_get_user_roles: Option<String>,
    loading_delete_roles: bool,
    error_delete_roles: Option<String>,
    route_service: RouteService,
}

pub enum Msg {
    RequestUserRoles,
    GetUserRoles(UserRoles),
    Delete,
    ResponseError(String, StateError),
    RedirectToRoles,
}

pub enum StateError {
    GetUserRoles,
    Delete,
}

impl Component for UserTabRoles {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let user_roles = UserRoles::new();

        UserTabRoles {
            link,
            fetch_task: None,
            user_roles,
            loading_get_user_roles: false,
            error_get_user_roles: None,
            loading_delete_roles: false,
            error_delete_roles: None,
            route_service: RouteService::new(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::RequestUserRoles)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestUserRoles => {
                let request = Request::get(format!("{}/users/tenant_id/users/:id/roles", API_URL))
                    .header("access_token", "telkomidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<UserRoles, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(dataok) => Msg::GetUserRoles(dataok),
                            Err(error) => {
                                Msg::ResponseError(error.to_string(), StateError::GetUserRoles)
                            }
                        }
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.loading_get_user_roles = true;
                true
            }
            Msg::GetUserRoles(data) => {
                self.user_roles = data;
                self.fetch_task = None;
                self.loading_get_user_roles = false;
                true
            }
            Msg::Delete => {
                let request = Request::delete(format!("{}/users/tenant_id/users/auth0|7CYXV0aDAlN0M2MTM3MTIyMTAxY2VmYTAwNzM0NzRmYmI/roles", API_URL))
                    .header("access_token", "telkomidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self.link.callback(|response: Response<Json<Result<ResponseMessage, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    match data {
                        Ok(dataok) => {
                            ConsoleService::info(&format!("{:?}", dataok));
                            Msg::RedirectToRoles
                        }
                        Err(error) => {
                            ConsoleService::info(&error.to_string());
                            Msg::ResponseError(error.to_string(), StateError::Delete)
                        }
                    }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_delete_roles = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::GetUserRoles => {
                        self.loading_get_user_roles = false;
                        self.error_get_user_roles = Some(message);
                    }
                    StateError::Delete => {
                        self.loading_delete_roles = false;
                        self.error_delete_roles = Some(message);
                    }
                }
                self.fetch_task = None;
                true
            }
            Msg::RedirectToRoles => {
                self.loading_delete_roles = false;
                self.fetch_task = None;
                self.route_service.set_route(&format!("/{}/roles", "tenant_id_not_from_reducer"), ());
                true
            }
            
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="mt-4">

                <div class="row">
                    <div class="col text-center d-flex justify-content-start m-0">
                        <p>{"All Roles assigned to this User."}</p>
                    </div>
                    <div class="col d-flex justify-content-end">
                        <button type="button" class="btn btn-primary">{"Assign Roles"}</button>
                    </div>
                </div>

            <div class="mt-4 table-responsive">
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">{"Name"}</th>
                            <th scope="col">{"Description"}</th>
                            <th scope="col">{"Assignment"}</th>
                            <th scope="col"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            if !self.loading_get_user_roles && !self.error_get_user_roles.is_some() {
                                html! { 
                                    { self.view_content() }
                                }
                            } else {
                                html! {}
                            }
                        }
                    </tbody>
                </table>
                {
                    if self.loading_get_user_roles {
                        html! {
                            <div style="position: relative; margin-top:4rem;">
                                <Loading2 width = 45 />
                            </div>
                        }
                    } else if self.error_get_user_roles.is_some() {
                        html! {
                            <div class="alert alert-warning mb-5" role="alert">
                                <i class="bi bi-exclamation-triangle me-2"></i>
                                { self.error_get_user_roles.clone().unwrap() }
                            </div>
                        }
                    } else {
                        html! { }
                    }
                }
            </div>

            <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                <div class="modal-dialog modal-dialog-centered">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="exampleModalLabel">{"Remove from Role?"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            {"Are you sure that you want to remove Yeska Haganta from role 'admin'?"}
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-outline-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                            <button 
                                type="button" 
                                class="btn btn-danger"
                                onclick=self.link.callback(|_|Msg::Delete)
                                >{"Yes, remove"}</button>
                                {
                                    if self.error_delete_roles.is_some() {
                                        html! {
                                            <div class="alert alert-warning" role="alert">
                                                <i class="bi bi-exclamation-triangle me-2"></i>
                                                { self.error_delete_roles.clone().unwrap() }
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



        </div>
            </>
        }
    }

}

impl UserTabRoles {
    fn view_content(&self) -> Html {
        let UserRoles {
            id: _,
            name,
            description,
        } = self.user_roles.clone();

        html! {
            <tr>
                <th scope="row" class="align-middle">
                    <a href="">{name}</a>
                </th>
                <td class="align-middle">{description}</td>
                <td class="align-middle">{"Direct"}</td>
                <td class="text-end">
                    <button 
                    type="button" 
                    class="btn btn-outline-secondary px-2 py-1" 
                    data-bs-toggle="modal" 
                    data-bs-target="#exampleModal">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                            <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                            <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                        </svg>
                    </button>
                </td>
            </tr>
        }
    }
}
