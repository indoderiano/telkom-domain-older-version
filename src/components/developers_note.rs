use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Note {
    #[prop_or("Create your note warnings here")]
    pub message: str,
}

#[derive(Properties, Clone, PartialEq)]
pub struct DevelopersNote {
    message: str,
}

pub enum Msg {}

impl Component for DevelopersNote {
    type Message = Msg;
    type Properties = Note;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DevelopersNote {
            message: props.message,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.message == props.message {
            false
        } else {
            self.message = props.message;
            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <div
                class="pt-2"
            >
                <div
                    class="alert alert-danger position-relative"
                    role="alert"
                    style="font-size: 14px;"
                >
                    <i
                        class="bi bi-exclamation-triangle-fill"
                        style="font-size: 18px;"    
                    />
                    { self.message.clone() }
                    <span
                        class="position-absolute top-0 translate-middle-y badge rounded-pill bg-danger"
                        style="left: 10px;"    
                    >
                        { "For Developers" }
                    </span>
                </div>
            </div>
        }
    }
}