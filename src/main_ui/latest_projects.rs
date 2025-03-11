use crate::aside::Links;
use crate::main_ui::Header;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

#[autoprops]
#[function_component]
pub fn LatestProjects() -> Html {
    html! {
        <div class="flex flex-col">
            <Header text = {"Latest Projects"}/>
            <div class="flex flex-row space-x-3">
                <LatestProject
                    heading= {"Build A Web + Desktop Application With Rust"}
                    text = {"Learn how to create your web, mobile or desktop application with\
                     Rust using tauri, tailwind and leptos frameworks. Get hands-on experience..."}
                    img = {
                        html! {
                            <img src={"assets/level-up-coding.jpg"}
                                class= "size-14 min-w-12 rounded-md mt-1"
                            />
                        }
                    }
                    link = {
                        ("https://levelup.gitconnected.com".into(),
                            "https://medium.com/gitconnected/create-a-web-desktop-application-with-rust-c8449f661ecc".into()
                        )
                    }
                    border_radius = {"rounded-l-xl"}
                />
                <LatestProject
                    heading= {"Why Rust Might Be Your Best First Language"}
                    text = {"Ah, the firsts in life! Your first bike, first crush, first taste of a bizarre foreign delicacy, and… your first programming language"}
                    img = {
                        html! {
                            <div class="bg-neutral-300 rounded-md size-14 min-w-12 mt-1">
                                <img src={"assets/stackademic.png"}/>
                            </div>
                        }
                    }
                    link = {("https://blog.stackademic.com".into(),
                        "https://blog.stackademic.com/breaking-tradition-why-rust-might-be-your-best-first-language-d10afc482ac1".into())}
                    border_radius = {"rounded-r-xl"}
                />
            </div>
        </div>
    }
}

#[autoprops]
#[function_component]
fn LatestProject(
    heading: AttrValue,
    text: AttrValue,
    img: Html,
    link: (AttrValue, AttrValue),
    border_radius: AttrValue,
) -> Html {
    html! {
        <a href={link.1} target="_blank" class={format!("flex flex-col bg-slate-100/80 {border_radius} py-4 pl-4 pr-6 flex-1 space-y-1")}>
            <div class="flex flex-row space-x-4 grow">
                {img}
                <div class="flex flex-col m-0">
                    <p class="text-stone-950 m-0"> {heading} </p>
                    <p class="text-slate-500 text-sm"> {text} </p>
                </div>
            </div>
            <div class="flex flex-row space-x-3 items-center text-indigo-500/90 font-bold">
                <div class="size-8 bg-indigo-100 rounded-full text-xs flex items-center justify-center">
                    {Links::Website.img()}
                </div>
                <p> {link.0} </p>
            </div>
        </a>
    }
}
