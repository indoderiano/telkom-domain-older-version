use yewdux::prelude::*;
use yew::services::ConsoleService;

use crate::types::{
    ResponseLogin
};

pub enum DataAccountAction {
    Update(ResponseLogin),
    Logout
}

#[derive(Clone, Debug)]
pub struct DataAccount {
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
    pub tenant_id: Option<String>,
}

impl Reducer for DataAccount {
    type Action = DataAccountAction;

    fn new() -> Self {
        Self { 
            username: None,
            email: None,
            token: None,
            tenant_id: Some(String::from("dev-ofzd5p1b"))
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            DataAccountAction::Update(data) => {
                ConsoleService::info("action reducer");
                self.username = Some(data.username);
                self.email = Some(data.email);
                self.token = Some(data.token);
                true
            }
            DataAccountAction::Logout => {
                *self = DataAccount::new();
                true
            }
        }
    }
}

pub type AppDispatch = DispatchProps<ReducerStore<DataAccount>>;