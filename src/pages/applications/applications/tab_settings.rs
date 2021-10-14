use yew::{
    prelude::*,
    format::{ Json, Nothing },
    services::{
        ConsoleService,
        fetch::{FetchService, FetchTask, Request, Response},
    }
};
use crate::app::AppRoute;
use yew_router::service::RouteService;
use crate::types::{
	application::{ AppDetails, ResponseAppDetails },
	ResponseMessage,
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct AppsTabSettingsProps {
    pub app_details: AppDetails,
}

pub enum StateError {
    Update,
    Delete,
}

pub enum Data {
    Tenant,
    Name,
    Domain,
    ClientId,
    ClientSecret,
    Description,
    AppLogo,
    AppType,
    AuthenticationMethod,
    LoginUrl,
    AllowedUrls ,
    AllowedLogoutUrls ,
    AllowedWebOrigins,
    AllowedOrigins,
    TokenExp,
    RefreshTokenRotation,
    RefreshTokenRotationInterval,
    RefeshTokenAbsoluteExpiration,
    RefeshTokenAbsoluteExpirationLifetime,
    RefeshTokenInactivityExpiration,
    RefeshTokenInactivityExpirationLifetime,
}

pub struct TabSettings {
  app_details: AppDetails,
  link: ComponentLink<Self>,
  fetch_task: Option<FetchTask>,
  loading_update_app: bool,
  error_update_app: Option<String>,
  loading_delete_app: bool,
  error_delete_app: Option<String>,
  route_service: RouteService,
}

pub enum Msg {
  InputText(String, Data),
  Save,
  GetAppDetails(AppDetails),
  ResponseError(String, StateError),
  Delete,
  RedirectToApp,
}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = AppsTabSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TabSettings {
          app_details: props.app_details,
          link,
          fetch_task: None,
          loading_update_app: false,
          error_update_app: None,
          loading_delete_app: false,
          error_delete_app: None,
          route_service: RouteService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
        Msg::InputText(input, data) => {
          match data {
            Data::Tenant => {
                self.app_details.tenant = input;
            }
            Data::Name => {
                self.app_details.name = input;
            }
            Data::Domain => {
                self.app_details.domain = input;
            }
            Data::ClientId => {
                self.app_details.client_id = input;
            }
            Data::ClientSecret => {
                self.app_details.client_secret = input;
            }
            Data::Description => {
                self.app_details.description = input;
            }
            Data::AppLogo => {
                self.app_details.app_logo = input;
            }
            Data::AppType => {
                self.app_details.app_type = input;
            }
            Data::AuthenticationMethod => {
                self.app_details.authentication_method = input;
            }
            Data::LoginUrl => {
                self.app_details.login_url = input;
            }
            Data::AllowedUrls => {
                self.app_details.allowed_urls = input;
            }
            Data::AllowedLogoutUrls => {
                self.app_details.allowed_logout_urls = input;
            }
            Data::AllowedWebOrigins => {
                self.app_details.allowed_web_origins = input;
            }
            Data::AllowedOrigins => {
                self.app_details.allowed_origins = input;
            }
            Data::TokenExp => {
                if input.is_empty() {
                    self.app_details.token_exp = 36000;
                } else {
                    self.app_details.token_exp = input.parse::<u32>().unwrap();
                }
            }
            Data::RefreshTokenRotation => {
                self.app_details.refresh_token_rotation = !self.app_details.refresh_token_rotation;
            }
            Data::RefreshTokenRotationInterval => {
                if input.is_empty() {
                    self.app_details.refresh_token_rotation_interval = 0;
                } else {
                    self.app_details.refresh_token_rotation_interval = input.parse::<i32>().unwrap();
                }
            }
            Data::RefeshTokenAbsoluteExpiration => {
                self.app_details.refesh_token_absolute_expiration = !self.app_details.refesh_token_absolute_expiration;
            }
            Data::RefeshTokenAbsoluteExpirationLifetime => {
                if input.is_empty() {
                    self.app_details.refesh_token_absolute_expiration_lifetime = 2592000;
                } else {
                    self.app_details.refesh_token_absolute_expiration_lifetime = input.parse::<i32>().unwrap();
                }
            }
            Data::RefeshTokenInactivityExpiration => {
                self.app_details.refesh_token_inactivity_expiration = !self.app_details.refesh_token_inactivity_expiration;
            }
            Data::RefeshTokenInactivityExpirationLifetime => {
                if input.is_empty() {
                    self.app_details.refesh_token_inactivity_expiration_lifetime = 1296000;
                } else {
                    self.app_details.refesh_token_inactivity_expiration_lifetime = input.parse::<i32>().unwrap();
                }
            }
            _ => {
              ()
            }
          }
          true
        }
        Msg::Save => {
          ConsoleService::info(&format!("{:?}", self.app_details));
          let request = Request::put("http://localhost:3000/applications/tenantid/applications/dev-1wj84p4q")
              .header("Content-Type", "application/json")
              .header("access_token", "tokenidtelkomdomain")
              .body(Json(&self.app_details))
              .expect("Could not build request.");
          let callback = self.link.callback(|response: Response<Json<Result<ResponseAppDetails, anyhow::Error>>>| {
          let Json(data) = response.into_body();
          match data {
              Ok(dataok) => {
                  ConsoleService::info(&format!("{:?}", dataok));
                  Msg::GetAppDetails(dataok.data)
              }
              Err(error) => {
                  ConsoleService::info(&error.to_string());
                  Msg::ResponseError(error.to_string(), StateError::Update)
              }
          }
          });
          let task = FetchService::fetch(request, callback).expect("failed to start request");
          self.loading_update_app = true;
          self.fetch_task = Some(task);
          true
        }
        Msg::GetAppDetails(data) => {
          self.fetch_task = None;
          self.loading_update_app = false;
          self.app_details = data;
          true
        }
        Msg::ResponseError(message, state) => {
          match state {
            StateError::Update => {
                self.fetch_task = None;
                self.loading_update_app = false;
                self.error_update_app = Some(message);
            }
            StateError::Delete => {
                    self.fetch_task = None;
                    self.loading_delete_app = false;
                    self.error_delete_app = Some(message);
            }
          }
          true
        }
        Msg::Delete => {
          let request = Request::delete("http://localhost:3000/applications/tenantid/applications/dev-1wj84p4q")
              .header("Content-Type", "application/json")
              .header("access_token", "tokenidtelkomdomain")
              .body(Nothing)
              .expect("Could not build request.");
          let callback = self.link.callback(|response: Response<Json<Result<ResponseMessage, anyhow::Error>>>| {
          let Json(data) = response.into_body();
          match data {
              Ok(dataok) => {
                  ConsoleService::info(&format!("{:?}", dataok));
                  Msg::RedirectToApp
              }
              Err(error) => {
                  ConsoleService::info(&error.to_string());
                  Msg::ResponseError(error.to_string(), StateError::Delete)
              }
          }
          });
          let task = FetchService::fetch(request, callback).expect("failed to start request");
          self.loading_delete_app = true;
          self.fetch_task = Some(task);
          true
        }
        Msg::RedirectToApp => {
          self.loading_delete_app = false;
          self.fetch_task = None;
          self.route_service.set_route(&format!("/{}/apis", self.app_details.tenant), ());
          true
        }
      }
    }
        

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
      let AppDetails {
            tenant: _,
            name,
            domain,
            client_id,
            client_secret,
            description,
            app_logo,
            app_type,
            authentication_method: _,
            login_url,
            allowed_urls,
            allowed_logout_urls,
            allowed_web_origins,
            allowed_origins,
            token_exp,
            refresh_token_rotation,
            refresh_token_rotation_interval,
            refesh_token_absolute_expiration,
            refesh_token_absolute_expiration_lifetime,
            refesh_token_inactivity_expiration,
            refesh_token_inactivity_expiration_lifetime,
        } = self.app_details.clone();
        html! {
            <>
                <div>

      <div class="container border rounded p-4 d-flex flex-column mb-5" style="font-size: 14px;">
        <div class="row border-bottom">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Basic Information"}
          </div>
          <div class="col-lg-6">
            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Name*"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={name}
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Domain"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={domain}
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Client ID"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={client_id}
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Client Secret"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={client_secret}
                  />
              </div>
              <p>
                {"The Client Secret is not base64 encoded."}
              </p>
            </div>

            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Description"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4" placeholder="Add a description in less than 140 character">{description}</textarea>
              </div>
              <p class="text-color-disabled">
                {"A free text description of the application. Max character count is 140."}
              </p>
            </div>

          </div>
        </div>



        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Application Properties"}
          </div>
          <div class="col-lg-6">
            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Application Logo"}
              </p>
              <div class="col-md-12 p-2 text-center border rounded p-5"><img height="65px" width="60px"
                  src="https://cdn.auth0.com/manhattan/versions/1.3431.0/assets/./badge.png"/></div>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="logo-path" value={app_logo}
                  />
              </div>
              <p>
                {"The URL of the logo to display for the application, if none is set the default badge for this type of
                application will be shown. Recommended size is 150x150 pixels."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Application Type"}
              </p>
              <div class="input-group mb-3">
                <select class="form-select" id="application_type">
                  <option selected=true>{app_type}</option>
                  <option value="1">{"Single Web Application"}</option>
                  <option value="2">{"Machine to Machine"}</option>
                  <option value="3">{"Native"}</option>
                </select>
              </div>
              <p>
                {"The type of application will determine which settings you can configure from the dashboard."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Token Endpoint Authentication Method"}
              </p>
              <div class="input-group mb-3">
                <select class="form-select" id="application_type" disabled=true>
                  <option selected=true disabled=true>{"None"}</option>
                </select>
              </div>
              <p>
                {"Defines the requested authentication method for the token endpoint. Possible values are 'None' (public
                application without a client secret), 'Post' (application uses HTTP POST parameters) or 'Basic'
                (application uses HTTP Basic)."}
              </p>
            </div>

          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Application URIs"}
          </div>
          <div class="col-lg-6">
            
            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Application Login URI"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" 
                  value={login_url}
                  />
              </div>
              <p>
                {"In some scenarios, Auth0 will need to redirect to your application’s login page. This URI needs to
                point to a route in your application that should redirect to your tenant’s /authorize endpoint"}
              </p>
            </div>


            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Allowed Callback URLs"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4">{allowed_urls}</textarea>
              </div>
              <p class="text-color-disabled">
                {"After the user authenticates we will only call back to any of these URLs. You can specify multiple
                valid URLs by comma-separating them (typically to handle different environments like QA or testing).
                Make sure to specify the protocol (https://) otherwise the callback may fail in some cases. With the
                exception of custom URI schemes for native clients, all callbacks should use protocol https://. You can
                use Organization URL parameters in these URLs."}
              </p>
            </div>

            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Allowed Logout URLs"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4"
                  placeholder="Add a description in less than 140 character">{allowed_logout_urls}</textarea>
              </div>
              <p class="text-color-disabled">
                {"A set of URLs that are valid to redirect to after logout from Auth0. After a user logs out from Auth0
                you can redirect them with the returnTo query parameter. The URL that you use in returnTo must be listed
                here. You can specify multiple valid URLs by comma-separating them. You can use the star symbol as a
                wildcard for subdomains (*.google.com). Query strings and hash information are not taken into account
                when validating these URLs."}
              </p>
            </div>

            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Allowed Web Origins"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4"
                  placeholder="Add a description in less than 140 character">{allowed_web_origins}</textarea>
              </div>
              <p class="text-color-disabled">
                {"Comma-separated list of allowed origins for use with Cross-Origin Authentication, Device Flow, and web
                message response mode, in the form of <scheme> // <host> [ : <port> ], such as
                https://login.mydomain.com or http://localhost:3000. You can use wildcards at the subdomain level
                (e.g.: https://*.contoso.com). Query strings and hash information are not taken into account when
                validating these URLs."}
              </p>
            </div>

            <div class="mb-5">
              <p class="mb-2 fw-bold">
                {"Allowed Origins (CORS)"}
              </p>
              <div class="input-group mb-2">
                <textarea class="form-control" rows="4"
                  placeholder="Add a description in less than 140 character">{allowed_origins}</textarea>
              </div>
              <p class="text-color-disabled">
                {"Allowed Origins are URLs that will be allowed to make requests from JavaScript to Auth0 API (typically
                used with CORS). By default, all your callback URLs will be allowed. This field allows you to enter
                other origins if you need to. You can specify multiple valid URLs by comma-separating them or one by
                line, and also use wildcards at the subdomain level (e.g.: https://*.contoso.com). Query strings and
                hash information are not taken into account when validating these URLs.. You can use Organization URL
                placeholders in these URLs."}
              </p>
            </div>

          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"ID TOKEN"}
          </div>
          <div class="col-lg-6">
            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"ID Token Expiration "}
              </p>
              <input type="number" class="form-control" min="1" value={token_exp.to_string()}width="50px" />
              <p class="text-color-disabled">
                {"This setting allows you to set the lifetime of the id_token (in seconds)"}
              </p>
            </div>
          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Refresh Token Rotation"}
          </div>
          <div class="col-lg-6">

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Rotation"}
              </p>
              <div class="form-check form-switch fs-3 mb-4">
                <input class="form-check-input" type="checkbox" 
                checked={refresh_token_rotation}
                />
              </div>
              <p class="text-color-disabled">
                {"When enabled, as a result of exchanging a refresh token, a new refresh token will be issued and the
                existing token will be invalidated. This allows for automatic detection of token reuse if the token is
                leaked. In addition, an absolute expiration lifetime must be set."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Reuse Interval"}
              </p>
              <input type="number" class="form-control" min="1" value={refresh_token_rotation_interval.to_string()} width="50px" />
              <p class="text-color-disabled">
                {"The allowable leeway time that the same refresh_token can be used to request an access_token without
                triggering automatic reuse detection."}
              </p>
            </div>
          </div>
        </div>

        <div class="row border-bottom mt-5">
          <div class="col-lg-6 text-color-primary fw-bold mb-4">
            {"Refresh Token Expiration"}
          </div>
          <div class="col-lg-6">

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Absolute Expiration"}
              </p>
              <div class="form-check form-switch fs-3 mb-4">
                <input class="form-check-input" type="checkbox" checked={refesh_token_absolute_expiration}/>
              </div>
              <p class="text-color-disabled">
                {"When enabled, a refresh_token will expire based on an absolute lifetime, after which the token can no
                longer be used. If rotation is enabled, an expiration lifetime must be set."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Absolute Lifetime"}
              </p>
              <input type="number" class="form-control col-lg-8" min="1" value={refesh_token_absolute_expiration_lifetime.to_string()} width="50px" />
              <p class="text-color-disabled">
                {"Sets the absolute lifetime of a refresh_token (in seconds)."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Inactivity Expiration"}
              </p>
              <div class="form-check form-switch fs-3 mb-4">
                <input class="form-check-input" type="checkbox" checked={refesh_token_inactivity_expiration} />
              </div>
              <p class="text-color-disabled">
                {"When enabled, a refresh_token will expire based on a specified inactivity lifetime, after which the
                token can no longer be used."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Inactivity Lifetime"}
              </p>
              <input type="number" class="form-control col-lg-8" min="1" value={refesh_token_inactivity_expiration_lifetime.to_string()} width="50px" />
              <p class="text-color-disabled">
                {"Sets the inactivity lifetime of a refresh_token (in seconds)."}
              </p>
            </div>

          </div>
        </div>

        <div class="row border-bottom mt-5">

            <div class="mb-5 mt-3">
              <div class="col-md-12 p-2 text-center">
                <button
                  type="button"
                  class=format!("btn {} btn-primary position-relative", if self.loading_update_app {"loading"} else {""} )
                  onclick=self.link.callback(|_| Msg::Save)
                  disabled={ if self.loading_update_app {true} else {false} }
                >
                  <div class="telkom-label">
                    {"Save Changes"}
                  </div>
                  <div class="telkom-spinner telkom-center">
                    <div class="spinner-border spinner-border-sm" role="status"/>
                  </div>
                </button>
              </div>

              {
                if self.error_update_app.is_some() {
                  html! {
                    <div class="alert alert-warning" role="alert">
                        <i class="bi bi-exclamation-triangle me-2"></i>
                        { self.error_update_app.clone().unwrap() }
                    </div>
                  }
                } else {
                    html! {}
                }
              }

            </div>
          </div>
        </div>

      

      <div style="font-size: 14px;">
        <p class="fw-bold">
          {"Danger Zone"}
        </p>

        <div class="alert alert-danger d-flex flex-row justify-content-between" role="alert">
          <div>
            <p class="fw-bold">
              {"Delete This Application"}
            </p>
            {"Once confirmed, this operation can't be undone!"}
          </div>
          <div>
          <button
            type="button"
            class=format!("btn {} btn-danger position-relative", if self.loading_delete_app {"loading"} else {""} )
            onclick=self.link.callback(|_| Msg::Delete)
            disabled={ if self.loading_delete_app {true} else {false} }
        >
            <div class="telkom-label">
                {"Delete"}
            </div>
            <div class="telkom-spinner telkom-center">
                <div class="spinner-border spinner-border-sm" role="status"/>
            </div>
        </button>
            {
                if self.error_delete_app.is_some() {
                    html! {
                        <div class="alert alert-warning" role="alert">
                            <i class="bi bi-exclamation-triangle me-2"></i>
                            { self.error_delete_app.clone().unwrap() }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            
          </div>
        </div>
        
        <div class="alert alert-danger d-flex flex-row justify-content-between" role="alert">
          <div>
            <p class="fw-bold">
              {"Rotate secret"}
            </p>
            {"All authorized apps will need to be updated with the new client secret."}
          </div>
          <div>
            <button type="button" class="btn btn-danger m-auto p-2">{"Delete"}</button>

          </div>
        </div>




      </div>
    </div>
                
            </>
    }
  }
}