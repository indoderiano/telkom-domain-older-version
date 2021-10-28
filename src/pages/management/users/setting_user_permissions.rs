use yew::prelude::*;

pub struct UserTabPermissions {}

pub enum Msg {}

impl Component for UserTabPermissions {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        UserTabPermissions {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="mt-4">
                    <div class="row">
                        <div class="col d-flex justify-content-start">
                            <p>{"List of permissions this user has."}</p>
                        </div>
                        <div class="col d-flex justify-content-end">
                            <button type="button" class="btn btn-primary">{"Assign Permissions"}</button>
                        </div>
                    </div>
                </div>
                <div class="mt-4 table-responsive">
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col">{"Name"}</th>
                                <th scope="col">{"Description"}</th>
                                <th scope="col">{"API"}</th>
                                <th scope="col">{"Assignment"}</th>
                                <th scope="col"></th>
                            </tr>
                        </thead>
                        <tbody>
                                <tr>
                                    <th scope="row">{"create:client_grants"}</th>
                                    <td>{"Create New Data"}</td>
                                    <td>{"Example API"}</td>
                                    <td>{"Direct"}</td>
                                    <td>
                                        <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                                                <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                                                <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                                              </svg>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <th scope="row">{"read:client_grants"}</th>
                                    <td>{"Read Data"}</td>
                                    <td>{"Example API"}</td>
                                    <td>{"Direct"}</td>
                                    <td>
                                        <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                                                <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                                                <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                                              </svg>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <th scope="row">{"update:client_grants"}</th>
                                    <td>{"Update New Data"}</td>
                                    <td>{"Example API"}</td>
                                    <td>{"Direct"}</td>
                                    <td>
                                        <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                                                <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                                                <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                                              </svg>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <th scope="row">{"delete:client_grants"}</th>
                                    <td>{"Delete Data"}</td>
                                    <td>{"Example API"}</td>
                                    <td>{"Direct"}</td>
                                    <td>
                                        <button type="button" class="btn btn-outline-secondary px-2 py-1" data-bs-toggle="modal" data-bs-target="#permissionDeleteModal">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-trash" viewBox="0 0 16 16">
                                                <path d="M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5zm3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0V6z"/>
                                                <path fill-rule="evenodd" d="M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1v1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4H4.118zM2.5 3V2h11v1h-11z"/>
                                              </svg>
                                        </button>
                                    </td>
                                </tr>
                            </tbody>
                    </table>


                    <div class="modal fade" id="permissionDeleteModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                        <div class="modal-dialog modal-dialog-centered">
                            <div class="modal-content">
                                <div class="modal-header">
                                    <h5 class="modal-title" id="exampleModalLabel">{"Remove from Role?"}</h5>
                                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                                </div>
                                <div class="modal-body">
                                    {"Are you sure that you want to unassign permission 'create:client_grants'?"}
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
