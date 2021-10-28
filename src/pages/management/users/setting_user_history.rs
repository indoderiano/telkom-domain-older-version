use yew::prelude::*;

pub struct UserTabHistory {}

pub enum Msg {}

impl Component for UserTabHistory {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        UserTabHistory {}
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
                <div class="mt-4">
                    <p>
                        {"Max. Log Storage: 2 days"}
                    </p>
                </div>
                
                <div class="mt-4 table-responsive">
                    <table class="table">
                        <thead>
                            <tr>
                                <th scope="col"></th>
                                <th scope="col">{"Event"}</th>
                                <th scope="col">{"When"}</th>
                                <th scope="col">{"App"}</th>
                                <th scope="col">{"Identity Provider"}</th>
                                <th scope="col">{"From"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th scope="row"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#13a688" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg></th>
                                <td><a href="">{"Success Logout"}</a></td>
                                <td>{"12 minutes ago"}</td>
                                <td>{"My NextJs App"}</td>
                                <td>{"google-oauth2"}</td>
                                <td>
                                    <p class="m-0">{"IP: 36.96.215.66"}</p>
                                    <p class="m-0">{"Tangerang, Indonesia"}</p>
                                </td>
                            </tr>
                            <tr>
                                <th scope="row"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#13a688" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg></th>
                                <td><a href="">{"Success Login"}</a></td>
                                <td>{"15 minutes ago"}</td>
                                <td>{"My NextJs App"}</td>
                                <td>{"google-oauth2"}</td>
                                <td>
                                    <p class="m-0">{"IP: 36.96.215.66"}</p>
                                    <p class="m-0">{"Tangerang, Indonesia"}</p>
                                </td>
                            </tr>
                            <tr>
                                    <th scope="row"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#13a688" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg></th>
                                    <td><a href="">{"Success Signup"}</a></td>
                                    <td>{"15 minutes ago"}</td>
                                    <td>{"My NextJs App"}</td>
                                    <td>{"google-oauth2"}</td>
                                    <td>
                                        <p class="m-0">{"IP: 36.96.215.66"}</p>
                                        <p class="m-0">{"Tangerang, Indonesia"}</p>
                                    </td>
                            </tr>
                             <tr>
                                <th scope="row"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#13a688" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg></th>
                                <td><a href="">{"Success Exchange"}</a></td>
                                <td>{"18 minutes ago"}</td>
                                <td>{"My NextJs App"}</td>
                                <td>{"N/A"}</td>
                                <td>
                                    <p class="m-0">{"IP: 36.96.215.66"}</p>
                                    <p class="m-0">{"Tangerang, Indonesia"}</p>
                                </td>
                                </tr>
                        </tbody>     
                    </table>
                   
                    
                </div>
                

                <div class="mt-4">
                        <div class="row">
                            <div class="col d-flex justify-content-start">
                                <button type="button" class="btn btn-primary" disabled=true>
                                    <i class="bi bi-arrow-left"></i>
                                    <span>{"NEWER"}</span>
                                </button>
                            </div>
                            <div class="col d-flex justify-content-center">
                                <p>{"Page 1"}</p>
                            </div>
                            <div class="col d-flex justify-content-end">
                                <button type="button" class="btn btn-primary" disabled=true>
                                    <i class="bi bi-arrow-right"></i>
                                    <span>{"OLDER"}</span>
                                </button>
                            </div>
                        </div>
                </div>

            </>
        }
    }
}
