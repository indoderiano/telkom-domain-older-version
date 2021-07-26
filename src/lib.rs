mod app;
mod pages;
mod components;
mod store;

use yewdux::prelude::WithDispatch;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    // yew::start_app::<WithDispatch<app::App>>();
    // yew::start_app::<pages::outer::login_page::LoginPage>();
    // yew::start_app::<pages::outer::register_page::RegisterPage>();
    yew::start_app::<pages::outer::password_page::RequestPassPage>();



    Ok(())
}
