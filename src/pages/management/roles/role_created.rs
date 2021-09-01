use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;

pub struct RolesCreated {}

pub enum Msg {}

impl Component for RolesCreated {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        RolesCreated {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <>
            <div class="mx-auto pt-5 pb-5 px-4" style="max-width: 1048px;">

                <div class="row">
                    <div class="col-9">
                        <h1 class="fw-bold">{"Roles"}</h1>
                    </div>
                    <div class="col-3">
                        <button type="button" data-bs-toggle="modal" data-bs-target="#addRoleModal" class="btn btn-primary text-center">
                            <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                            <span>{"Create Role"}</span>
                         </button>
                    </div>
                </div>
        
                <div class="mt-3">
                    <p class="text-muted fs-6">{"Create and manage Roles for your applications. Roles contain collections of Permissions and can be assigned to Users."} </p>
                </div>
        
                <div class="mt-5">
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col">{"Name"}</th>
                                <th scope="col">{"Description"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><a href="" class="text-decoration-none">{"Yeska"}</a></td>
                                <td>
                                    <div class="row">
                                        <div class="col-10">
                                            <p>{"Frontend Developer"}</p>
                                        </div>
                                        <div class="col-2">
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
                                                    <Anchor route=AppRoute::ViewDetail classes="dropdown-item fs-7">
                                                        {"View Details"}
                                                    </Anchor>
                                                </li>
                                                <li>
                                                    <Anchor route=AppRoute::ApisSettings classes="dropdown-item fs-7">
                                                        {"Assign To Users"}
                                                    </Anchor>
                                                </li>
                                                <li>
                                                    <Anchor route=AppRoute::ApisSettings classes="dropdown-item fs-7">
                                                        {"Delete Role"}
                                                    </Anchor>
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
    
            </div>


            <div class="modal fade" id="addRoleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                <div class="modal-dialog modal-dialog-scrollable" role="document">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="exampleModalLabel">{"New Role"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close">
                            </button>
                        </div>
                        <div class="modal-body">
                            <div class="form-group">
                                <label for="exampleInputEmail1">{"Name "} <span style="color:red">{"*"}</span></label>
                                <input type="text" class="form-control" id="exampleInputEmail1" />
                            </div>
                            <div class="form-group mt-3 mb-3">
                                <label for="exampleInputPassword1">{"Description "}<span style="color:red">{"*"}</span></label>
                                <input type="text" class="form-control" id="exampleInputPassword1" />
                            </div>
        
        
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"} </button>
                            <button type="button" class="btn btn-primary">
                                {"Create"}
                            </button>
                        </div>
        
        
                    </div>
                </div>
            </div>


            </>
        }
    }
}
