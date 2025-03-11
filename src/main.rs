mod aside;
mod main_ui;

use crate::aside::Aside;
use crate::main_ui::MainUI;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="flex flex-row w-screen pt-4 px-6">
            <div class="mr-15 min-w-54">
                <Aside/>
            </div>
            <MainUI/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
