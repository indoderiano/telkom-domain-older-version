use yewdux::prelude::*;

pub enum DataAccountAction {
    Update(DataAccount)
}

#[derive(Clone)]
pub struct DataAccount {
    pub name: Option<String>
}

impl Reducer for DataAccount {
    type Action = DataAccountAction;

    fn new() -> Self {
        Self { 
            name: None
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            DataAccountAction::Update(data) => {
                self.name = data.name;
                true
            }
        }
    }
}

pub type AppDispatch = DispatchProps<ReducerStore<DataAccount>>;