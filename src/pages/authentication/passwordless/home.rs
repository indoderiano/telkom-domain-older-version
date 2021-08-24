use yew::prelude::*;
use super::sms_app::SmsApp;
use super::sms_setting::SmsSetting;
use super::sms_try::SmsTry;
use super::email_app::EmailApp;
use super::email_setting::EmailSettings;
use super::email_try::EmailTry;

pub enum Content{
    Settings,
    Try,
    Application,
}

pub enum EmailContent{
    EmailSettings,
    EmailTry,
    EmailApplication,
}



pub struct AuthPasswordLess {
    content: Content,
    email_content: EmailContent,
    link: ComponentLink<Self>
}

pub enum Msg {
    ChangeContent(Content),
    ChangeEmailContent(EmailContent)
}

impl Component for AuthPasswordLess {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        AuthPasswordLess {
            content: Content::Settings,
            email_content: EmailContent::EmailSettings,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::ChangeContent(content) => {
                self.content = content;
                true
            },
            Msg::ChangeEmailContent(email_content) => {
                self.email_content = email_content;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="flex-fill fs-3 fw-bold mt-3"> {"Passwordless Connections"}</div>
                <p class="text-muted">{"Configure Passwordless Connections like SMS Login, Email Login and others so that you can let your users login without having to choose yet another password. "} <a href="">{"Learn more "} <i class="bi bi-arrow-right-short"></i></a></p>
                <div class="container" style="padding: 0px;">
                    <div class="row">
                        <div class="col">
                            <div class="card">
                                <div class="card-body" data-bs-toggle="modal" data-bs-target="#smsModal">
                                    <div class="container">
                                        <div class="row">
                                            <div class="col-2 pt-2">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" class="bi bi-chat-left" viewBox="0 0 16 16">
                                                    <path d="M14 1a1 1 0 0 1 1 1v8a1 1 0 0 1-1 1H4.414A2 2 0 0 0 3 11.586l-2 2V2a1 1 0 0 1 1-1h12zM2 0a2 2 0 0 0-2 2v12.793a.5.5 0 0 0 .854.353l2.853-2.853A1 1 0 0 1 4.414 12H14a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2H2z"/>
                                                </svg>
                                            </div>
                                            <div class="col-8 text-start">
                                                <p style="font-size: 28px;">{"SMS"}</p>
                                            </div>
                                            <div class="col-2 pt-2">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" class="bi bi-toggle2-off" viewBox="0 0 16 16">
                                                    <path d="M9 11c.628-.836 1-1.874 1-3a4.978 4.978 0 0 0-1-3h4a3 3 0 1 1 0 6H9z"/>
                                                    <path d="M5 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm0 1A5 5 0 1 0 5 3a5 5 0 0 0 0 10z"/>
                                                </svg>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="col">
                            <div class="card">
                                <div class="card-body" data-bs-toggle="modal" data-bs-target="#emailModal">
                                    <div class="container">
                                        <div class="row">
                                            <div class="col-2 pt-2">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" class="bi bi-chat-left" viewBox="0 0 16 16">
                                                    <path d="M14 1a1 1 0 0 1 1 1v8a1 1 0 0 1-1 1H4.414A2 2 0 0 0 3 11.586l-2 2V2a1 1 0 0 1 1-1h12zM2 0a2 2 0 0 0-2 2v12.793a.5.5 0 0 0 .854.353l2.853-2.853A1 1 0 0 1 4.414 12H14a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2H2z"/>
                                                </svg>
                                            </div>
                                            <div class="col-8 text-start">
                                                <p style="font-size: 28px;">{"Email"}</p>
                                            </div>
                                            <div class="col-2 pt-2">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" class="bi bi-toggle2-off" viewBox="0 0 16 16">
                                                    <path d="M9 11c.628-.836 1-1.874 1-3a4.978 4.978 0 0 0-1-3h4a3 3 0 1 1 0 6H9z"/>
                                                    <path d="M5 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm0 1A5 5 0 1 0 5 3a5 5 0 0 0 0 10z"/>
                                                </svg>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                // sms smsModal
                <div
                    class="modal fade"
                    id="smsModal"
                    tabindex="-1"
                    aria-labelledby="exampleModalLabel"
                    aria-hidden="true"
                >
                    <div class="modal-dialog modal-dialog-scrollable">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"SMS (Twilio)"}</h5>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body" style="font-size: 14px;">
                                <div class="mb-4">
                                    <ul class="nav nav-tabs" id="myTab" role="tablist">
                                        <li 
                                            onclick= self.link.callback(|_| Msg::ChangeContent(Content::Settings))
                                            class="nav-item" 
                                            role="presentation"
                                        >
                                            <button 
                                                class={
                                                    match self.content {
                                                        Content::Settings => "nav-link active",
                                                        _ => "nav-link"
                                                    }
                                                } 
                                                id="home-tab" 
                                                data-bs-toggle="tab" 
                                                data-bs-target="#home" 
                                                type="button" 
                                                role="tab" 
                                                aria-controls="home" 
                                                aria-selected="true">{"Settings"}
                                            </button>
                                        </li>

