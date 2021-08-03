use yew::prelude::*;
// use yew_router::components::RouterAnchor;
// use crate::app::AppRoute;

pub struct CreateSso {}

pub enum Msg {}

impl Component for CreateSso {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CreateSso {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // type Anchor = RouterAnchor<Route>;
        html! {
        <>
            <div class="col py-3">
                <div>
                    <div class="mx-auto pt-5 pb-5 px-4" style="max-width: 1048px;">
                    <div class="mb-5">
                        <div class="d-flex flex-row mb-3">
                        <div class="flex-fill fs-3 fw-bold">
                            {"New Single Sign On Integration"}
                        </div>
                    </div>

                    <div class="input-group pt-4">
                        <span class="input-group-text" id="basic-addon1">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" 
                            class="bi bi-search" viewBox="0 0 16 16">
                            <path
                                d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z">
                            </path>
                        </svg>
                        </span>
                        <input type="text" class="form-control" id="ssoSearch" placeholder="Search for SSO Integrations"/>
                    </div>

                    <div class="py-5">
                        <div>
                            <div class="row hidden-md-up">
                                <div class="col-md-4">
                                <div class="card" 
                                    style="padding: 24px;
                                    margin-bottom: 24px;
                                            min-height: 160px;
                                            max-height: 160px;">
                                    <div class="card-block">
                                    <div>
                                        <div>
                                        <div
                                            style="
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            padding-bottom: 15px;
                                            "
                                        >
                                            <img
                                            src="https://cdn.auth0.com/marketplace/catalog/content/assets/creators/microsoft/microsoft-avatar.png"
                                            class="jss6144 jss6146"
                                            style="color: #65676e;
                                            margin: 0px;
                                            width: 2.5rem;
                                            height: 2.5rem;
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            justify-content: center;"
                                            />
                                            
                                            <h5 
                                            style="padding: 0px 15px;">{"AD RMS"}</h5>
                                            
                                        </div>
                                        </div>
                                        <div>
                                            <p
                                            style=" max-height: calc(3.2em);
                                            overflow: hidden;
                                            display: -webkit-box;
                                            -webkit-line-clamp: 2;
                                            -webkit-box-orient: vertical;"
                                            >{"Single sign-on for accounts using Active Directory RMS to secure their resources"}</p>
                                        </div>
                                    </div>
                                    </div>
                                </div>
                                </div>
                                <div class="col-md-4">
                                <div class="card" style="padding: 24px;
                                    margin-bottom: 24px;
                                            min-height: 160px;
                                            max-height: 160px;">
                                    <div class="card-block">
                                    <div>
                                        <div>
                                        <div style="
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            padding-bottom: 15px;
                                            ">
                                            <img
                                            src="https://cdn.auth0.com/marketplace/catalog/content/assets/creators/adobe/adobe-avatar.png"
                                            class="jss6144 jss6146" style="color: #65676e;
                                            margin: 0px;
                                            width: 2.5rem;
                                            height: 2.5rem;
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            justify-content: center;" />

                                            <h5 style="padding: 0px 15px;">{"Adobe Sign"}</h5>

                                        </div>
                                        </div>
                                        <div>
                                        <p style=" max-height: calc(3.2em);
                                            overflow: hidden;
                                            display: -webkit-box;
                                            -webkit-line-clamp: 2;
                                            -webkit-box-orient: vertical;">{"SSO for adobe Sign."}</p>
                                        </div>
                                    </div>
                                    </div>
                                </div>
                                </div>
                                <div class="col-md-4">
                                <div class="card" style="padding: 24px;
                                    margin-bottom: 24px;
                                            min-height: 160px;
                                            max-height: 160px;">
                                    <div class="card-block">
                                    <div>
                                        <div>
                                        <div style="
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            padding-bottom: 15px;
                                            ">
                                            <img
                                            src="https://cdn.auth0.com/marketplace/catalog/content/assets/creators/aha/aha-avatar.png"
                                            class="jss6144 jss6146" style="color: #65676e;
                                            margin: 0px;
                                            width: 2.5rem;
                                            height: 2.5rem;
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            justify-content: center;" />

                                            <h5 style="padding: 0px 15px;">{"Aha!"}</h5>

                                        </div>
                                        </div>
                                        <div>
                                        <p style=" max-height: calc(3.2em);
                                            overflow: hidden;
                                            display: -webkit-box;
                                            -webkit-line-clamp: 2;
                                            -webkit-box-orient: vertical;">{"The World's #1 Roadmap Software"}</p>
                                        </div>
                                    </div>
                                    </div>
                                </div>
                                </div>
                                <div class="col-md-4">
                                <div class="card" style="padding: 24px;
                                    margin-bottom: 24px;
                                            min-height: 160px;
                                            max-height: 160px;">
                                    <div class="card-block">
                                    <div>
                                        <div>
                                        <div style="
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            padding-bottom: 15px;
                                            ">
                                            <img
                                            src="https://cdn.auth0.com/marketplace/catalog/content/assets/creators/airbrake/airbrake-avatar.png"
                                            class="jss6144 jss6146" style="color: #65676e;
                                            margin: 0px;
                                            width: 2.5rem;
                                            height: 2.5rem;
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            justify-content: center;" />

                                            <h5 style="padding: 0px 15px;">{"Air Brake"}</h5>

                                        </div>
                                        </div>
                                        <div>
                                        <p style=" max-height: calc(3.2em);
                                            overflow: hidden;
                                            display: -webkit-box;
                                            -webkit-line-clamp: 2;
                                            -webkit-box-orient: vertical;">{"Frictionless Error Monitoring and Application
                                            Performance Management"}</p>
                                        </div>
                                    </div>
                                    </div>
                                </div>
                                </div>
                                <div class="col-md-4">
                                <div class="card" style="padding: 24px;
                                    margin-bottom: 24px;
                                            min-height: 160px;
                                            max-height: 160px;">
                                    <div class="card-block">
                                    <div>
                                        <div>
                                        <div style="
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            padding-bottom: 15px;
                                            ">
                                            <img
                                            src="https://cdn.auth0.com/marketplace/catalog/content/assets/creators/atatus/atatus-avatar.png"
                                            class="jss6144 jss6146" style="color: #65676e;
                                            margin: 0px;
                                            width: 2.5rem;
                                            height: 2.5rem;
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            justify-content: center;" />

                                            <h5 style="padding: 0px 15px;">{"Atatus"}</h5>

                                        </div>
                                        </div>
                                        <div>
                                        <p style=" max-height: calc(3.2em);
                                            overflow: hidden;
                                            display: -webkit-box;
                                            -webkit-line-clamp: 2;
                                            -webkit-box-orient: vertical;">{"Seamlessly monitor your entire software stack"}</p>
                                        </div>
                                    </div>
                                    </div>
                                </div>
                                </div>
                                <div class="col-md-4">
                                <div class="card" style="padding: 24px;
                                    margin-bottom: 24px;
                                            min-height: 160px;
                                            max-height: 160px;">
                                    <div class="card-block">
                                    <div>
                                        <div>
                                        <div style="
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            padding-bottom: 15px;
                                            ">
                                            <img
                                            src="https://cdn.auth0.com/marketplace/catalog/content/assets/creators/blocksedit/blocksedit-avatar.png"
                                            class="jss6144 jss6146" style="color: #65676e;
                                            margin: 0px;
                                            width: 2.5rem;
                                            height: 2.5rem;
                                            display: flex;
                                            overflow: hidden;
                                            position: relative;
                                            font-size: 1rem;
                                            align-items: center;
                                            font-weight: 500;
                                            text-transform: uppercase;
                                            justify-content: center;" />

                                            <h5 style="padding: 0px 15px;">{"Blocks Edit"}</h5>

                                        </div>
                                        </div>
                                        <div>
                                        <p style=" max-height: calc(3.2em);
                                            overflow: hidden;
                                            display: -webkit-box;
                                            -webkit-line-clamp: 2;
                                            -webkit-box-orient: vertical;">{"Create on-brand email campaigns faster and easier"}</p>
                                        </div>
                                    </div>
                                    </div>
                                </div>
                                </div>
                            </div>
                        
                        </div>
                    </div>

                    </div>
                </div>
                </div>
            </div>
        </>
        }
    }
}
