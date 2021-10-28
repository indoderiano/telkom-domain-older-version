use yew::prelude::*;

pub struct TabSettings {}

pub enum Msg {}

impl Component for TabSettings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TabSettings {}
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
                <div class="mt-4 p-4">
                    <form>
                        <div class="mb-3">
                            <label for="roleName" class="form-label">{"Name"}</label>
                            <input type="text" class="form-control w-50" id="roleName" aria-describedby="emailHelp"/>
                        </div>
                        <div class="mb-3">
                            <label for="exampleInputPassword1" class="form-label">{"Description"}</label>
                            <input type="password" class="form-control w-50" id="exampleInputPassword1"/>
                        </div>

                        <button type="submit" class="btn btn-primary">{"Save"}</button>
                    </form>
                </div>
                
                <div class="mt-2 p-4 pt-0">
                    <p class="fw-bold fs-5">{"Danger Zone"}</p>
                    <div class="alert alert-danger" role="alert">
                        <div class="row">
                            <div class="col">
                                <p class="text-danger fw-bold m-0">{"Delete Role"}</p>
                                <p class="text-danger m-0">{"Once confirmed, this operations can't be undone!"}</p>
                            </div>
                            <div class="col d-flex justify-content-end">
                                <button type="button" class="btn btn-danger">{"Remove this role!"}</button>
                            </div>
                        </div>
                    </div>
                </div>

            </>
        }
    }
}
