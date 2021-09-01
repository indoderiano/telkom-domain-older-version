use yew::prelude::*;

pub struct RolesManagement {}

pub enum Msg {}

impl Component for RolesManagement {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        RolesManagement {}
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
                <div
                    class="mx-auto pt-5 pb-5 px-4"
                    style="max-width: 1048px;"
                >
                    
                    <h1 class="fw-bold">{"Roles"}</h1>

                    <div class="mt-3">
                        <div class="card">
                            <div class="card-body">
                                <div class="container text-center p-5">

                                    <div>
                                        <svg width="200px" height="200px" viewBox="0 0 250 250" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M160.44 152.7h-69.9c-6.83 0-12.37-5.54-12.37-12.37 0-6.83 5.54-12.37 12.37-12.37h69.9c6.83 0 12.37 5.54 12.37 12.37-.01 6.83-5.54 12.37-12.37 12.37z" fill="url(#roles_svg__paint0_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M90.93 149.75a9.42 9.42 0 100-18.84 9.42 9.42 0 000 18.84z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M93.21 134.62a6.15 6.15 0 01-2.28 11.86c-3.4 0-6.15-2.75-6.15-6.15 0-2.62 1.64-4.86 3.95-5.74M90.93 132.53v9.34" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M115.01 170.33c-6.83-6.25-10.75-11.66-13.55-15.51-4.69-6.44-8.21-18.37.43-22.49 8.53-4.06 11.47 9.86 24.96 21.06 13.5 11.19 22.23 14.51 28.84 29.96 1.19 2.78 7.59 20.68 7.59 20.68l4.67 13.92h-64.73s-4.99-9.6-2.77-21.62" fill="url(#roles_svg__paint1_linear)"></path><path d="M115.01 170.33c-6.83-6.25-10.75-11.66-13.55-15.51-4.69-6.44-8.21-18.37.43-22.49 8.53-4.06 11.47 9.86 24.96 21.06 13.5 11.19 22.23 14.51 28.84 29.96 1.19 2.78 7.59 20.68 7.59 20.68l4.67 13.92h-64.73s-4.99-9.6-2.77-21.62" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round" stroke-linejoin="round"></path><path d="M100.27 190.98l-4.39-5.33c-2.82-3.76-.88-7.96-.88-7.96 3.55-7.63 13.27-8.14 20.63-7.28l.54.07s4.61.56 9.22 1.99c0 0 9.82 2.62 15.42 8.09" fill="url(#roles_svg__paint2_linear)"></path><path d="M100.27 190.98l-4.39-5.33c-2.82-3.76-.88-7.96-.88-7.96 3.55-7.63 13.27-8.14 20.63-7.28l.54.07s4.61.56 9.22 1.99c0 0 9.82 2.62 15.42 8.09" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round" stroke-linejoin="round"></path><path d="M109.31 133.69s-.43 1.28-1.55 2.79c-.67.91-1.59 1.91-2.8 2.77-.66.47-1.12.98-1.41 1.51-.88 1.58-.25 3.32.96 4.52.83.83 2.62 1.53 4.76 1.09 2.35-.59 4.22-1.35 6.31-5.23" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M101.5 213.47c-1.98.6-4.33.51-5.76.06-2.98-.92-6.96-3.91-8.3-11.15-.87-4.89.24-9.49.24-9.49l.22-.78c.82-2.78 2.65-5.75 4.94-7.32 4.44-3.04 10.07-2.55 14.37-1.46 0 0 10.24 2.23 19.82 10.02" fill="url(#roles_svg__paint3_linear)"></path><path d="M101.5 213.47c-1.98.6-4.33.51-5.76.06-2.98-.92-6.96-3.91-8.3-11.15-.87-4.89.24-9.49.24-9.49l.22-.78c.82-2.78 2.65-5.75 4.94-7.32 4.44-3.04 10.07-2.55 14.37-1.46 0 0 10.24 2.23 19.82 10.02" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round" stroke-linejoin="round"></path><path d="M185.13 217.96H75.58" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M100.45 196.34s-1.76 8.05 1.05 17.12l-1.05-17.12z" fill="#fff"></path><path d="M100.45 196.34s-1.76 8.05 1.05 17.12" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M77.5 109.87H24.99c-1.65 0-2.99-1.34-2.99-2.99V45.05c0-1.65 1.34-2.99 2.99-2.99H77.5c1.65 0 2.99 1.34 2.99 2.99v61.84c0 1.65-1.34 2.98-2.99 2.98z" fill="#BDC4CF" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M25.27 109.87v-8.5c0-6.73 7.88-12.11 18.67-13.91V76.5c0-3.76 2.82-7.14 6.58-7.33a6.947 6.947 0 017.32 6.94v1.06l2.36 4.8h-2.35v5.39c11.12 1.67 19.36 7.45 19.36 14.31v8.2H25.27zM28.12 46.58c0 .81-.66 1.47-1.47 1.47-.81 0-1.47-.66-1.47-1.47 0-.81.66-1.47 1.47-1.47.81 0 1.47.66 1.47 1.47zM32.8 46.58c0 .81-.66 1.47-1.47 1.47-.81 0-1.47-.66-1.47-1.47 0-.81.66-1.47 1.47-1.47.81 0 1.47.66 1.47 1.47zM37.48 46.58c0 .81-.66 1.47-1.47 1.47-.81 0-1.47-.66-1.47-1.47 0-.81.66-1.47 1.47-1.47.81 0 1.47.66 1.47 1.47z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M75.48 65.9H63.56c-.72 0-1.3-.58-1.3-1.3V52.68c0-.72.58-1.3 1.3-1.3h11.92c.72 0 1.3.58 1.3 1.3V64.6c0 .72-.58 1.3-1.3 1.3z" fill="url(#roles_svg__paint4_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M64.29 53.47l10.47 10.47M74.76 53.47L64.29 63.94" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M57.21 65.9H45.29c-.72 0-1.3-.58-1.3-1.3V52.68c0-.72.58-1.3 1.3-1.3h11.92c.72 0 1.3.58 1.3 1.3V64.6c0 .72-.59 1.3-1.3 1.3z" fill="url(#roles_svg__paint5_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M56.22 53.62l-6.65 9.24-3.36-3.36" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M38.87 65.9H26.95c-.72 0-1.3-.58-1.3-1.3V52.68c0-.72.58-1.3 1.3-1.3h11.92c.72 0 1.3.58 1.3 1.3V64.6c0 .72-.58 1.3-1.3 1.3z" fill="url(#roles_svg__paint6_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M37.89 53.62l-6.66 9.24-3.36-3.36" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M158.72 119.01H92.19c-2.09 0-3.79-1.69-3.79-3.79V36.79c0-2.09 1.69-3.79 3.79-3.79h66.53c2.09 0 3.79 1.69 3.79 3.79v78.44c0 2.09-1.7 3.78-3.79 3.78z" fill="url(#roles_svg__paint7_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round" stroke-linejoin="round"></path><path d="M92.55 119.01v-10.86c0-8.53 9.99-15.35 23.66-17.62V76.64c0-4.77 3.57-9.04 8.33-9.29 5.08-.27 9.28 3.77 9.28 8.79v1.34l2.99 6.08h-2.98v6.84c14.09 2.12 24.53 9.43 24.53 18.13v10.48H92.55zM96.32 38.72c0 1.03-.83 1.86-1.86 1.86-1.03 0-1.86-.83-1.86-1.86 0-1.03.83-1.86 1.86-1.86 1.03.01 1.86.84 1.86 1.86zM102.25 38.72c0 1.03-.83 1.86-1.86 1.86-1.03 0-1.86-.83-1.86-1.86 0-1.03.83-1.86 1.86-1.86 1.02.01 1.86.84 1.86 1.86zM108.17 38.72c0 1.03-.83 1.86-1.86 1.86-1.03 0-1.86-.83-1.86-1.86 0-1.03.83-1.86 1.86-1.86 1.03.01 1.86.84 1.86 1.86z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M133.18 63.2h-15.1c-.91 0-1.65-.74-1.65-1.65v-15.1c0-.91.74-1.65 1.65-1.65h15.1c.91 0 1.65.74 1.65 1.65v15.1c0 .91-.74 1.65-1.65 1.65z" fill="url(#roles_svg__paint8_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M131.93 47.65l-8.43 11.7-4.25-4.26" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M109.79 63.2h-15.1c-.91 0-1.65-.74-1.65-1.65v-15.1c0-.91.74-1.65 1.65-1.65h15.1c.91 0 1.65.74 1.65 1.65v15.1c0 .91-.74 1.65-1.65 1.65z" fill="url(#roles_svg__paint9_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M108.54 47.65l-8.42 11.7-4.26-4.26" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M225.92 109.87H173.4c-1.65 0-2.99-1.34-2.99-2.99V45.05c0-1.65 1.34-2.99 2.99-2.99h52.52c1.65 0 2.99 1.34 2.99 2.99v61.84c0 1.65-1.34 2.98-2.99 2.98z" fill="#BDC4CF" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M173.69 109.88v-8.51c0-6.73 7.88-12.11 18.67-13.91V76.5c0-3.76 2.82-7.14 6.58-7.33a6.947 6.947 0 017.32 6.94v1.06l2.36 4.8h-2.35v5.39c11.12 1.67 19.36 7.45 19.36 14.31v8.21h-51.94zM176.66 46.58c0 .81-.66 1.47-1.47 1.47-.81 0-1.47-.66-1.47-1.47 0-.81.66-1.47 1.47-1.47.82 0 1.47.66 1.47 1.47zM181.34 46.58c0 .81-.66 1.47-1.47 1.47-.81 0-1.47-.66-1.47-1.47 0-.81.66-1.47 1.47-1.47.82 0 1.47.66 1.47 1.47zM186.03 46.58c0 .81-.66 1.47-1.47 1.47-.81 0-1.47-.66-1.47-1.47 0-.81.66-1.47 1.47-1.47.81 0 1.47.66 1.47 1.47z" fill="#fff" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M206.05 65.9h-11.92c-.72 0-1.3-.58-1.3-1.3V52.68c0-.72.58-1.3 1.3-1.3h11.92c.72 0 1.3.58 1.3 1.3V64.6c0 .72-.58 1.3-1.3 1.3z" fill="url(#roles_svg__paint10_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10"></path><path d="M194.86 53.47l10.47 10.47M205.33 53.47l-10.47 10.47" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M187.67 65.9h-11.92c-.72 0-1.3-.58-1.3-1.3V52.68c0-.72.58-1.3 1.3-1.3h11.92c.72 0 1.3.58 1.3 1.3V64.6c0 .72-.58 1.3-1.3 1.3z" fill="url(#roles_svg__paint11_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M186.69 53.62l-6.65 9.24-3.36-3.36" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M224.43 65.9h-11.92c-.72 0-1.3-.58-1.3-1.3V52.68c0-.72.58-1.3 1.3-1.3h11.92c.72 0 1.3.58 1.3 1.3V64.6c0 .72-.58 1.3-1.3 1.3z" fill="url(#roles_svg__paint12_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M223.71 53.47l-10.47 10.47M213.33 53.47l10.47 10.47" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M156.57 63.2h-15.1c-.91 0-1.65-.74-1.65-1.65v-15.1c0-.91.74-1.65 1.65-1.65h15.1c.91 0 1.65.74 1.65 1.65v15.1c-.01.91-.74 1.65-1.65 1.65z" fill="url(#roles_svg__paint13_linear)" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><path d="M155.32 47.65l-8.43 11.7-4.26-4.26" stroke="#000" stroke-width="0.4" stroke-miterlimit="10" stroke-linecap="round"></path><defs><linearGradient id="roles_svg__paint0_linear" x1="78.166" y1="140.332" x2="172.803" y2="140.332" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="roles_svg__paint1_linear" x1="102.302" y1="131.12" x2="141.393" y2="231.369" gradientUnits="userSpaceOnUse"><stop stop-color="#E8EAEC"></stop><stop offset="1" stop-color="#fff"></stop></linearGradient><linearGradient id="roles_svg__paint2_linear" x1="98.503" y1="132.602" x2="137.593" y2="232.851" gradientUnits="userSpaceOnUse"><stop stop-color="#E8EAEC"></stop><stop offset="1" stop-color="#fff"></stop></linearGradient><linearGradient id="roles_svg__paint3_linear" x1="83.811" y1="138.326" x2="122.905" y2="238.584" gradientUnits="userSpaceOnUse"><stop stop-color="#E8EAEC"></stop><stop offset="1" stop-color="#fff"></stop></linearGradient><linearGradient id="roles_svg__paint4_linear" x1="63.03" y1="67.205" x2="76.107" y2="49.951" gradientUnits="userSpaceOnUse"><stop stop-color="#FC50D5"></stop><stop offset="1" stop-color="#EAC550"></stop></linearGradient><linearGradient id="roles_svg__paint5_linear" x1="40.866" y1="68.603" x2="58.956" y2="51.237" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="roles_svg__paint6_linear" x1="22.533" y1="68.603" x2="40.623" y2="51.237" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="roles_svg__paint7_linear" x1="88.406" y1="76.005" x2="162.506" y2="76.005" gradientUnits="userSpaceOnUse"><stop stop-color="#3885FF"></stop><stop offset="1" stop-color="#635DFF"></stop></linearGradient><linearGradient id="roles_svg__paint8_linear" x1="112.481" y1="66.627" x2="135.397" y2="44.627" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="roles_svg__paint9_linear" x1="89.093" y1="66.627" x2="112.009" y2="44.627" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="roles_svg__paint10_linear" x1="192.784" y1="66.347" x2="207.707" y2="50.609" gradientUnits="userSpaceOnUse"><stop stop-color="#FC50D5"></stop><stop offset="1" stop-color="#EAC550"></stop></linearGradient><linearGradient id="roles_svg__paint11_linear" x1="171.335" y1="68.603" x2="189.425" y2="51.237" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient><linearGradient id="roles_svg__paint12_linear" x1="209.945" y1="66.74" x2="226.225" y2="51.274" gradientUnits="userSpaceOnUse"><stop offset="0.044" stop-color="#FF4F40"></stop><stop offset="1" stop-color="#FC44DD"></stop></linearGradient><linearGradient id="roles_svg__paint13_linear" x1="135.869" y1="66.627" x2="158.785" y2="44.627" gradientUnits="userSpaceOnUse"><stop stop-color="#1BC99F"></stop><stop offset="1" stop-color="#3EC6EB"></stop></linearGradient></defs>
                                        </svg>
                                    </div>

                                    <h2>{"You don't have any roles yet"}</h2>

                                    <div class="pt-3">
                                        <p class="fs-6 text-center">
                                            {"Create roles to represent the types of users that access your applications. Assign permissions to those roles to control what users are allowed to do in your apps."}
                                        </p>
                                    </div>

                                    <div class="mt-2">
                                        <button type="button" data-bs-toggle="modal" data-bs-target="#addRoleModal" class="btn btn-primary text-center">
                                            <i class="bi bi-plus me-2" style="margin-left: -5px;"></i>
                                            <span>{"Create Role"}</span>
                                        </button>
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
