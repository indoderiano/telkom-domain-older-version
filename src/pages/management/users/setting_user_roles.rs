use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew::services::ConsoleService;
use crate::configs::server::API_URL;
use crate::types::users::{UserRoles};
use crate::components::loading2::Loading2;

pub struct UserTabRoles {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    user_roles: UserRoles,
}

pub enum Msg {
    RequestUserRoles,
    GetUserRoles(Result<UserRoles, anyhow::Error>),
}

impl Component for UserTabRoles {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        
        let user_roles= UserRoles::new();

        UserTabRoles {
            link,
            fetch_task: None,
            user_roles
        }
    }

    fn rendered(&mut self, first_render: bool){
        if first_render {
            self.link.send_message(Msg::RequestUserRoles)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::RequestUserRoles => {
                let request = Request::get(format!("{}/users/tenant_id/users/:id/roles", API_URL))
                    .header("access_token", "telkomidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<UserRoles, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::GetUserRoles(data)
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::GetUserRoles(response) => {
                ConsoleService::info(&format!(" ===> {:?}", response));
                match response {
                    Ok(data) => {
                        ConsoleService::info(&format!(" Ini di get user roles ==> {:?}", data));
                        self.user_roles = data;
                    }
                    Err(error) => {
                        ConsoleService::info(&error.to_string());
                    }
                }
                self.fetch_task = None;
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
                    {
                        if self.fetch_task.is_some() {
                            html! {
                                <div style="position: relative; margin-top:8rem;">
                                    <Loading2 width = 45 />
                                </div>
                            }
                        } else {
                            html! {
                        
                                {self.view_content()}
                                
                            }
                        }
                    }
                </table>
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
                            <button type="button" class="btn btn-danger">{"Yes, remove"}</button>
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
            <>
                <tbody>
                <tr>
                    <th scope="row" class="align-middle">
                        <a href="">{name}</a>
                    </th>
                    <td class="align-middle">{description}</td>
                    <td class="align-middle">{"Direct"}</td>
                    <td class="text-end">
                        <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#exampleModal">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                                <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                                <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                            </svg>
                        </button>
                    </td>
                </tr>
                </tbody>
            </>
        }
    }

}
