use yew::prelude::*;
use yew::services::ConsoleService;
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;
use crate::store::reducer_account::{
  AppDispatch,
  // DataAccountAction,
  // DataAccount,
};

pub struct Sidebar {
  dispatch: AppDispatch,
}

pub enum Msg {}

impl Component for Sidebar {
  type Message = Msg;
  type Properties = AppDispatch;

  fn create(dispatch: Self::Properties, _: ComponentLink<Self>) -> Self {
    Sidebar {
      dispatch,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    type Anchor = RouterAnchor<AppRoute>;
    let acc = self.dispatch.state().clone();
    ConsoleService::info(&format!("sidebar acc tenant id is {:?}", acc.tenant_id));
    let tenant_id = if let Some(id) = acc.tenant_id { id } else { String::from("no_tenant_id") };
    html! {
      <div
        class="col-auto col-md-3 col-xl-3 px-sm-3 px-0 bg-white fw-bold"
        style="font-size:14px; height: calc(100vh - 64px); overflow-y: scroll;"
      >
        <div
          class="d-flex border-end flex-column align-items-center align-items-sm-start py-4 px-3 text-white h-auto overflow-auto"
        >
          
          <ul
            class="nav flex-column w-100 text-color-primary"
            id="nav_accordion"
            style="list-style-type:none; font-weight: bold; "
          >
            <li class="nav-item">
                <Anchor
                  route=AppRoute::GettingStarted
                  classes="text-decoration-none"
                >
                <a class="nav-link text-color-primary" href="#"> <span
                    style="padding: 4px 8px;"><i class="bi bi-lightning-charge-fill"></i></span>
                  {"Getting Started"}
                </a>
                </Anchor>
            </li>
            
            <li class="nav-item">
              <Anchor
                route=AppRoute::Activity
                classes="text-decoration-none"
              >
              <a class="nav-link text-color-primary" href="#"> <span
                  style="padding: 4px 8px;"><i class="bi bi-graph-up"></i></span>
                {"Activity"}
              </a>
              </Anchor>
            </li>

            <li class="nav-item">
              <a class="nav-link text-color-primary" data-bs-toggle="collapse" data-bs-target="#menu_item1" href="#"
                > <span style="padding: 4px 8px;"><i class="bi bi-app-indicator"></i></span>
                {"Applications"}
                <i class="bi small bi-caret-down-fill float-end"></i>
                </a>
              <ul id="menu_item1" class="submenu collapse" data-bs-parent="#nav_accordion"
                style="list-style-type:none; font-weight: 400;">
                <li>
                  <Anchor
                    route=AppRoute::ApplicationHome
                    classes="text-decoration-none"
                  >
                    <a class="nav-link" href="#"
                      style=" padding: 4px 8px; font-size: 15px; color: #65676e;">{"Applications"}
                    </a>
                  </Anchor>
                </li>
                <li>
                  <Anchor
                    route=AppRoute::ApisHome{ tenant_id: tenant_id }
                    classes="text-decoration-none"
                  >
                    <a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">
                      {"APIs"}
                    </a>
                  </Anchor>
                </li>
                <li>
                <Anchor
                    route=AppRoute::SsoHome
                    classes="text-decoration-none"
                  >
                    <a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"SSO Integrations "}</a>
                  </Anchor>
                </li>
              </ul>
            </li>

            <li class="nav-item">
              <a class="nav-link text-color-primary" data-bs-toggle="collapse" data-bs-target="#menu_item2" href="#"
                > <span style="padding: 4px 8px;"><i class="bi bi-file-lock2"></i></span>
                {"Authentication"} <i
                  class="bi small bi-caret-down-fill  float-end"></i> </a>
              <ul id="menu_item2" class="submenu collapse" data-bs-parent="#nav_accordion"
                style="list-style-type:none; font-weight: 400">
                <li>
                  <Anchor
                    route=AppRoute::DatabaseHome
                    classes="text-decoration-none"
                  >
                    <a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Database"}</a>
                  </Anchor>
                </li>
                <li>
                  <Anchor
                    route=AppRoute::SocialHome
                    classes="text-decoration-none"
                  >
                    <a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">
                      {"Social"}
                    </a>
                  </Anchor>
                </li>
                <li>
                  <Anchor
                    route=AppRoute::EnterpriseHome
                    classes="text-decoration-none"
                  >
                    <a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">
                      {"Enterprise"}
                    </a>
                  </Anchor>
                </li>
                <li>
                  <Anchor
                    route=AppRoute::AuthPasswordless
                    classes="text-decoration-none"
                  >
                  <a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Passwordless"}
                  </a>
                  </Anchor>
                </li>
                <li><a class="nav-link" href="#"
                    style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Authentication Profile"} </a>
                </li>
              </ul>
            </li>
            <li class="nav-item">
              <a class="nav-link text-color-primary" href="#"> <span style="padding: 4px 8px;"><i
                    class="bi bi-building"></i></span> {"Organizations"} </a>
            </li>
            <li class="nav-item">
              <a class="nav-link text-color-primary" data-bs-toggle="collapse" data-bs-target="#menu_item3" href="#"
                > <span style="padding: 4px 8px;"><i class="bi bi-person-check"></i></span> {"User
                Management"} <i
                  class="bi small bi-caret-down-fill float-end"></i> </a>
              <ul id="menu_item3" class="submenu collapse" data-bs-parent="#nav_accordion"
                style="list-style-type:none; font-weight: 400">
                <li>
                  <Anchor
                    route=AppRoute::UsersManagement
                    classes="text-decoration-none"
                  >
                  <a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Users"} </a>
                  </Anchor>
                </li>
                <li>
                  <Anchor
                    route=AppRoute::RolesCreated
                    classes="text-decoration-none"
                  >
                  <a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Roles"} </a>
                  </Anchor>
                </li>
              </ul>
            </li>
            <li class="nav-item">
              <a class="nav-link text-color-primary" data-bs-toggle="collapse" data-bs-target="#menu_item4" href="#"
                > <span style="padding: 4px 8px;"><i class="bi bi-pie-chart"></i></span> {"Branding"}
                <i
                  class="bi small bi-caret-down-fill float-end"></i> </a>
              <ul id="menu_item4" class="submenu collapse" data-bs-parent="#nav_accordion"
                style="list-style-type:none; font-weight: 400">
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Universal Login"} </a></li>
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Custom Domains"} </a></li>
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Email Templates"} </a> </li>
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Email Provider"} </a> </li>
              </ul>
            </li>
            <li class="nav-item">
              <a class="nav-link text-color-primary" data-bs-toggle="collapse" data-bs-target="#menu_item5" href="#"
                > <span style="padding: 4px 8px;"><i class="bi bi-shield-check"></i></span>
                {"Security"}
                <i
                  class="bi small bi-caret-down-fill float-end"></i> </a>
              <ul id="menu_item5" class="submenu collapse" data-bs-parent="#nav_accordion"
                style="list-style-type:none; font-weight: 400">
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Attack Protection"} </a></li>
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Multi-factor Auth "}</a></li>
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Monitoring"}</a> </li>
              </ul>
            </li>
            <li class="nav-item">
              <a class="nav-link text-color-primary" data-bs-toggle="collapse" data-bs-target="#menu_item6" href="#"
                > <span style="padding: 4px 8px;"><i class="bi bi-exclude"></i></span> {"Actions"}<i class="bi small bi-caret-down-fill float-end"></i> </a>
              <ul id="menu_item6" class="submenu collapse" data-bs-parent="#nav_accordion"
                style="list-style-type:none; font-weight: 400">
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Flows"} </a>
                </li>
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Custom"} </a>
                </li>
              </ul>
            </li>
            <li class="nav-item">
              <a class="nav-link text-color-primary" data-bs-toggle="collapse" data-bs-target="#menu_item7" href="#"
                > <span style="padding: 4px 8px;"><i class="bi bi-arrow-left-right"></i></span>
                {"Auth Pipeline"} <i
                  class="bi small bi-caret-down-fill float-end"></i> </a>
              <ul id="menu_item7" class="submenu collapse" data-bs-parent="#nav_accordion"
                style="list-style-type:none; font-weight: 400">
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Rules"} </a>
                </li>
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Hooks"} </a>
                </li>
              </ul>
            </li>
            <li class="nav-item">
              <a class="nav-link text-color-primary" data-bs-toggle="collapse" data-bs-target="#menu_item8" href="#"
                > <span style="padding: 4px 8px;"><i class="bi bi-tv"></i></span>
                {"Monitoring"} <i
                  class="bi small bi-caret-down-fill float-end"></i> </a>
              <ul id="menu_item8" class="submenu collapse" data-bs-parent="#nav_accordion"
                style="list-style-type:none; font-weight: 400">
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Log"} </a>
                </li>
                <li><a class="nav-link" href="#" style=" padding: 4px 8px; font-size: 15px; color: #65676e">{"Streams"} </a>
                </li>
              </ul>
            </li>

            <li class="nav-item">
              <Anchor
                route=AppRoute::SettingsHome
                classes="text-decoration-none"
              >
                <a class="nav-link text-color-primary" href="#">
                  <span style="padding: 4px 8px;">
                    <i class="bi bi-gear"></i>
                  </span>
                  {"Settings"}
                </a>
              </Anchor>
            </li>
            
          </ul>
          <hr/>
        </div>
      </div>
    }
  }
}
