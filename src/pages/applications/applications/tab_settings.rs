use yew::prelude::*;
pub struct TabSettings {}
pub enum Msg {}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TabSettings {}
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
                  aria-label="Dollar amount (with dot and two decimal places)" value={"name"}
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Domain"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={"domain"}
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Client ID"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={"client_id"}
                  />
              </div>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Client Secret"}
              </p>
              <div class="input-group mb-2">
                <input type="text" class="form-control bg-input-grey"
                  aria-label="Dollar amount (with dot and two decimal places)" value={"client_secret"}
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
                <textarea class="form-control" rows="4" placeholder="Add a description in less than 140 character"></textarea>
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
                  aria-label="logo-path" value={"https://path.to/my_logo.png"}
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
                  <option selected=true>{"Custom App"}</option>
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
                //   value={https://myapp.org/login }
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
                <textarea class="form-control" rows="4"></textarea>
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
                  placeholder="Add a description in less than 140 character"></textarea>
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
                  placeholder="Add a description in less than 140 character"></textarea>
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
                  placeholder="Add a description in less than 140 character"></textarea>
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
              <input type="number" class="form-control" min="1" value="36000" width="50px" />
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
                checked=true
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
              <input type="number" class="form-control" min="1" value="0" width="50px" />
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
                <input class="form-check-input" type="checkbox" checked=true/>
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
              <input type="number" class="form-control col-lg-8" min="1" value="0" width="50px" />
              <p class="text-color-disabled">
                {"Sets the absolute lifetime of a refresh_token (in seconds)."}
              </p>
            </div>

            <div class="mb-4">
              <p class="mb-2 fw-bold">
                {"Inactivity Expiration"}
              </p>
              <div class="form-check form-switch fs-3 mb-4">
                <input class="form-check-input" type="checkbox" checked=true />
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
              <input type="number" class="form-control col-lg-8" min="1" value="0" width="50px" />
              <p class="text-color-disabled">
                {"Sets the inactivity lifetime of a refresh_token (in seconds)."}
              </p>
            </div>



          </div>
        </div>

        <div class="row border-bottom mt-5">

            <div class="mb-5 mt-3">
              <div class="col-md-12 p-2 text-center">
                <button type="button" class="btn btn-primary">{"Save Changes"} </button>
              </div>

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
          <button type="button" class="btn btn-danger m-auto p-2">{"Delete"}</button>
            
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