use yew::prelude::*;

pub struct UsersCreated {}

pub enum Msg {}

impl Component for UsersCreated {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        UsersCreated {}
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
            <div class="p-5">
            <h1>{"Users"}</h1>
            <div class="mt-5">
                <div class="card">
                    <div class="card-body">
                        <div class="container text-center p-5">
                            <div>
                                <svg width="200px" height="200px" viewBox="0 0 250 250" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M129.489 177.593H17.939a8.549 8.549 0 01-8.549-8.549v-82.28a8.549 8.549 0 018.549-8.55h111.55a8.55 8.55 0 018.549 8.55v82.28a8.549 8.549 0 01-8.549 8.549z" fill="#EAECEE" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M38.718 111.866H20.95a3.84 3.84 0 01-3.837-3.838V90.251a3.84 3.84 0 013.837-3.837h17.768a3.84 3.84 0 013.837 3.837v17.768a3.83 3.83 0 01-3.837 3.847z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M19.804 102.375c0-2.604 3.04-4.79 7.208-5.489v-4.342c0-1.506 1.234-2.76 2.74-2.73a2.69 2.69 0 012.632 2.681v.408l.913 1.855h-.903v2.08c4.294.64 7.48 2.875 7.48 5.527v5.197a.87.87 0 01-.874.874H20.736a.912.912 0 01-.913-.913v-5.148h-.019zM48.131 125.883h-29.96a1.065 1.065 0 01-1.068-1.068c0-.593.476-1.069 1.069-1.069H48.13c.593 0 1.069.476 1.069 1.069 0 .592-.476 1.068-1.069 1.068zM38.68 119.89H18.171a1.066 1.066 0 01-1.068-1.069c0-.592.476-1.068 1.068-1.068H38.67c.592 0 1.068.476 1.068 1.068 0 .593-.476 1.069-1.059 1.069zM48.131 131.877h-29.96a1.065 1.065 0 01-1.068-1.068c0-.593.476-1.069 1.069-1.069H48.13c.593 0 1.069.476 1.069 1.069 0 .592-.476 1.068-1.069 1.068zM38.795 159.32H18.172a1.065 1.065 0 01-1.069-1.068c0-.593.476-1.069 1.069-1.069h20.623c.593 0 1.07.476 1.07 1.069 0 .592-.477 1.068-1.07 1.068zM38.795 165.1H18.172a1.065 1.065 0 01-1.069-1.068c0-.593.476-1.069 1.069-1.069h20.623c.593 0 1.07.476 1.07 1.069 0 .583-.477 1.068-1.07 1.068zM170.698 84.792h51.165a4.333 4.333 0 014.323 4.323v10.87h6.577v-10.87c0-6.014-4.886-10.9-10.9-10.9h-51.136l-.029 6.577z" fill="#BDC4CF" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M223.116 105.027h12.338v-2.915a3.254 3.254 0 00-3.255-3.254h-5.828a3.255 3.255 0 00-3.255 3.254v2.915z" fill="#BDC4CF" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M164.393 177.583h71.867c2.059 0 3.74-1.67 3.74-3.74v-65.076c0-2.06-1.671-3.74-3.74-3.74h-71.867a3.741 3.741 0 00-3.74 3.74v65.076a3.74 3.74 0 003.74 3.74z" fill="#EAECEE" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path stroke-width="0.4" stroke-miterlimit="10"></path><path d="M211.469 134.06h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 134.06h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 134.06h7.13v-7.131h-7.13v7.131zM190.076 134.06h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 134.06h7.13v-7.131h-7.13v7.131zM211.469 141.187h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 141.187h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 141.187h7.13v-7.13h-7.13v7.13zM190.076 141.187h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 141.187h7.13v-7.13h-7.13v7.13zM211.469 148.315h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 148.315h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 148.315h7.13v-7.131h-7.13v7.131zM190.076 148.315h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 148.315h7.13v-7.131h-7.13v7.131zM211.469 155.442h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 155.442h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 155.442h7.13v-7.13h-7.13v7.13zM190.076 155.442h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 155.442h7.13v-7.13h-7.13v7.13zM211.469 162.57h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 162.57h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 162.57h7.13v-7.13h-7.13v7.13zM190.076 162.57h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 162.57h7.13v-7.13h-7.13v7.13zM211.469 169.698h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 169.698h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 169.698h7.13v-7.131h-7.13v7.131zM190.076 169.698h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 169.698h7.13v-7.131h-7.13v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M229.217 122.649a6.12 6.12 0 10-.002-12.242 6.12 6.12 0 00.002 12.242z" fill="#BDC4CF" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M229.217 120.599a4.07 4.07 0 10-.001-8.141 4.07 4.07 0 00.001 8.141z" fill="#EAECEE" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M231.449 117.356a2.38 2.38 0 10-4.462-1.658 2.38 2.38 0 004.462 1.658z" fill="#EAECEE" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M199.433 182.214v16.493c0 7.725-34.822 13.984-77.784 13.984s-77.784-6.259-77.784-13.984v-16.493" fill="#BDC4CF"></path><path d="M199.433 182.214v16.493c0 7.725-34.822 13.984-77.784 13.984s-77.784-6.259-77.784-13.984v-16.493" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M121.649 196.198c42.959 0 77.784-6.261 77.784-13.984 0-7.724-34.825-13.985-77.784-13.985s-77.784 6.261-77.784 13.985c0 7.723 34.825 13.984 77.784 13.984z" fill="url(#users_svg__paint0_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M181.518 185.237H61.791a4.076 4.076 0 01-4.075-4.076V52.13a4.076 4.076 0 014.075-4.076h119.727a4.076 4.076 0 014.076 4.076v129.042a4.082 4.082 0 01-4.076 4.065z" fill="url(#users_svg__paint1_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M57.716 71.13h127.866V52.164a4.105 4.105 0 00-4.109-4.109H61.825a4.105 4.105 0 00-4.11 4.11V71.13z" fill="#BDC4CF" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M103.69 127.26V99.313c0-9.45 7.658-17.109 17.108-17.109s17.109 7.659 17.109 17.109v2.598l5.811 5.811h-5.777v19.292l-16.325 16.974-17.926-16.728z" fill="url(#users_svg__paint2_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M121.604 143.977l16.247-16.896.078-.09c27.376 4.109 47.654 18.329 47.654 35.225v19.001a4.005 4.005 0 01-4.009 4.009H61.735a4.005 4.005 0 01-4.008-4.009v-19.001c0-16.571 19.404-30.556 45.962-34.978l17.915 16.739z" fill="url(#users_svg__paint3_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M85.64 158.04v27.197M156.941 158.04v27.197" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M143.774 66.865a7.278 7.278 0 100-14.557 7.278 7.278 0 000 14.557z" fill="url(#users_svg__paint4_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M158.33 66.865a7.278 7.278 0 100-14.557 7.278 7.278 0 000 14.557z" fill="url(#users_svg__paint5_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M179.961 61.276a7.277 7.277 0 10-14.165-3.345 7.277 7.277 0 005.41 8.755 7.277 7.277 0 008.755-5.41z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M165.596 59.273c1.411-2.25 4.143-3.773 7.278-3.773s5.867 1.534 7.278 3.785v.01c-1.411 2.251-4.131 3.774-7.266 3.774-3.147-.011-5.879-1.534-7.29-3.796z" fill="url(#users_svg__paint6_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M173.02 63.069a3.784 3.784 0 100-7.568 3.784 3.784 0 000 7.568z" fill="#000" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M162.282 54.604h-5.755v8.308h5.755v-8.308z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M160.121 56.273h-5.755v8.307h5.755v-8.307z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M143.774 55.657v7.871M139.844 59.587h7.871M55.275 206.007v-16.493M43.877 182.852v15.855M68.32 192.414v16.493M76.504 193.657v16.493M199.433 182.852v15.866s.66 3.896-11.399 7.3v-16.493M174.979 192.414v16.493M166.794 193.657v16.493" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M48.3 61.86v6.192c0 10.547 8.554 19.101 19.101 19.101 10.547 0 19.102-8.554 19.102-19.101V61.86" fill="#000"></path><path d="M48.3 61.86v6.192c0 10.547 8.554 19.101 19.101 19.101 10.547 0 19.102-8.554 19.102-19.101V61.86" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M67.412 80.962c10.55 0 19.102-8.553 19.102-19.102 0-10.55-8.552-19.102-19.102-19.102S48.31 51.31 48.31 61.86s8.553 19.102 19.102 19.102z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M67.412 75.71c7.65 0 13.85-6.2 13.85-13.85 0-7.65-6.2-13.85-13.85-13.85-7.65 0-13.85 6.2-13.85 13.85 0 7.65 6.2 13.85 13.85 13.85z" fill="url(#users_svg__paint7_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M57.66 58.176V62.7l10.413 10.413 25.853-25.854v-4.49L68.073 68.59 57.66 58.176z" fill="#000" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M68.073 68.59L57.66 58.175l6.147-6.136 4.266 4.266L87.79 36.59l6.136 6.147-25.853 25.853z" fill="url(#users_svg__paint8_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><defs><linearGradient id="users_svg__paint0_linear" x1="43.87" y1="182.217" x2="199.436" y2="182.217" gradientUnits="userSpaceOnUse"><stop offset="0.044" stop-color="#FF4F40"></stop><stop offset="1" stop-color="#FC44DD"></stop></linearGradient><linearGradient id="users_svg__paint1_linear" x1="49.404" y1="189.625" x2="170.201" y2="67.609" gradientUnits="userSpaceOnUse"><stop stop-color="#3885FF"></stop><stop offset="1" stop-color="#635DFF"></stop></linearGradient><linearGradient id="users_svg__paint2_linear" x1="89.714" y1="177.883" x2="111.067" y2="132.127" gradientUnits="userSpaceOnUse"><stop stop-color="#E9ECEE"></stop><stop offset="1" stop-color="#fff"></stop></linearGradient><linearGradient id="users_svg__paint3_linear" x1="112.834" y1="188.672" x2="134.187" y2="142.916" gradientUnits="userSpaceOnUse"><stop stop-color="#E9ECEE"></stop><stop offset="1" stop-color="#fff"></stop></linearGradient><linearGradient id="users_svg__paint4_linear" x1="136.504" y1="59.592" x2="151.053" y2="59.592" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="users_svg__paint5_linear" x1="151.053" y1="59.592" x2="165.602" y2="59.592" gradientUnits="userSpaceOnUse"><stop stop-color="#3885FF"></stop><stop offset="1" stop-color="#635DFF"></stop></linearGradient><linearGradient id="users_svg__paint6_linear" x1="165.6" y1="59.281" x2="180.154" y2="59.281" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="users_svg__paint7_linear" x1="66.828" y1="87.133" x2="67.903" y2="40.499" gradientUnits="userSpaceOnUse"><stop stop-color="#3885FF"></stop><stop offset="1" stop-color="#635DFF"></stop></linearGradient><linearGradient id="users_svg__paint8_linear" x1="57.663" y1="52.589" x2="93.927" y2="52.589" gradientUnits="userSpaceOnUse"><stop stop-color="#2BC8A0"></stop><stop offset="1" stop-color="#47C7E9"></stop></linearGradient></defs></svg>
                            </div>
                            <h2>{"You don't have any users yet"}</h2>
                            <div class="pt-3">
                                <p class="fs-6 text-center">
                                    {"All of your users will be found here, regardless of the authentication method they use to access your applications."}
                                </p>
                            </div>
                            <div class="mt-4">
                                <button type="button" class="btn btn-primary align-items-center" data-bs-toggle="modal" data-bs-target="#addRoleModal">
                                    <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                    <span>{"Create User"}</span>
                                </button>
                            </div>
                            <div class="mt-4">
                                <a href="">{"Learn more"}</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div class="modal fade" id="addRoleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-scrollable" role="document">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Create user"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close">
                        </button>
                    </div>
                    <div class="modal-body">
                        <div class="form-group mt-2">
                            <label for="userInputEmail">{"Email "} <span style="color:red">{"*"}</span></label>
                            <input type="email" class="form-control" id="userInputEmail" aria-describedby="emailHelp" placeholder="email@example.com" />
                        </div>
                        <div class="form-group mt-4 mb-3">
                            <label for="createUserInputPassword">{"Password "}<span style="color:red">{"*"}</span></label>
                            <input type="password" class="form-control" id="createUserInputPassword" placeholder="****************" />
                        </div>
                        <div class="form-group mt-4 mb-3">
                            <label for="repeatInputPassword">{"Repeat Password "}<span style="color:red">{"*"}</span></label>
                            <input type="password" class="form-control" id="repeatInputPassword" placeholder="****************" />
                        </div>
                        <div class="form-group mt-4 mb-3">
                            <label>{"Connection "}<span style="color:red">{"*"}</span></label>
                            <select class="form-select" aria-label="Default select example">
                                <option selected=true value="default">{"Default"}</option>
                            </select>
                        </div>


                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"} </button>
                        <button type="button" class="btn btn-primary">{"Create"}</button>
                    </div>

