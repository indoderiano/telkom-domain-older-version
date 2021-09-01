use yew::prelude::*;
use yew_router::components::RouterAnchor;
use crate::app::AppRoute;
use super::tab_setting::TabSettings;
use super::tab_permission::TabPermissions;
use super::tab_users::TabUsers;


pub enum Content{
    Settings,
    Permissions,
    Users
}


pub struct ViewDetail {
    content: Content,
    link: ComponentLink<Self>
}

pub enum Msg {
    ChangeContent(Content)
}

impl Component for ViewDetail {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ViewDetail {
            content: Content::Settings,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <>
            <div class="mx-auto pt-5 pb-5 px-4" style="max-width: 1048px;">
                <div>
                    <Anchor route=AppRoute::RolesCreated classes="text-decoration-none text-muted">
                        <i class="bi bi-arrow-left"></i>
                        <span>{"Back To Roles"}</span>
                    </Anchor>
                </div>

                <div class="mt-2">
                    <p class="fw-bold fs-2">{"Brother Yeska"}</p>
                    <div class="pt-2">
                        <span class="text-muted">{"Role ID"}</span>
                        <code class="text-dark ms-2" style="background-color: #eff0f2; font-family: Roboto, sans-serif;">{"rol_Vbah0QfGcsV9Y28d"}</code>
                    </div>
                </div>

                <div class="mt-4">
                    <ul class="nav nav-tabs">
                            <li onclick=self.link.callback(|_|Msg::ChangeContent(Content::Settings)) class="nav-item">
                                <a class={
                                   match self.content {
                                       Content::Settings => "nav-link active",
                                       _ => "nav-link"
                                   }     
                                } 
                                aria-current="page" 
                                href="#">{"Settings"}</a>
                            </li>
                            <li onclick=self.link.callback(|_|Msg::ChangeContent(Content::Permissions)) class="nav-item">
                                <a class={
                                    match self.content{
                                        Content::Permissions => "nav-link active",
                                        _ => "nav-link"
                                    }
                                } 
                                href="#">{"Permissions"}</a>
                            </li>
                            <li onclick=self.link.callback(|_|Msg::ChangeContent(Content::Users)) class="nav-item">
                                <a class={
                                    match self.content{
                                        Content::Users => "nav-link active",
                                        _ => "nav-link"
                                    }
                                } href="#">{"Users"}</a>
                            </li>
                    </ul>
                </div>

                {
                    match self.content {
                        Content::Settings => html! { <TabSettings/> },
                        Content::Permissions => html! { <TabPermissions/> },
                        Content::Users => html! { <TabUsers/> }
                    }
                }



            </div>
            </>
        }
    }
}
