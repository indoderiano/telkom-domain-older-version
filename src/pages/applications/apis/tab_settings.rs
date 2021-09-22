use yew::{
    prelude::*,
    services::ConsoleService,
};
use crate::types::api::{ ApiDetails };


#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ApisTabSettingsProps {
    pub api_details: ApiDetails,
}

enum Data {
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
}

pub enum Msg {
    InputText(String, Data),
    Save,
}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = ApisTabSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info(&format!("Api Tab Settings props, api details = {:?}", props.api_details));

        TabSettings {
            api_details: props.api_details,
            link,
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
                _ => {
                  ()
                }
              }
              true
            }
            Msg::Save => {
              ConsoleService::info(&format!("{:?}", self.api_details));
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
                                      type="text"
                                      class="form-control bg-input-grey"
                                      aria-label="Dollar amount (with dot and two decimal places)"
                                      value={token_exp.to_string()}
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
                                />
                              </div>
                              <p>
                                  {"If this setting is enabled, Auth0 will allow applications to ask for Refresh Tokens for this API."}
                              </p>
                          </div>
          
                          <button
                            type="button"
                            class="btn btn-primary mb-5 mt-3"
                            onclick=self.link.callback(|_| Msg::Save)
                          >
                            {"Save"}
                          </button>
          
          
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
                          <button
                              type="button"
                              class="btn btn-danger"
                          >{"Delete"}</button>
                      </div>
                  </div>
              </div>
          </div>
        }
    }
}
