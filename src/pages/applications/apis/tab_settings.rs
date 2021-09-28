use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use yew_router::service::RouteService;
use crate::types::{
	api::{ ApiDetails, ResponseApiDetails },
	ResponseMessage,
};


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ApisTabSettingsProps {
    pub api_details: ApiDetails,
}

pub enum StateError {
    Update,
    Delete,
}

pub enum Data {
    ApiId,
    Name,
    Identifier,
    TokenExp,
    TokenExpBrowser,
    SignAlg,
    Rbac,
    PermissionAccToken,
    AllowSkipUser,
    AllowOffAcc,
}

pub struct TabSettings {
    api_details: ApiDetails,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    loading_update_api: bool,
	error_update_api: Option<String>,
    loading_delete_api: bool,
    error_delete_api: Option<String>,
    route_service: RouteService,
}

pub enum Msg {
    InputText(String, Data),
    Save,
    GetApiDetails(ApiDetails),
    ResponseError(String, StateError),
    Delete,
    RedirectToApi,
}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = ApisTabSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Api Tab Settings props, api details = {:?}", props.api_details));

        TabSettings {
            api_details: props.api_details,
            link,
            fetch_task: None,
            loading_update_api: false,
		    error_update_api: None,
            loading_delete_api: false,
            error_delete_api: None,
            route_service: RouteService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputText(input, data) => {
              match data {
                Data::ApiId => {
                    self.api_details.api_id = input;
                }
                Data::Name => {
                    self.api_details.name = input;
                }
                Data::Identifier => {
                    self.api_details.identifier = input;
                }
                Data::TokenExp => {
                    if input.is_empty() {
                        self.api_details.token_exp = 0;
                    } else {
                        self.api_details.token_exp = input.parse::<u32>().unwrap();
                    }
                }
                Data::TokenExpBrowser => {
                    if input.is_empty() {
                        self.api_details.token_exp_browser = 0;
                    } else {
                        self.api_details.token_exp_browser = input.parse::<u32>().unwrap();
                    }
                }
                Data::SignAlg => {
                    self.api_details.sign_algorithm = input;
                }
                Data::Rbac => {
                    self.api_details.rbac = !self.api_details.rbac;
                }
                Data::PermissionAccToken => {
                    self.api_details.permission_acc_token = !self.api_details.permission_acc_token;
                }
                Data::AllowSkipUser => {
                    self.api_details.allow_skip_user = !self.api_details.allow_skip_user;
                }
                Data::AllowOffAcc => {
                    self.api_details.allow_off_acc = !self.api_details.allow_off_acc;
                }
                _ => {
                  ()
                }
              }
              true
            }
            Msg::Save => {
                ConsoleService::info(&format!("{:?}", self.api_details));
                let request = Request::put("http://localhost:3000/api/dev-ofzd5p1b/apis/60daccd6dff9a6003e8ef6ef")
                    .header("Content-Type", "application/json")
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Json(&self.api_details))
                    .expect("Could not build request.");
                let callback = self.link.callback(|response: Response<Json<Result<ResponseApiDetails, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                match data {
                    Ok(dataok) => {
                        ConsoleService::info(&format!("{:?}", dataok));
                        Msg::GetApiDetails(dataok.data)
                    }
                    Err(error) => {
                        ConsoleService::info(&error.to_string());
                        Msg::ResponseError(error.to_string(), StateError::Update)
                    }
                }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_update_api = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::GetApiDetails(data) => {
                self.fetch_task = None;
                self.loading_update_api = false;
                self.api_details = data;
                true
            }
            Msg::ResponseError(message, state) => {
                match state {
                    StateError::Update => {
                        self.fetch_task = None;
                        self.loading_update_api = false;
                        self.error_update_api = Some(message);
                    }
                    StateError::Delete => {
                        self.fetch_task = None;
                        self.loading_delete_api = false;
                        self.error_delete_api = Some(message);
                    }
                }
                true
            }
            Msg::Delete => {
                let request = Request::delete("http://localhost:3000/api/dev-ofzd5p1b/apis/60daccd6dff9a6003e8ef6ef")
                    // .header("Content-Type", "application/json")
                    .header("access_token", "tokenidtelkomdomain")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(|response: Response<Json<Result<ResponseMessage, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                match data {
                    Ok(dataok) => {
                        ConsoleService::info(&format!("{:?}", dataok));
                        Msg::RedirectToApi
                    }
                    Err(error) => {
                        ConsoleService::info(&error.to_string());
                        Msg::ResponseError(error.to_string(), StateError::Delete)
                    }
                }
                });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.loading_delete_api = true;
                self.fetch_task = Some(task);
                true
            }
            Msg::RedirectToApi => {
                self.loading_delete_api = false;
                self.fetch_task = None;
                self.route_service.set_route(&format!("/{}/apis", self.api_details.tenant_id), ());
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let ApiDetails {
            id: _,
            name,
            api_id,
            api_type: _,
            identifier,
            token_exp,
            token_exp_browser,
            sign_algorithm,
            rbac,
            permission_acc_token,
            allow_skip_user,
            allow_off_acc,
            tenant_id: _,
        } = self.api_details.clone();
        html! {
            <div>

              <div
                  class="container border rounded p-4 d-flex flex-column mb-5"
                  style="font-size: 14px;"
              >
                  <div
                      class="row border-bottom"
                  >
                      <div
                          class="col-lg-6 text-color-primary fw-bold mb-4"
                      >
                          {"General Settings"}
                      </div>
                      <div
                          class="col-lg-6"
                      >
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"ID"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="text"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={api_id}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::ApiId))
                                  />   
                              </div>
                              <p>
                                  {"The API id on our system. Useful if you prefer to work directly with Auth0's Management API instead."}
                              </p>
                          </div>
          
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Name*"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="text"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={name}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::Name))
                                  />   
                              </div>
                              <p>
                                  {"A friendly name for the API. The following characters are not allowed "}
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      <i class="bi bi-chevron-left"></i>
                                  </code>
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      <i class="bi bi-chevron-right"></i>
                                  </code>
                              </p>
                          </div>
          
                          <div
                              class="mb-5"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Identifier"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="text"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={identifier}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::Identifier))
                                  />
                              </div>
                              <p>
                                  {"Unique identifier for the API. This value will be used as the "}
                                  <code
                                      style="padding: 2px 6px; font-size: 14px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      {"audience"}
                                  </code>
          
                                  {" parameter on authorization calls."}
                              </p>
                          </div>
          
                      </div>
                  </div>
          
          
                  
                  <div
                      class="row border-bottom mt-5"
                  >
                      <div
                          class="col-lg-6 text-color-primary fw-bold mb-4"
                      >
                          {"Tokens Settings"}
                      </div>
                      <div
                          class="col-lg-6"
                      >
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Token Expirations (Seconds)*"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="number"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={token_exp.to_string()}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::TokenExp))
                                  />
                              </div>
                              <p>
                                  {"Expiration value (in seconds) for "}
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      {"access tokens"}
                                  </code>
                                  {" issued for this API from the Token Endpoint."}
                              </p>
                          </div>
          
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Token Expirations For Browser Flows (Seconds)*"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="text"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={token_exp_browser.to_string()}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::TokenExpBrowser))
                                  />
                              </div>
                              <p>
                                  {"Expiration value (in seconds) for "}
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      {"access tokens"}
                                  </code>
                                  {" issued for this API via Implicit or Hybrid Flows. Cannot be greater than the Token Lifetime value."}
                              </p>
                          </div>
          
                          <div
                              class="mb-5"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Signing Algorithm"}
                              </p>
                              <div class="input-group mb-2">
                                  <input
                                      type="text"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={sign_algorithm}
                                      oninput=self.link.callback(|data: InputData| Msg::InputText(data.value, Data::SignAlg))
                                  />
                              </div>
                              <p>
                                  {"Algorithm to be used when signing the "}
                                  <code
                                      style="padding: 2px 6px; font-size: 11px;"
                                      class="bg-input-grey text-color-primary"
                                  >
                                      {"access tokens"}
                                  </code>
          
                                  {" for this API. You can find more information"}
                              </p>
                          </div>
          
                      </div>
                  </div>
          
          
                  <div
                      class="row border-bottom mt-5"
                  >
                      <div
                          class="col-lg-6 text-color-primary fw-bold mb-4"
                      >
                          {"RBAC Settings"}
                      </div>
                      <div
                          class="col-lg-6"
                      >
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Enable RBAC "}
                              </p>
                              <div class="form-check form-switch fs-3 mb-4">
                                  <input
                                    class="form-check-input"
                                    type="checkbox"
                                    checked={rbac}
                                    onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::Rbac))
                                />
                              </div>
                              <p class="text-color-disabled">
                                  {"If this setting is enabled, RBAC authorization policies will be enforced for this API. Role and permission assignments will be evaluated during the login transaction."}
                              </p>
                          </div>
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Add Permissions in the Access Token"}
                              </p>
                              <div class="form-check form-switch fs-3 mb-4">
                                  <input
                                    class="form-check-input"
                                    type="checkbox"
                                    checked={permission_acc_token}
                                    onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::PermissionAccToken))
                                />
                              </div>
                              <p class="text-color-disabled">
                                  {"If this setting is enabled, the Permissions claim will be added to the access token. Only available if RBAC is enabled for this API."}
                              </p>
                          </div>
                      </div>
                  </div>


                  <div
                      class="row border-bottom mt-5"
                  >
                      <div
                          class="col-lg-6 text-color-primary fw-bold mb-4"
                      >
                          {"Access Settings"}
                      </div>
                      <div
                          class="col-lg-6"
                      >
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Allow Skipping User Consent"}
                              </p>
                              <div class="form-check form-switch fs-3 mb-4">
                                  <input
                                    class="form-check-input"
                                    type="checkbox"
                                    checked={allow_skip_user}
                                    onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::AllowSkipUser))
                                />
                              </div>
                              <p class="text-color-disabled">
                                  {"If this setting is enabled, this API will skip user consent for applications flagged as First Party."}
                              </p>
                          </div>
          
                          <div
                              class="mb-4"
                          >
                              <p class="mb-2 fw-bold">
                                  {"Allow Offline Access"}
                              </p>
                              <div class="form-check form-switch fs-3 mb-4">
                                  <input
                                    class="form-check-input"
                                    type="checkbox"
                                    id="flexSwitchCheckDefault"
                                    checked={allow_off_acc}
                                    onclick=self.link.callback(|_| Msg::InputText(String::from("none"), Data::AllowOffAcc))
                                />
                              </div>
                              <p>
                                  {"If this setting is enabled, Auth0 will allow applications to ask for Refresh Tokens for this API."}
                              </p>
                          </div>

                          <div
                            class="mb-5 mt-3"
                          >
                            <button
                              type="button"
                              class=format!("btn {} btn-primary position-relative", if self.loading_update_api {"loading"} else {""} )
                              onclick=self.link.callback(|_| Msg::Save)
                              disabled={ if self.loading_update_api {true} else {false} }
                            >
                              <div class="telkom-label">
                                {"Save"}
                              </div>
                              <div class="telkom-spinner telkom-center">
                                <div class="spinner-border spinner-border-sm" role="status"/>
                              </div>
                            </button>

                            {
                              if self.error_update_api.is_some() {
                                html! {
                                  <div class="alert alert-warning" role="alert">
                                      <i class="bi bi-exclamation-triangle me-2"></i>
                                      { self.error_update_api.clone().unwrap() }
                                  </div>
                                }
                              } else {
                                  html! {}
                              }
                            }

                          </div>
                      </div>
                  </div>
          
              </div>

              <div
                  style="font-size: 14px;"
              >
                  <p
                      class="fw-bold"
                  >
                      {"Danger Zone"}
                  </p>

                  <div class="alert alert-danger d-flex flex-row justify-content-between" role="alert">
                      <div>
                          <p
                              class="fw-bold"
                          >
                              {"Delete This API"}
                          </p>
                          {"Once confirmed, this operation can't be undone!"}
                      </div>
                      <div>
                        //   <button
                        //       type="button"
                        //       class="btn btn-danger"
                        //   >{"Delete"}</button>

                        <button
                            type="button"
                            class=format!("btn {} btn-danger position-relative", if self.loading_delete_api {"loading"} else {""} )
                            onclick=self.link.callback(|_| Msg::Delete)
                            disabled={ if self.loading_delete_api {true} else {false} }
                        >
                            <div class="telkom-label">
                                {"Delete"}
                            </div>
                            <div class="telkom-spinner telkom-center">
                                <div class="spinner-border spinner-border-sm" role="status"/>
                            </div>
                        </button>
                            {
                                if self.error_delete_api.is_some() {
                                    html! {
                                        <div class="alert alert-warning" role="alert">
                                            <i class="bi bi-exclamation-triangle me-2"></i>
                                            { self.error_delete_api.clone().unwrap() }
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                      </div>
                  </div>
              </div>
          </div>
        }
    }
}
