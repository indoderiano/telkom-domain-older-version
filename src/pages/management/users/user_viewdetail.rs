use yew::prelude::*;

pub struct UserViewDetail {}

pub enum Msg {}

impl Component for UserViewDetail {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        UserViewDetail {}
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
                <div class="mx-auto pt-5 pb-5 px-4" style="max-width: 1048px">
                    <div>
                        <a href="" class="text-muted">
                            <i class="bi bi-arrow-left me-2"></i>
                            <span>{"Back to users"}</span>
                        </a>
                    </div>
            
                    <div class="mt-2">
                        <div class="row">
                            <div class="col">
                                <p class="mb-0" style="font-size: 32px; font-weight: bold">
                                    {"yeskahaganta3838@gmail.com"}
                                </p>
                                <p class="text-muted">
                                    {"user_id : "}
                                    <span>
                            <code style="background-color: beige; color: black"
                            >{"auth0|6137122101cefa0073474fbb"}</code
                            >
                        </span>
                                </p>
                            </div>
                            <div class="col-auto">
                                <div class="dropdown">
                                    <a class="btn btn-primary dropdown-toggle mt-3" href="#" role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                            {"Actions"}
                        </a>
            
                                    <ul class="dropdown-menu" aria-labelledby="dropdownMenuLink">
                                        <li>
                                            <a class="dropdown-item" href="#"><i class="bi bi-envelope me-2"></i
                                ><span>{"Send Verification Email"}</span></a
                            >
                            </li>
                            <li>
                            <hr class="dropdown-divider" />
                            </li>
                            <li><a class="dropdown-item" href="#">{"Change Email"}</a></li>
                                        <li>
                                            <a class="dropdown-item" href="#">{"Change Password"}</a>
                                        </li>
                                        <li>
                                            <hr class="dropdown-divider" />
                                        </li>
                                        <li>
                                            <a class="dropdown-item" href="#">
                                                <svg xmlns="http://www.w3.org/2000/svg" class="me-1" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line></svg>
                                                <span> {"Block"} </span>
                                            </a>
                                        </li>
                                        <li>
                                            <a class="dropdown-item" href="#">
                                                <i class="bi bi-trash text-danger"></i>
                                                <span class="text-danger">{"Delete"}</span>
                                            </a>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                        </div>
                    </div>
            
                    <div class="mt-3">
                        <ul class="nav nav-tabs" id="myTab" role="tablist" style="font-size:13px;">
                            <li class="nav-item" role="presentation">
                                <button class="nav-link active" id="user-details-tab" data-bs-toggle="tab" data-bs-target="#detailtab" type="button" role="tab" aria-controls="detailtab" aria-selected="true">{"Details"}</button>
                            </li>
                            <li class="nav-item" role="presentation">
                                <button class="nav-link" id="user-devices-tab" data-bs-toggle="tab" data-bs-target="#devicetab" type="button" role="tab" aria-controls="devicetab" aria-selected="false">{"Devices"}</button>
                            </li>
                            <li class="nav-item" role="presentation">
                                <button class="nav-link" id="user-history-tab" data-bs-toggle="tab" data-bs-target="#historytab" type="button" role="tab" aria-controls="historytab" aria-selected="false">{"History"}</button>
                            </li>
                            <li class="nav-item" role="presentation">
                                <button class="nav-link" id="rawjson-tab" data-bs-toggle="tab" data-bs-target="#rawjsontab" type="button" role="tab" aria-controls="rawjsontab" aria-selected="false">{"RAW JSON"}</button>
                            </li>
                            <li class="nav-item" role="presentation">
                                <button class="nav-link" id="authorapp-tab" data-bs-toggle="tab" data-bs-target="#authorapptab" type="button" role="tab" aria-controls="authorapptab" aria-selected="false">{"Authorized Applications"}</button>
                            </li>
                            <li class="nav-item" role="presentation">
                                <button class="nav-link" id="permission-tab" data-bs-toggle="tab" data-bs-target="#permissiontab" type="button" role="tab" aria-controls="permissiontab" aria-selected="false">{"Permissions"}</button>
                            </li>
                            <li class="nav-item" role="presentation">
                                <button class="nav-link" id="roles-tab" data-bs-toggle="tab" data-bs-target="#rolestab" type="button" role="tab" aria-controls="roles" aria-selected="false">{"Roles"}</button>
                            </li>
                        </ul>
                        <div class="tab-content" id="myTabContent">
                            <div class="tab-pane fade show active" id="detailtab" role="tabpanel" aria-labelledby="user-details-tab">
                                <div class="mt-4">
                                    <div class="card">
                                        <div class="card-body">
                                            <div class="container">
                                                <div class="row">
                                                    <div class="col">
                                                        <p class="text-muted mb-1">{"Name"}</p>
                                                        <p class="mb-1">{"yeskahaganta3838@gmail.com"}</p>
                                                        <a href="">{"Edit"}</a>
                                                    </div>
                                                    <div class="col">
                                                        <p class="text-muted mb-1 ">{"Email"}</p>
                                                        <p class="mb-1">{"yeskahaganta3838@gmail.com"}</p>
                                                        <p class="text-muted mb-1">{"(verified)"}</p>
                                                        <a href="">{"Edit"}</a>
                                                    </div>
                                                    <div class="col">
                                                        <p class="text-muted mb-1">{"Signed Up"}</p>
                                                        <p class="mb-1">{"September 7th 2021, 2:17:53 PM"}</p>
                                                    </div>
                                                </div>
                                                <div class="row mt-3">
                                                    <div class="col">
                                                        <p class="text-muted mb-1">{"Primary Identity Provider"}</p>
                                                        <p class="mb-1">{"Database"}</p>
                                                    </div>
                                                    <div class="col mb-1">
                                                        <p class="text-muted mb-1">{"Latest Login"}</p>
                                                        <p class="mb-1">{"Never"}</p>
                                                    </div>
                                                    <div class="col">
                                                        <p class="text-muted mb-1">{"Accounts Associated"}</p>
                                                        <p>{"None"}</p>
                                                    </div>
                                                </div>
                                                <div class="row mt-3">
                                                    <div class="col">
                                                        <p class="text-muted mb-1">{"Browser"}</p>
                                                        <p class="mb-1">{"Chrome 91.0.4472/ Linux 0.0.0"}</p>
                                                    </div>
                                                    <div class="col">
                                                    </div>
                                                    <div class="col">
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
            
