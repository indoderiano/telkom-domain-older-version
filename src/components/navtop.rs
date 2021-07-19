use yew::prelude::*;

pub struct Navtop {}

pub enum Msg {}

impl Component for Navtop {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Navtop {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="d-flex justify-content-between px-4 py-2 bg-dark"
        style="font-size: 14px;"
    >
        <ul class="nav text-light">
            <li class="nav-item justify-content-center my-auto">
                <div class="bg-white p-1 pt-0 rounded me-3 navtop-logo">
                    <img
                        src="https://i.stack.imgur.com/3Stuq.png"
                        style="width: 23px;"
                    />
                </div>
            </li>
            <div
                class="nav-item justify-content-center my-auto bg-light me-3"
                style="width: 1px; height: 24px; opacity: .8;"
            >
            </div>
            <li class="nav-item px-2"
                style="min-width: 64px;"
            >
                <p class="fw-bolder mb-1">{"user-asdfbd"}</p>
                <span>{"Development"}</span>
            </li>
        </ul>
        <ul class="nav justify-content-end"
            style="flex: 1 1 0%;"
        >
            <li class="nav-item my-auto me-3">
                <button type="button" class="btn btn-dark navtop-hover">
                    <i class="bi bi-search"></i>
                </button>
            </li>
            <li class="nav-item my-auto me-3">
                <button type="button" class="btn btn-outline-light btn-sm">{"Discuss your needs"}</button>
            </li>
            <li class="nav-item my-auto me-3">
                <button type="button" class="btn btn-dark navtop-hover">
                    <i class="bi bi-book me-2"></i> {"Docs"}
                </button>
            </li>
            <li class="nav-item my-auto me-3">
                <button type="button" class="btn btn-dark navtop-hover">
                    <i class="bi bi-bell"></i>
                </button>
            </li>
            <li class="nav-item justify-content-center my-auto">
                <div class="bg-white rounded-circle p-1 navtop-logo">
                    <img
                        src="https://cdn0.iconfinder.com/data/icons/set-ui-app-android/32/8-512.png"
                        style="width: 23px;"
                    />
                </div>
            </li>
        </ul>
    </div>
        }
    }
}
