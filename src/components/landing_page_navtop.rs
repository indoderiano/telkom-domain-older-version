use yew::prelude::*;
use yewdux::prelude::*;
use yewtil::NeqAssign;

use crate::store::reducer_account::{
    AppDispatch,
    DataAccountAction,
    DataAccount
};

pub struct LandingPageNavTop {
    dispatch: AppDispatch,
}

pub enum Msg {}

impl Component for LandingPageNavTop {
    type Message = Msg;
    type Properties = AppDispatch;

    fn create(dispatch: Self::Properties, _: ComponentLink<Self>) -> Self {
        LandingPageNavTop { dispatch }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
        // false
    }

    fn view(&self) -> Html {
        let signup = self.dispatch.callback(|_| {
            let newdata = DataAccount {
                name: Some(String::from("Batman"))
            };
            DataAccountAction::Update(newdata)
        });
        html! {
            <div>
                <div
                    class="d-flex justify-content-between bg-dark"
                    style="padding:0 7rem; font-size: 13px;"
                >
                    <div
                        class="d-flex"
                    >
                        <a
                            class="py-3 text-decoration-none text-light"
                        >
                            <span>
                                {"Okta Completes Acquisition of Auth0"}
                            </span>
                            <span
                                class="ms-3"
                            >
                                {"Learn more"}
                                <i class="bi bi-arrow-right" style="font-size: 13px;"></i>
                            </span>
                        </a>
                    </div>
                    <div
                        class="d-flex justify-content-between align-items-center"
                    >
                        <a
                            class="text-decoration-none text-light px-2 link-primary"
                            style="cursor: pointer;"
                        >
                            {"Login"}
                        </a>
            
                        <a class="dropdown-toggle text-decoration-none text-light px-2 d-flex align-items-center" href="#" id="navbarDarkDropdownMenuLink" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                            <i class="bi bi-globe me-2 fs-5"></i>
                            {"Language"}
                        </a>
                        <ul class="dropdown-menu dropdown-menu-dark" aria-labelledby="navbarDarkDropdownMenuLink">
                            <li><a class="dropdown-item" href="#">{"Deutsch"}</a></li>
                            <li><a class="dropdown-item" href="#">{"English"}</a></li>
                            <li><a class="dropdown-item" href="#">{"Français"}</a></li>
                            <li><a class="dropdown-item" href="#">{"日本語"}</a></li>
                        </ul>
                        
                    </div>
                </div>


                <div
                    class="d-flex justify-content-between border-bottom"
                    style="padding:0 7rem; font-size: 13px;"
                >
                    <div class="nav-item justify-content-center my-auto py-4 d-flex align-items-center">
                        <div class="bg-white p-1 pt-0 rounded me-1 navtop-logo d-inline">
                            <img
                                class=""
                                src="https://i.stack.imgur.com/3Stuq.png"
                                style="width: 23px;"
                            />
                        </div>
                        <span
                            class="fs-4 fw-bold"
                        >{"Telkom Domain"}</span>
                    </div>
                    <div
                        class="d-flex justify-content-between align-items-center"
                    >
                        <button onclick=signup type="button" class="btn btn-outline-dark">{"Sign Up"}</button>
                        
                    </div>
                </div>
            </div>
        }
    }
}