                </div>
            </div>
        </div>
        stroke-width="0.4" stroke-miterlimit="10"></path><path d="M183.987 113.332h33.582c.574 0 1.03-.456 1.03-1.029s-.456-1.03-1.03-1.03h-33.582c-.573 0-1.03.457-1.03 1.03s.457 1.029 1.03 1.029zM183.987 117.548h33.582c.574 0 1.03-.456 1.03-1.029s-.456-1.03-1.03-1.03h-33.582c-.573 0-1.03.457-1.03 1.03s.457 1.029 1.03 1.029zM183.987 121.774h33.582c.574 0 1.03-.456 1.03-1.029 0-.574-.456-1.03-1.03-1.03h-33.582a1.03 1.03 0 000 2.059z" fill="#EAECEE" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M211.469 134.06h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 134.06h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 134.06h7.13v-7.131h-7.13v7.131zM190.076 134.06h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 134.06h7.13v-7.131h-7.13v7.131zM211.469 141.187h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 141.187h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 141.187h7.13v-7.13h-7.13v7.13zM190.076 141.187h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 141.187h7.13v-7.13h-7.13v7.13zM211.469 148.315h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 148.315h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 148.315h7.13v-7.131h-7.13v7.131zM190.076 148.315h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 148.315h7.13v-7.131h-7.13v7.131zM211.469 155.442h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 155.442h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 155.442h7.13v-7.13h-7.13v7.13zM190.076 155.442h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 155.442h7.13v-7.13h-7.13v7.13zM211.469 162.57h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 162.57h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 162.57h7.13v-7.13h-7.13v7.13zM190.076 162.57h7.131v-7.13h-7.131v7.13z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 162.57h7.13v-7.13h-7.13v7.13zM211.469 169.698h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M204.341 169.698h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M197.214 169.698h7.13v-7.131h-7.13v7.131zM190.076 169.698h7.131v-7.131h-7.131v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M182.949 169.698h7.13v-7.131h-7.13v7.131z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M229.217 122.649a6.12 6.12 0 10-.002-12.242 6.12 6.12 0 00.002 12.242z" fill="#BDC4CF" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M229.217 120.599a4.07 4.07 0 10-.001-8.141 4.07 4.07 0 00.001 8.141z" fill="#EAECEE" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M231.449 117.356a2.38 2.38 0 10-4.462-1.658 2.38 2.38 0 004.462 1.658z" fill="#EAECEE" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M199.433 182.214v16.493c0 7.725-34.822 13.984-77.784 13.984s-77.784-6.259-77.784-13.984v-16.493" fill="#BDC4CF"></path><path d="M199.433 182.214v16.493c0 7.725-34.822 13.984-77.784 13.984s-77.784-6.259-77.784-13.984v-16.493" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M121.649 196.198c42.959 0 77.784-6.261 77.784-13.984 0-7.724-34.825-13.985-77.784-13.985s-77.784 6.261-77.784 13.985c0 7.723 34.825 13.984 77.784 13.984z" fill="url(#users_svg__paint0_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M181.518 185.237H61.791a4.076 4.076 0 01-4.075-4.076V52.13a4.076 4.076 0 014.075-4.076h119.727a4.076 4.076 0 014.076 4.076v129.042a4.082 4.082 0 01-4.076 4.065z" fill="url(#users_svg__paint1_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M57.716 71.13h127.866V52.164a4.105 4.105 0 00-4.109-4.109H61.825a4.105 4.105 0 00-4.11 4.11V71.13z" fill="#BDC4CF" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M103.69 127.26V99.313c0-9.45 7.658-17.109 17.108-17.109s17.109 7.659 17.109 17.109v2.598l5.811 5.811h-5.777v19.292l-16.325 16.974-17.926-16.728z" fill="url(#users_svg__paint2_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M121.604 143.977l16.247-16.896.078-.09c27.376 4.109 47.654 18.329 47.654 35.225v19.001a4.005 4.005 0 01-4.009 4.009H61.735a4.005 4.005 0 01-4.008-4.009v-19.001c0-16.571 19.404-30.556 45.962-34.978l17.915 16.739z" fill="url(#users_svg__paint3_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M85.64 158.04v27.197M156.941 158.04v27.197" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M143.774 66.865a7.278 7.278 0 100-14.557 7.278 7.278 0 000 14.557z" fill="url(#users_svg__paint4_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M158.33 66.865a7.278 7.278 0 100-14.557 7.278 7.278 0 000 14.557z" fill="url(#users_svg__paint5_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M179.961 61.276a7.277 7.277 0 10-14.165-3.345 7.277 7.277 0 005.41 8.755 7.277 7.277 0 008.755-5.41z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M165.596 59.273c1.411-2.25 4.143-3.773 7.278-3.773s5.867 1.534 7.278 3.785v.01c-1.411 2.251-4.131 3.774-7.266 3.774-3.147-.011-5.879-1.534-7.29-3.796z" fill="url(#users_svg__paint6_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M173.02 63.069a3.784 3.784 0 100-7.568 3.784 3.784 0 000 7.568z" fill="#000" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M162.282 54.604h-5.755v8.308h5.755v-8.308z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M160.121 56.273h-5.755v8.307h5.755v-8.307z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M143.774 55.657v7.871M139.844 59.587h7.871M55.275 206.007v-16.493M43.877 182.852v15.855M68.32 192.414v16.493M76.504 193.657v16.493M199.433 182.852v15.866s.66 3.896-11.399 7.3v-16.493M174.979 192.414v16.493M166.794 193.657v16.493" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M48.3 61.86v6.192c0 10.547 8.554 19.101 19.101 19.101 10.547 0 19.102-8.554 19.102-19.101V61.86" fill="#000"></path><path d="M48.3 61.86v6.192c0 10.547 8.554 19.101 19.101 19.101 10.547 0 19.102-8.554 19.102-19.101V61.86" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M67.412 80.962c10.55 0 19.102-8.553 19.102-19.102 0-10.55-8.552-19.102-19.102-19.102S48.31 51.31 48.31 61.86s8.553 19.102 19.102 19.102z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M67.412 75.71c7.65 0 13.85-6.2 13.85-13.85 0-7.65-6.2-13.85-13.85-13.85-7.65 0-13.85 6.2-13.85 13.85 0 7.65 6.2 13.85 13.85 13.85z" fill="url(#users_svg__paint7_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M57.66 58.176V62.7l10.413 10.413 25.853-25.854v-4.49L68.073 68.59 57.66 58.176z" fill="#000" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M68.073 68.59L57.66 58.175l6.147-6.136 4.266 4.266L87.79 36.59l6.136 6.147-25.853 25.853z" fill="url(#users_svg__paint8_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><defs><linearGradient id="users_svg__paint0_linear" x1="43.87" y1="182.217" x2="199.436" y2="182.217" gradientUnits="userSpaceOnUse"><stop offset="0.044" stop-color="#FF4F40"></stop><stop offset="1" stop-color="#FC44DD"></stop></linearGradient><linearGradient id="users_svg__paint1_linear" x1="49.404" y1="189.625" x2="170.201" y2="67.609" gradientUnits="userSpaceOnUse"><stop stop-color="#3885FF"></stop><stop offset="1" stop-color="#635DFF"></stop></linearGradient><linearGradient id="users_svg__paint2_linear" x1="89.714" y1="177.883" x2="111.067" y2="132.127" gradientUnits="userSpaceOnUse"><stop stop-color="#E9ECEE"></stop><stop offset="1" stop-color="#fff"></stop></linearGradient><linearGradient id="users_svg__paint3_linear" x1="112.834" y1="188.672" x2="134.187" y2="142.916" gradientUnits="userSpaceOnUse"><stop stop-color="#E9ECEE"></stop><stop offset="1" stop-color="#fff"></stop></linearGradient><linearGradient id="users_svg__paint4_linear" x1="136.504" y1="59.592" x2="151.053" y2="59.592" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="users_svg__paint5_linear" x1="151.053" y1="59.592" x2="165.602" y2="59.592" gradientUnits="userSpaceOnUse"><stop stop-color="#3885FF"></stop><stop offset="1" stop-color="#635DFF"></stop></linearGradient><linearGradient id="users_svg__paint6_linear" x1="165.6" y1="59.281" x2="180.154" y2="59.281" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="users_svg__paint7_linear" x1="66.828" y1="87.133" x2="67.903" y2="40.499" gradientUnits="userSpaceOnUse"><stop stop-color="#3885FF"></stop><stop offset="1" stop-color="#635DFF"></stop></linearGradient><linearGradient id="users_svg__paint8_linear" x1="57.663" y1="52.589" x2="93.927" y2="52.589" gradientUnits="userSpaceOnUse"><stop stop-color="#2BC8A0"></stop><stop offset="1" stop-color="#47C7E9"></stop></linearGradient></defs></svg>
                            </div>
                            <h2>{"You don't have any users yet"}</h2>
                            <div class="pt-3">
                                <p class="fs-6 text-center">
                                    {"All of your users will be found here, regardless of the authentication method they use to access your applications."}
                                </p>
                            </div>
                            <div class="mt-4">
                                <button type="button" class="btn btn-primary align-items-center" data-bs-toggle="modal" data-bs-target="#addRoleModal">
                                    <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                    <span>{"Create User"}</span>
                                </button>
                            </div>
                            <div class="mt-4">
                                <a href="">{"Learn more"}</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div class="modal fade" id="addRoleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
            <div class="modal-dialog modal-dialog-scrollable" role="document">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title" id="exampleModalLabel">{"Create user"}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close">
                        </button>
                    </div>
                    <div class="modal-body">
                        <div class="form-group mt-2">
                            <label for="userInputEmail">{"Email "} <span style="color:red">{"*"}</span></label>
                            <input type="email" class="form-control" id="userInputEmail" aria-describedby="emailHelp" placeholder="email@example.com" />
                        </div>
                        <div class="form-group mt-4 mb-3">
                            <label for="createUserInputPassword">{"Password "}<span style="color:red">{"*"}</span></label>
                            <input type="password" class="form-control" id="createUserInputPassword" placeholder="****************" />
                        </div>
                        <div class="form-group mt-4 mb-3">
                            <label for="repeatInputPassword">{"Repeat Password "}<span style="color:red">{"*"}</span></label>
                            <input type="password" class="form-control" id="repeatInputPassword" placeholder="****************" />
                        </div>
                        <div class="form-group mt-4 mb-3">
                            <label>{"Connection "}<span style="color:red">{"*"}</span></label>
                            <select class="form-select" aria-label="Default select example">
                                <option selected=true value="default">{"Default"}</option>
                            </select>
                        </div>


                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancel"} </button>
                        <button type="button" class="btn btn-primary">{"Create"}</button>
                    </div>

                </div>
            </div>
        </div
            </>
        }
    }
}
