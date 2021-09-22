use crate::app::AppRoute;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct UsersManagement {
    learn_more: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    LearnMore,
    HideDetails
}

impl Component for UsersManagement {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        UsersManagement { 
            learn_more: false, 
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LearnMore => {
                self.learn_more = true;
                true
            }
            Msg::HideDetails => {
                self.learn_more = false;
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
            <div>

                <div class="container mx-auto pt-5 pb-5 px-4" style="max-width: 1048px;">

                    <div class=" d-flex row align-center">
                        <div class="col">
                            <p class="fs-2 fw-bold">{"Users"}</p>
                        </div>
                        <div class="col d-flex justify-content-end">
                            <button type="button" class="btn btn-primary ms-3 mt-3 mb-3" data-bs-toggle="modal" data-bs-target="#createNewUser">
                                <i class="bi bi-plus"></i>
                                <span>{"Create User"}</span>
                            </button>
                        </div>
                    </div>

                    <div class="mt-3">
                        <p>
                            {"An easy to use UI to help administrators manage user identities including password resets, creating, and provisioning, blocking, and deleting users. "}
                            {
                                if self.learn_more == true {
                                    html!{
                                        <a 
                                            href="javascript: void(0);" 
                                            class="text-decoration-none" 
                                            onclick=self.link.callback(|_| Msg::HideDetails)
                                        >
                                            <span
                                                style="
                                                    white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 14px;
                                                    text-decoration: none;
                                                "
                                            >
                                                {"Hide details"}
                                                <i
                                                class="bi bi-arrow-right-short fs-5"
                                                style="vertical-align: -3px; margin-left: -2px;"></i>
                                            </span>
                                        </a>
                                    }
                                } else {
                                    html! {
                                        <a
                                            href="javascript: void(0)"
                                            class="text-decoration-none"
                                            onclick=self.link.callback(|_| Msg::LearnMore)
                                        >
                                            <span
                                                style="
                                                    white-space: nowrap;
                                                    text-overflow: ellipsis;
                                                    overflow: hidden;
                                                    font-size: 14px;
                                                    text-decoration: none;
                                                "
                                            >
                                                {"Learn more"}
                                                <i
                                                    class="bi bi-arrow-right-short fs-5"
                                                    style="vertical-align: -3px; margin-left: -2px;"></i>
                                            </span>
                                        </a>
                                    }
                                }
                            }
                        </p>
                        {
                            if self.learn_more == true {
                                html! {
                                    <div
                                        class="alert alert-secondary"
                                        role="alert"
                                        style="font-size: 13px;"
                                    >
                                        <div
                                            class="fw-bold mb-3 pb-2"
                                            style="
                                                font-size: 13px;
                                                text-transform: uppercase;
                                                letter-spacing: 1px;
                                                border-bottom: 1px solid rgb(200, 200, 200);
                                            "
                                        >
                                            {"With users you can"}
                                        </div>
                                        <div
                                            class="d-inline-flex flex-row w-50"
                                        >
                                            <i class="bi bi-info-circle-fill me-4"></i>
                                            <p
                                                class="pe-5"
                                            >
                                                {"Manage user identities including password resets, creating and provisioning, blocking and deleting users."}
                                            </p>
                                        </div>
                                        <div
                                            class="d-inline-flex flex-row"
                                            style="width: 49%;"
                                        >
                                            <i class="bi bi-info-circle-fill me-4"></i>
                                            <p
                                                class="pe-5"
                                            >
                                                {"Store arbitrary JSON objects attached to an Auth0 user."}
                                            </p>
                                        </div>
                                        <div
                                            class="d-inline-flex flex-row w-50"
                                        >
                                            <i class="bi bi-info-circle-fill me-4"></i>
                                            <p
                                                class="pe-5"
                                            >
                                                {"Associate user accounts with multiple connections such as database, enterprise or social with the same user on Auth0, allowing that user to authenticate with any of them."}
                                            </p>
                                        </div>
                                        <div
                                            class="d-inline-flex flex-row"
                                            style="width: 49%;"
                                        >
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>

                    <div class="mt-5">
                        <div class="row">
                            <div class="col-md col-lg">
                                <div class="input-group flex-nowrap">
                                    <span class="input-group-text" id="addon-wrapping"><i class="bi bi-search"></i></span>
                                    <input type="text" class="form-control" placeholder="Search for users" aria-label="Username" aria-describedby="addon-wrapping" />
                                </div>
                            </div>
                            <div class="col-md-auto col-lg-auto">
                                <div class="input-group mb-3">
                                    <label class="input-group-text" for="inputGroupSelect01">{"Search by"}</label>
                                    <select class="form-select" id="inputGroupSelect01">
                                    <option selected=true>{"User"}</option>
                                    <option value="1">{"Email"}</option>
                                    <option value="2">{"Identity Provider"}</option>
                                    <option value="3">{"Connection"}</option>
                                    <option value="3">{"Connection"}</option>
                                    <option value="3">{"Login Count"}</option>
                                    <option value="3">{"Last Login"}</option>
                                    <option value="3">{"Phone Number"}</option>
                                    <option value="lucene_syntax">{"Lucene Syntax ()"}</option>
                                    </select>
                                </div>
                            </div>
                            <div class="col-md-auto col-lg-auto">
                                <button type="button" class="btn btn-outline-secondary">
                                    <i class="bi bi-x"></i>
                                    <span>{"Reset"}</span>
                                </button>
                            </div>
                        </div>
                    </div>

                    <div class="mt-2 table-responsive-md table-responsive-lg">
                        <table class="table">
                            <thead>
                                <tr>
                                    <th scope="col">{"Name"}</th>
                                    <th scope="col-auto">{"Connection"}</th>
                                    <th scope="col-auto">{"Logins"}</th>
                                    <th scope="col-auto">{"Latest Login"}</th>
                                    <th></th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <th scope="row">
                                        <div>
                                            <a href="" class="text-decoration-none">{"yeskahaganta3838@gmail.com"}</a>
                                            <p class="text-muted overflow-hidden">{"yeskahaganta3838@gmail.com"}</p>
                                        </div>
                                    </th>
                                    <td>{"User Database"}</td>
                                    <td>{"0"}</td>
                                    <td>{"never"}</td>
                                    <td>
                                        <button type="button" style="flex: 0 0 auto; width: 30px; height: 30px;" class="btn d-flex justify-content-center align-items-center rounded border" role="button" id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false">
                                            <i class="bi bi-three-dots"></i>
                                        </button>
                                        <ul class="dropdown-menu pt-1" aria-labelledby="dropdownMenuButton1">
                                            <li class="p-1 text-muted" style="font-size:13px;">
                                                <Anchor route=AppRoute::UserViewDetail classes="dropdown-item">
                                                    {"View Details"}
                                                </Anchor>
                                            </li>
                                            <li>
                                                <hr class="dropdown-divider"/>
                                            </li>
                                            <li class="p-1 text-muted">
                                                        <div class="ms-1 d-flex flex-row inline-block align-items-center" style="font-size:13px;" >
                                                            <i class="bi bi-person-check"></i>
                                                            <span data-bs-toggle="modal" data-bs-target="#assignRoles">
                                                            <a class="dropdown-item" href="#">
                                                                {"Assign Roles"}
                                                            </a>
                                                        </span>
                                                        </div>
                                            </li>
                                            <li class="p-1 text-muted" style="font-size:13px;">
                                                        <div class="ms-1 d-flex flex-row inline-block align-items-center">
                                                            <i class="bi bi-check2-square"></i>
                                                            <span data-bs-toggle="modal" data-bs-target="#assignPermissions">
                                                                <a class="dropdown-item" href="#" >
                                                                    {"Assign Permissions"}
                                                                </a>
                                                            </span>
                                                        </div>
                                            </li>
                                            <li class="p-1 text-muted" style="font-size:13px;">
                                                <div class="ms-1 d-flex flex-row inline-block align-items-center">
                                                    <i class="bi bi-envelope "></i>
                                                    <span  data-bs-toggle="modal" data-bs-target="#resendConfirmation">
                                                        <a class="dropdown-item" href="#">
                                                            {"Send Verification Email "}
                                                        </a>
                                                    </span>
                                                </div>
                                            </li>
                                            <li>
                                                <hr class="dropdown-divider"/>
                                            </li>
                                            <li class="p-1 text-muted" style="font-size:13px;" data-bs-toggle="modal" data-bs-target="#changeEmail">
                                                <a class="dropdown-item" href="#" >
                                                    {"Change Email "}
                                                </a>
                                            </li>
                                            <li class="p-1 text-muted" style="font-size:13px;" data-bs-toggle="modal" data-bs-target="#changePassword">
                                                <a class="dropdown-item" href="#">
                                                    {"Change Password "}
                                                </a>
                                            </li>
                                            <li>
                                                <hr class="dropdown-divider" />
                                            </li>
                                            <li class="p-1" style="font-size:13px;">
                                                <div class="ms-1 d-flex flex-row text-muted inline-block align-items-center">
                                                    <svg xmlns="http://www.w3.org/2000/svg " width="13" height="13" viewBox="0 0 24 24 " fill="none " stroke="currentColor " stroke-width="2 " stroke-linecap="round " stroke-linejoin="round"><circle cx="12 " cy="12 " r="10 "></circle><line x1="4.93 " y1="4.93 " x2="19.07 " y2="19.07 "></line></svg>
                                                    <span>
                                                        <a class="dropdown-item" href="#">
                                                            {"Block "}
                                                        </a>
                                                    </span>
                                                </div>
                                            </li>
                                            <li class="p-1 text-danger " style="font-size:13px;">
                                                <div class="ms-1 d-flex flex-row">
                                                    <i class="bi bi-trash "></i>
                                                    <span data-bs-toggle="modal" data-bs-target="#deleteUsers">
                                                        <a class="dropdown-item fs-7" href="#">
                                                            {"Delete "}
                                                        </a>
                                                    </span>
                                                </div>
                                            </li>
                                        </ul>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>

                </div>



        <div class="modal fade" id="createNewUser" tabindex="-1" aria-labelledby="createNewUserLabel" aria-hidden="true">
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="createNewUserLabel">{"Create User"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <form>
                            <div class="mb-3">
                                <label for="recipient-name" class="col-form-label">{"Email"} <span class="text-danger">{"*"}</span></label>
                                <input type="text" class="form-control" id="recipient-name" />
                            </div>
                            <div class="mb-3">
                                <label for="recipient-name" class="col-form-label">{"Password"} <span class="text-danger">{"*"}</span> </label>
                                <input type="password" class="form-control" id="recipient-name" />
                            </div>
                            <div class="mb-3">
                                <label for="recipient-name" class="col-form-label">{"Repeat Password"} <span class="text-danger">{"*"}</span></label>
                                <input type="password" class="form-control" id="recipient-name" />
                            </div>
                            <div class="mb-3">
                                <label for="recipient-name" class="col-form-label">{"Connection"} <span class="text-danger">{"*"}</span></label>
                                <select class="form-select" aria-label="Default select example">
                                    <option selected=true>{"User Database"}</option>
                                  </select>
                            </div>

                        </form>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                        <button type="button" class="btn btn-primary">{"Create"}</button>
                    </div>
                </div>
            </div>
        </div>

        <div class="modal fade" id="assignRoles" tabindex="-1" role="dialog" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered" role="document" style="max-width: 640px;">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Assign Roles"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="mt-2">
                            <p>{"Select roles to assign to this user. You may assign up to 50 roles per user."}</p>
                        </div>

                        <div class="mt-2">
                            <select class="form-select" aria-label="Default select example">
                                <option selected=true>
                                    <p>{"Brother Yeska"}</p>
                                    <span>{" - "}</span>
                                    <p style="color: darkgray">{"Manager"}</p>
                                </option>
                            </select>
                        </div>

                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-primary">{"Assign"}</button>
                    </div>
                </div>
            </div>
        </div>

        <div class="modal fade" id="assignPermissions" tabindex="-1" role="dialog" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered" role="document" style="max-width: 640px;">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Assign Roles"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="mt-2">
                            <p>{"Select permissions from existing APIs."}</p>
                        </div>

                        <div class="mt-2">
                            <select class="form-select" aria-label="Default select example">
                                <option value="">
                                    <div>
                                        <div class="card">
                                        <div class="card-body">
                                            <p>
                                                {"Placeholder"}
                                            </p>
                                            <span>{" - "}</span>
                                            <p style="color:darkgrey;">{"https://https://jsonplaceholder.typicode.com/posts"}</p>
                                        </div>
                                    </div>
                                    </div>
                                </option>
                            </select>
                        </div>

                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-primary">{"Assign"}</button>
                    </div>
                </div>
            </div>
        </div>

        <div class="modal fade" id="resendConfirmation" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Resend confirmation email "}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="mt-2">
                            <p>
                                {"Do you want to send a confirmation e-mail to yeskahaganta3838@gmail.com?"}
                            </p>
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                        <button type="button" class="btn btn-danger">{"Confirm"}</button>
                    </div>
                </div>
            </div>
        </div>

        <div class="modal fade" id="changeEmail" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Edit E-mail"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <form action="">
                        <div class="modal-body">
                            <div class="mt-1 mb-3">
                                <label for="updateUserEmail" class="form-label">{"Email"}</label>
                                <input type="email" class="form-control" id="updateUserEmail" aria-describedby="updateUserEmail" value="yeskahaganta3838@gmail.com" />
                                <div id="emailHelp" class="form-text">{"Verified"}</div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                            <button type="button" class="btn btn-primary">{"Save"}</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>

        <div class="modal fade" id="changePassword" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Change Password"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <form action="">
                        <div class="modal-body">
                            <div class="mb-3">
                                <label for="nextPass" class="form-label">{"Password"} <span class="text-danger">{"*"}</span></label>
                                <input type="password" class="form-control" id="nextPass" />
                            </div>
                            <div class="mb-3">
                                <label for="repeatNextPass" class="form-label">{"Next Password"} <span class="text-danger">{"*"}</span></label>
                                <input type="password" class="form-control" id="repeatNextPass" />
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                            <button type="button" class="btn btn-primary">{"Save"}</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>

        <div class="modal fade" id="deleteUsers" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Delete user"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="mt-2 mb-2">
                            <p>
                                {"Are you really sure you want to delete yeskahaganta3838@gmail.com? This cannot be undone!"}
                            </p>
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"}</button>
                        <button type="button" class="btn btn-danger">{"Yes, Delete it"}</button>
                    </div>
                </div>
            </div>
        </div>





            </div>
        }
    }
}
