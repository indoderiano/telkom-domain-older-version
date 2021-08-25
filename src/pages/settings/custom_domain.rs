use yew::prelude::*;
// use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;

pub struct SettingsCustomDomain {}

impl Component for SettingsCustomDomain {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        SettingsCustomDomain {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                // class="mx-auto"
                // style="max-width: 1048px;"
            >
                <div
                    class="mb-3"
                >
                    <p>
                        {"Using a custom domain with Auth0 allows you to maintain a consistent user experience. Your users will remain in your domain instead of dev-ofzd5p1b.au.auth0.com. For more information about custom domains, please refer to our documentation."}                        
                    </p>
                </div>


                <div class="alert alert-info mb-5" role="alert">
                    <i class="bi bi-info-circle me-2"></i>
                    {"This feature is not available for free plans. To configure a custom domain, please upgrade your account to any paid plan."}
                </div>
                
            </div>
        }
    }
}
