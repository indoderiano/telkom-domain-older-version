use yew::prelude::*;
use super::general::SettingsGeneral;

pub enum Content {
    General,
    TenantMembers,
    CustomDomains,
    SigningKeys,
    Advanced
}

pub struct SettingsHome {
    content: Content,
    link: ComponentLink<Self>
}

pub enum Msg {
    ChangeContent(Content)
}

impl Component for SettingsHome {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsHome {
            content: Content::General,
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
        html! {
            <div
                class="py-5 px-4 m-auto"
                style="max-width: 1048px; font-size:14px;"
            >
                <div
                    class="d-flex mb-5 mt-3"
                >
                    <div
                        class="d-flex flex-column"
                    >
                        <h2>{"Tenant Settings"}</h2>
                    </div>
                </div>

                <div
                    class="mb-4"
                >
                    <ul class="nav nav-tabs">
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::General))
                            class="nav-item"
                        >
                        <a
                            // class="nav-link active"
                            class={
                                match self.content {
                                    Content::General => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            aria-current="page"
                            href="#"
                        >
                            {"General"}</a>
                        </li>
                        <li
                            onclick=self.link.callback(|_| Msg::ChangeContent(Content::TenantMembers))
                            class="nav-item">
                        <a
                            // class="nav-link"
                            class={
                                match self.content {
                                    Content::TenantMembers => "nav-link active",
                                    _ => "nav-link"
                                }
                            }
                            href="#">{"Tenant Members"}</a>
                        </li>
                        <li class="nav-item">
                        <a class="nav-link" href="#">{"Custom Domain"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Signing Keys"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#">{"Advanced"}</a>
                        </li>
                    </ul>
                </div>

                {
                    match self.content {
                        Content::General => html! { <SettingsGeneral/> },
                        _ => html! {}
                    }
                }
            </div>
        }
    }
}
