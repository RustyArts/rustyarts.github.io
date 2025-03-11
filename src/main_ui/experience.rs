use crate::main_ui::Header;
use derive_more::Display;
use web_sys::js_sys::Date;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

#[function_component]
pub fn Experience() -> Html {
    html! {
        <div class= "">
            <Header text = {"Experience"}/>
            <ul class="list-none space-y-5">
                {for [ExperienceDetails::Upwork,ExperienceDetails::Medium,ExperienceDetails::Voiceban,ExperienceDetails::Assistant]
                    .map(|details| html!{
                        <li class="flex items-center justify-center">
                            <div class="size-2 bg-stone-950 rounded-full ml-5 mr-5 flex-none"/>
                            {details.experience_item()}
                        </li>
                    })
                }
            </ul>
        </div>
    }
}

#[autoprops]
#[function_component]
fn ExperienceItem(
    timeline: &DateTimeDetails,
    img_src: AttrValue,
    heading: AttrValue,
    subtitle: AttrValue,
    text: AttrValue,
) -> Html {
    html! {
        <div class ="flex flex-row space-x-1">
            <div class ="flex flex-col flex-none w-64">
                <p class="text-slate-400 text-sm">
                    <time datetime={timeline.start_datetime.clone()}>{&timeline.start}</time>
                    {" - "}
                    <time datetime={timeline.end_datetime.clone()}>{&timeline.end}</time>
                </p>
                <div class ="flex flex-row space-x-4 mt-1">
                    <img class ="size-10 min-w-8 rounded-md" src={img_src} />
                    <div class ="flex flex-col">
                        <p class="text-slate-500 text-[13px]"> {heading}</p>
                        <p class="text-stone-950 text-sm"> {subtitle}</p>
                    </div>
                </div>
            </div>
            <p class="text-slate-500 text-xs">{text}</p>
        </div>
    }
}

#[derive(Clone, Copy, Display, Debug)]
enum ExperienceDetails {
    Voiceban,
    Medium,
    Upwork,
    Assistant,
}

#[derive(Clone, Debug, PartialEq)]
struct DateTimeDetails {
    start_datetime: String,
    end_datetime: String,
    start: String,
    end: String,
}

impl ExperienceDetails {
    fn timeline(self) -> DateTimeDetails {
        match self {
            ExperienceDetails::Voiceban => DateTimeDetails {
                start_datetime: "2022-08".to_string(),
                end_datetime: "2022-08".to_string(),
                start: "Aug 2022".to_string(),
                end: "Feb 2025".to_string(),
            },
            ExperienceDetails::Medium => DateTimeDetails {
                start_datetime: "2021-08".to_string(),
                end_datetime: Date::new_0().to_json().into(),
                start: "Aug 2021".to_string(),
                end: "Present".to_string(),
            },
            ExperienceDetails::Upwork => DateTimeDetails {
                start_datetime: "2020-06".to_string(),
                end_datetime: Date::new_0().to_json().into(),
                start: "June 2022".to_string(),
                end: "Present".to_string(),
            },
            ExperienceDetails::Assistant => DateTimeDetails {
                start_datetime: "2023-01".to_string(),
                end_datetime: "2024-12".to_string(),
                start: "Jan 2023".to_string(),
                end: "Dec 2024".to_string(),
            },
        }
    }

    fn img_src(self) -> &'static str {
        match self {
            ExperienceDetails::Voiceban => "assets/letter-v.png",
            ExperienceDetails::Medium => "https://cdn-icons-png.flaticon.com/512/5968/5968906.png",
            ExperienceDetails::Upwork => "assets/upwork.svg",
            ExperienceDetails::Assistant => "assets/wonders_co.jpeg",
        }
    }

    fn heading(self) -> &'static str {
        match self {
            ExperienceDetails::Voiceban => "Lead Software Developer",
            ExperienceDetails::Medium => "Writer",
            ExperienceDetails::Upwork => "Freelancer",
            ExperienceDetails::Assistant => "Coding Assistant",
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            ExperienceDetails::Voiceban => "Voiceban",
            ExperienceDetails::Medium => "Medium",
            ExperienceDetails::Upwork => "Upwork",
            ExperienceDetails::Assistant => "WondersCo",
        }
    }

    fn text(self) -> &'static str {
        match self {
            ExperienceDetails::Voiceban => {
                "At Voiceban, a blockchain startup, I led the development team in designing \
                and implementing decentralized solutions. I oversaw project \
                architecture, coordinate cross-functional collaboration, and ensure \
                secure code delivery, fostering a culture of continuous improvement and \
                rapid innovation in a dynamic startup environment."
            }
            ExperienceDetails::Medium => {
                "I write tutorials on Rust and Android development. My articles simplify \
                complex programming concepts into accessible, step-by-step guides that \
                engage a broad audience. This experience has refined my technical writing skills \
                and deepened my understanding of modern software practices."
            }
            ExperienceDetails::Upwork => {
                "I've collaborated with diverse clients to deploy scalable systems using Rust, Android and Python. \
                 I maintained 𝟭𝟬𝟬%client satisfaction through performance-driven development and \
                 effective communication."
            }
            ExperienceDetails::Assistant => {
                "I assisted a developer at WondersCo on Android and Rust \
            projects. I contributed to coding, debugging, and documentation efforts, streamlining \
            project workflows and ensuring high-quality software delivery."
            }
        }
    }

    fn experience_item(self) -> Html {
        html! {
            <ExperienceItem
                timeline = {self.timeline()}
                img_src = {self.img_src()}
                heading = {self.heading()}
                subtitle = {self.subtitle()}
                text = {self.text()}
            />
        }
    }
}
