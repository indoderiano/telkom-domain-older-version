use yewdux::prelude::*;
use yew::services::ConsoleService;

pub enum DataAccountAction {
    Update(DataAccount)
}

#[derive(Clone)]
pub struct DataAccount {
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
}

impl Reducer for DataAccount {
    type Action = DataAccountAction;

    fn new() -> Self {
        Self { 
            username: None,
            email: None,
            token: None,
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            DataAccountAction::Update(data) => {
                ConsoleService::info("action reducer");
                self.username = data.username;
                self.email = data.email;
                self.token = data.token;
                true
            }
        }
    }
}

pub type AppDispatch = DispatchProps<ReducerStore<DataAccount>>;