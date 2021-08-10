use yewdux::prelude::*;

pub enum Action {
    Increment,
    Update(Counter)
}

#[derive(Clone)]
pub struct Counter {
    pub count: u64,
    pub name: String
}

impl Reducer for Counter {
    type Action = Action;

    fn new() -> Self {
        Self { 
            count: 0,
            name: String::from("User") 
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::Increment => {
                self.count += 1;
                true
            }
            Action::Update(data) => {
                self.count = data.count;
                self.name = data.name;
                true
            }
        }
    }
}

pub type AppDispatch = DispatchProps<ReducerStore<Counter>>;
