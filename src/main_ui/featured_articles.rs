use crate::aside::Links;
use crate::main_ui::Header;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

#[function_component]
pub fn FeaturedArticles() -> Html {
    html! {
        <div class="flex flex-col">
            <Header text = {"Featured Articles"}/>
            <div class="flex flex-row space-x-2">
                <FeaturedArticle
                    heading= {"Integrating Rust With Android Development"}
                    text = {"Rust is a general-purpose, systems programming \
                    language that has been around for quite some time now. \
                    Being a systems..."}
                    img_src = {"assets/rust-android.webp"}
                    link = {("https://levelup.gitconnected.com".into(),
                        "https://medium.com/gitconnected/integrating-rust-with-android-development-ef341c2f9cca".into())
                    }
                    border_radius = {"rounded-l-xl"}
                />
                <FeaturedArticle
                    heading= {"Rust’s Approach to Local File Transfers"}
                    text = {"Ever wondered how to wirelessly transfer a file from your phone \
                    to your PC? In this beginner-friendly tutorial, we’ll learn how to use..."}
                    img_src = {"assets/sender-to-rec.webp"}
                    link = {("https://levelup.gitconnected.com".into(),
                        "https://medium.com/gitconnected/from-sender-to-receiver-rusts-approach-to-local-file-transfers-e6a778020d90".into())
                    }
                />
                <FeaturedArticle
                    heading= {"Implementing data parallelism with Rayon Rust"}
                    text = {"Concurrency and parallel computing are hot topics in the world of \
                    software development. To be fair, this popularity is well deserved because..."}
                    img_src = {"assets/implementing-data-parallelism-rayon-rust.avif"}
                    link = {("https://blog.logrocket.com".into(),
                        "https://blog.logrocket.com/implementing-data-parallelism-rayon-rust/".into())
                    }
                    border_radius = {"rounded-r-xl"}
                />
            </div>
        </div>
    }
}

#[autoprops]
#[function_component]
fn FeaturedArticle(
    heading: AttrValue,
    text: AttrValue,
    img_src: AttrValue,
    link: (AttrValue, AttrValue),
    #[prop_or_default] border_radius: AttrValue,
) -> Html {
    html! {
        <a href={link.1} target="_blank" class={format!("flex flex-col bg-slate-100/80 {border_radius} py-4 pl-4 pr-6 flex-1")}>
            <img src={img_src} alt={&heading}
                class= "size-18 min-w-32 rounded-md mb-1 object-fill"
            />
            <div class="flex flex-col m-0 grow">
                <p class="text-stone-950 m-0"> {heading} </p>
                <p class="text-slate-500 text-sm"> {text} </p>
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