                                </div>
                            </div>
                            <div class="tab-pane fade" id="devicetab" role="tabpanel" aria-labelledby="user-devices-tab">
                                <div class="mt-4">
                                    <p>{"These are the devices being used by this particular user. If you click on Unlink, the refresh token will be revoked, forcing the user to re-login on the application."}</p>
                                </div>
                                <div class="mt-4">
                                    <table class="table">
                                        <thead>
                                            <tr>
                                                <th scope="col">{"Client"}</th>
                                                <th scope="col">{"Devices"}</th>
                                                <th scope="col">{"Number of Refresh Tokens"}</th>
                                            </tr>
                                        </thead>
                                    </table>
                                    <div class="p-4" style="background-color: rgb(239,240,242)">
                                        <p class="text-center mb-0">{"There are no linked devices yet. "}</p>
                                    </div>
                                </div>
                            </div>
                            <div class="tab-pane fade" id="historytab" role="tabpanel" aria-labelledby="user-history-tab">
                                <div class="mt-4">
                                    <p>
                                        {"Max. Log Storage: 2 days"}
                                    </p>
                                </div>
                                <div class="mt-4">
                                    <table class="table">
                                        <thead>
                                            <tr>
                                                <th scope="col"></th>
                                                <th scope="col">{"Event"}</th>
                                                <th scope="col">{"When"}</th>
                                                <th scope="col">{"App"}</th>
                                                <th scope="col">{"Identity Provider"}</th>
                                                <th scope="col">{"From"}</th>
                                            </tr>
                                        </thead>
                                    </table>
                                    <div class="p-4" style="background-color: rgb(239,240,242)">
                                        <p class="text-center mb-0">{"There are no logs yet. "}</p>
                                    </div>
                                </div>
                                <div class="mt-4">
                                    <div class="row">
                                        <div class="col d-flex justify-content-start">
                                            <button type="button" class="btn btn-primary" disabled=true>
                                                <i class="bi bi-arrow-left"></i>
                                                <span>{"Previous"}</span>
                                            </button>
                                        </div>
                                        <div class="col d-flex justify-content-center">
                                            <p>{"Page 1"}</p>
                                        </div>
                                        <div class="col d-flex justify-content-end">
                                            <button type="button" class="btn btn-primary" disabled=true>
                                                <i class="bi bi-arrow-right"></i>
                                                <span>{"Next"}</span>
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="tab-pane fade" id="rawjsontab" role="tabpanel" aria-labelledby="rawjson-tab">
            
                            </div>
                            <div class="tab-pane fade" id="authorapptab" role="tabpanel" aria-labelledby="authorapp-tab">
                                <div class="mt-4">
                                    <p>{"List of applications that this user has authorized."}</p>
                                </div>
                                <div class="mt-4">
                                    <div class="p-4" style="background-color: rgb(239,240,242)">
                                        <p class="text-center mb-0">{"There are no applications to display. "}</p>
                                    </div>
                                </div>
                            </div>
                            <div class="tab-pane fade" id="permissiontab" role="tabpanel" aria-labelledby="permission-tab">
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
                                <div class="mt-4">
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
                                    </table>
                                </div>
                                <div class="mt-4">
                                    <div class="p-4" style="background-color: rgb(239,240,242)">
                                        <p class="text-center mb-0">{"There are no permissions assigned to this user yet. "}</p>
                                    </div>
                                </div>
                            </div>
                            <div class="tab-pane fade" id="rolestab" role="tabpanel" aria-labelledby="roles-tab">
                                <div class="mt-4">
                                    <div class="row">
                                        <div class="col d-flex justify-content-start">
                                            <p>{"All Roles assigned to this User."}</p>
                                        </div>
                                        <div class="col d-flex justify-content-end">
                                            <button type="button" class="btn btn-primary">{"Assign Roles"}</button>
                                        </div>
                                    </div>
                                    <div class="mt-4">
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
                                        </table>
                                    </div>
                                    <div class="mt-4">
                                        <div class="p-4" style="background-color: rgb(239,240,242)">
                                            <p class="text-center mb-0">{"There are no roles assigned to this user yet. "}</p>
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
