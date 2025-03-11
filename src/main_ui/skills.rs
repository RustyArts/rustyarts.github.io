#![allow(clippy::upper_case_acronyms)]
use crate::main_ui::Header;
use derive_more::Display;
use std::fmt::Display;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

#[function_component]
pub fn Skills() -> Html {
    html! {
        <>
            <Header text={"Skills"}/>
            <div class="flex flex-col space-y-3">
                <Section<Language> />
                <Section<WebFramework> />
                <Section<Android> />
                <Section<Desktop> />
                <Section<Database> />
                <Section<Blockchain> />
                <Section<Network> />
                <Section<DevOps> />
            </div>
        </>
    }
}

#[autoprops]
#[function_component]
fn SkillItem(text: AttrValue, color: AttrValue) -> Html {
    html! {
        <div class="p-4 bg-slate-100 rounded-lg flex items-center justify-center text-sm">
            <p class={color}>{text}</p>
        </div>
    }
}

#[autoprops]
#[function_component]
fn Section<T: Display + Details + PartialEq>() -> Html {
    html! {
        <div class="flex flex-row space-x-14">
            <div class="flex flex-col space-y-2 w-20">
                <div class={format!("size-8 {} rounded-full text-xs flex items-center justify-center", T::color().0)}>
                    <img src= {T::img_src()} alt="languages icon" class="size-4"/>
                </div>
                <p class="text-stone-950 text-sm">{T::section_text()}</p>
            </div>
            <div class="flex flex-row space-x-1">
                {for T::items().into_iter().map(|language|html!(<SkillItem text={language.to_string()} color={T::color().1} />))}
            </div>
        </div>
    }
}

#[derive(Display, Copy, Clone, PartialEq)]
enum Language {
    Rust,
    Python,
    Kotlin,
    Java,
    Javascript,
    Shell,
}
#[derive(Display, Copy, Clone, PartialEq)]
enum WebFramework {
    #[display("Html CSS")]
    Html,
    Yew,
    Leptos,
    ReactJS,
}

#[derive(Display, Copy, Clone, PartialEq)]
enum Android {
    JetpackCompose,
    Flutter,
    MaterialDesign,
    NDK,
}

#[derive(Display, Copy, Clone, PartialEq)]
enum Desktop {
    Tauri,
    RustDesk,
}

#[derive(Display, Copy, Clone, PartialEq)]
enum Database {
    Sql,
    Sqlite,
}

#[derive(Display, Copy, Clone, PartialEq)]
enum Blockchain {
    Substrate,
    Solana,
    Solidity,
    #[display("ink!")]
    Ink,
}

#[derive(Display, Copy, Clone, PartialEq)]
enum Network {
    #[display("SSL/TLS")]
    SSL,
    TCP,
    DNS,
}

#[derive(Display, Copy, Clone, PartialEq)]
enum DevOps {
    Git,
    Docker,
    Bitbucket,
}

trait Details {
    fn img_src() -> AttrValue;
    /// background, text
    fn color() -> (AttrValue, AttrValue);

    fn section_text() -> AttrValue;

    fn items() -> Vec<Self>
    where
        Self: Sized;
}

impl Details for WebFramework {
    fn img_src() -> AttrValue {
        "assets/web.svg".into()
    }

    fn color() -> (AttrValue, AttrValue) {
        ("bg-violet-100".into(), "text-violet-500".into())
    }

    fn section_text() -> AttrValue {
        "Web".into()
    }

    fn items() -> Vec<Self> {
        vec![Self::Html, Self::ReactJS, Self::Yew, Self::Leptos]
    }
}

impl Details for Language {
    fn img_src() -> AttrValue {
        "assets/coding.svg".into()
    }

    fn color() -> (AttrValue, AttrValue) {
        ("bg-purple-100".into(), "text-purple-500".into())
    }

    fn section_text() -> AttrValue {
        "Languages".into()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Rust,
            Self::Kotlin,
            Self::Java,
            Self::Javascript,
            Self::Python,
            Self::Shell,
        ]
    }
}

impl Details for Android {
    fn img_src() -> AttrValue {
        "assets/android.svg".into()
    }

    fn color() -> (AttrValue, AttrValue) {
        ("bg-lime-100".into(), "text-lime-700".into())
    }

    fn section_text() -> AttrValue {
        "Android".into()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        [
            Self::JetpackCompose,
            Self::Flutter,
            Self::MaterialDesign,
            Self::NDK,
        ]
        .to_vec()
    }
}
impl Details for Desktop {
    fn img_src() -> AttrValue {
        "assets/desktop.svg".into()
    }

    fn color() -> (AttrValue, AttrValue) {
        ("bg-sky-100".into(), "text-sky-500".into())
    }

    fn section_text() -> AttrValue {
        "Desktop".into()
    }
    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        [Self::RustDesk, Self::Tauri].to_vec()
    }
}
impl Details for Database {
    fn img_src() -> AttrValue {
        "assets/database.svg".into()
    }

    fn color() -> (AttrValue, AttrValue) {
        ("bg-teal-100".into(), "text-teal-600".into())
    }

    fn section_text() -> AttrValue {
        "Database".into()
    }
    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        [Self::Sql, Self::Sqlite].to_vec()
    }
}
impl Details for Blockchain {
    fn img_src() -> AttrValue {
        "assets/blockchain.svg".into()
    }

    fn color() -> (AttrValue, AttrValue) {
        ("bg-amber-100".into(), "text-amber-600".into())
    }

    fn section_text() -> AttrValue {
        "Blockchain".into()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![Self::Substrate, Self::Ink, Self::Solana, Self::Solidity]
    }
}

impl Details for Network {
    fn img_src() -> AttrValue {
        "assets/network.svg".into()
    }

    fn color() -> (AttrValue, AttrValue) {
        ("bg-pink-50".into(), "text-pink-600".into())
    }

    fn section_text() -> AttrValue {
        "Network".into()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![Self::SSL, Self::TCP, Self::DNS]
    }
}

impl Details for DevOps {
    fn img_src() -> AttrValue {
        "assets/devops.svg".into()
    }

    fn color() -> (AttrValue, AttrValue) {
        ("bg-stone-100".into(), "text-stone-700".into())
    }

    fn section_text() -> AttrValue {
        "DevOps".into()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        [Self::Git, Self::Docker, Self::Bitbucket].to_vec()
    }
}