                                        <li onclick= self.link.callback(|_| Msg::ChangeContent(Content::Application))
                                            class="nav-item" 
                                            role="presentation"
                                        >
                                            <button 
                                                class={
                                                    match self.content {
                                                        Content::Application => "nav-link active",
                                                        _ => "nav-link"
                                                    }
                                                } 
                                                id="profile-tab" 
                                                data-bs-toggle="tab" 
                                                data-bs-target="#profile" 
                                                type="button" 
                                                role="tab" 
                                                aria-controls="profile" 
                                                aria-selected="false">{"Application"}
                                            </button>
                                        </li>
                                        <li onclick= self.link.callback(|_| Msg::ChangeContent(Content::Try))
                                            class="nav-item" 
                                            role="presentation"
                                        >
                                            <button 
                                                class={
                                                    match self.content {
                                                        Content::Try => "nav-link active",
                                                        _ => "nav-link"
                                                    }
                                                } 
                                                id="contact-tab" 
                                                data-bs-toggle="tab" 
                                                data-bs-target="#contact" 
                                                type="button" role="tab" 
                                                aria-controls="contact" 
                                                aria-selected="false">{"Try"}
                                            </button>
                                        </li>
                                    </ul>
                                </div>
                                {
                                    match self.content {
                                        Content::Settings => html! { <SmsSetting/> },
                                        Content::Try => html! { <SmsTry/> },
                                        Content::Application => html! { <SmsApp/> },
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>


                <div
                    class="modal fade"
                    id="emailModal"
                    tabindex="-1"
                    aria-labelledby="exampleModalLabel"
                    aria-hidden="true"
                    // style="overflow: hidden;"
                >
                    <div class="modal-dialog modal-dialog-scrollable">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title" id="exampleModalLabel">{"Email"}</h5>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body" style="font-size: 14px;">
                                <div class="mb-4">
                                    <ul class="nav nav-tabs" id="myTab" role="tablist">
                                        <li 
                                            onclick= self.link.callback(|_| Msg::ChangeEmailContent(EmailContent::EmailSettings))
                                            class="nav-item" 
                                            role="presentation"
                                        >
                                            <button 
                                                class={
                                                    match self.email_content {
                                                        EmailContent::EmailSettings => "nav-link active",
                                                        _ => "nav-link"
                                                    }
                                                } 
                                                id="home-tab" 
                                                data-bs-toggle="tab" 
                                                data-bs-target="#home" 
                                                type="button" 
                                                role="tab" 
                                                aria-controls="home" 
                                                aria-selected="true">{"Settings"}
                                            </button>
                                        </li>

                                        <li onclick= self.link.callback(|_| Msg::ChangeEmailContent(EmailContent::EmailApplication))
                                            class="nav-item" 
                                            role="presentation"
                                        >
                                            <button 
                                                class={
                                                    match self.email_content {
                                                        EmailContent::EmailApplication => "nav-link active",
                                                        _ => "nav-link"
                                                    }
                                                } 
                                                id="profile-tab" 
                                                data-bs-toggle="tab" 
                                                data-bs-target="#profile" 
                                                type="button" 
                                                role="tab" 
                                                aria-controls="profile" 
                                                aria-selected="false">{"Application"}
                                            </button>
                                        </li>
                                        <li onclick= self.link.callback(|_| Msg::ChangeEmailContent(EmailContent::EmailTry))
                                            class="nav-item" 
                                            role="presentation"
                                        >
                                            <button 
                                                class={
                                                    match self.email_content {
                                                        EmailContent::EmailTry => "nav-link active",
                                                        _ => "nav-link"
                                                    }
                                                } 
                                                id="contact-tab" 
                                                data-bs-toggle="tab" 
                                                data-bs-target="#contact" 
                                                type="button" role="tab" 
                                                aria-controls="contact" 
                                                aria-selected="false">{"Try"}
                                            </button>
                                        </li>
                                    </ul>
                                </div>
                                {
                                    match self.email_content {
                                        EmailContent::EmailSettings => html!(<EmailSettings/>),
                                        EmailContent::EmailTry => html!(<EmailTry/>),
                                        EmailContent::EmailApplication => html!(<EmailApp/>),
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </div>


            </>    
        }
    }
}
