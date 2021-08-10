// requires the serde and anyhow crates

use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

// #[derive(Deserialize, Debug, Clone)]
// pub struct ISSPosition {
//     latitude: String,
//     longitude: String,
// }

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    name: String,
    age: u8
}

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<User, anyhow::Error>),
}

#[derive(Debug)]
pub struct TestingFetch {
    fetch_task: Option<FetchTask>,
    user: Option<User>,
    link: ComponentLink<Self>,
    error: Option<String>,
}
/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl TestingFetch {
    fn view_iss_location(&self) -> Html {
        match self.user {
            Some(ref user) => {
                html! {
                    <>
                        <p>{ "User profile:" }</p>
                        <p>{ format!("Name: {}", user.name) }</p>
                        <p>{ format!("Age: {}", user.age) }</p>
                    </>
                }
            }
            None => {
                html! {
                     <button onclick=self.link.callback(|_| Msg::GetLocation)>
                         { "Get User" }
                     </button>
                }
            }
        }
    }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}
impl Component for TestingFetch {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            user: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            GetLocation => {
                // 1. build the request
                let request = Request::get("http://localhost:3000/user")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<User, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        self.user = Some(data);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_iss_location() }
                { self.view_error() }
            </>
        }
    }
}