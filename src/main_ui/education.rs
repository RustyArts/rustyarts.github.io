use crate::main_ui::Header;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

#[function_component]
pub fn Education() -> Html {
    html! {
        <div class="flex flex-col">
            <Header text={"Education"}/>
            <div class="flex flex-row space-x-3">
                <School
                    name= {"Achimota Senior High School"}
                    text = {"High School"}
                    img_src = {"assets/achimota.png"}
                    time = {"2016 - 2019"}
                    border_radius = {"rounded-l-xl"}
                />
                <School
                    name= {"University of Ghana"}
                    text = {"MBChB"}
                    img_src = {"assets/university-of-ghana-logo.svg"}
                    time = {"2019 - 2025"}
                    border_radius = {"rounded-r-xl"}
                />
            </div>
        </div>
    }
}

#[autoprops]
#[function_component]
fn School(
    name: AttrValue,
    text: AttrValue,
    img_src: AttrValue,
    time: AttrValue,
    border_radius: AttrValue,
) -> Html {
    html! {
        <div class={format!("flex flex-col bg-slate-100/80 {border_radius} py-4 pl-4 pr-6 w-7/10")}>
            <div class="flex flex-row space-x-4 items-center">
                <img src={img_src} alt={&name}
                    class= "size-12 min-w-10 rounded-md mt-1"
                />
                <p class="text-stone-950 m-0"> {name} </p>
            </div>
            <div class="flex flex-col">
                <p class="text-stone-950 m-0"> {text} </p>
                <p class="text-slate-500 text-sm"> {time} </p>
            </div>
        </div>
    }
}
