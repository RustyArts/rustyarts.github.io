use crate::main_ui::education::Education;
use crate::main_ui::experience::Experience;
use crate::main_ui::featured_articles::FeaturedArticles;
use crate::main_ui::latest_projects::LatestProjects;
use crate::main_ui::skills::Skills;
use yew::{AttrValue, Html, function_component, html};
use yew_autoprops::autoprops;

mod education;
mod experience;
mod featured_articles;
mod latest_projects;
mod skills;

#[autoprops]
#[function_component]
pub fn MainUI() -> Html {
    html! {
        <div class="p-0 relative before:absolute before:inset-0 before:h-full before:w-0.5 \
        before:bg-gradient-to-b before:from-slate-50 before:via-slate-200 before:to-transparent space-y-7">
            <MainUIITem> <LatestProjects /> </MainUIITem>
            <MainUIITem> <Experience /> </MainUIITem>
            <MainUIITem> <Skills /> </MainUIITem>
            <MainUIITem> <FeaturedArticles /> </MainUIITem>
            <MainUIITem> <Education /> </MainUIITem>
        </div>
    }
}

#[autoprops]
#[function_component]
fn MainUIITem(children: Html) -> Html {
    html! {
        <div class="relative">
            <div class="flex flex-row">
                // circle marker
                <div class="flex items-center justify-center w-7 h-7 min-w-7 rounded-full bg-white shadow-md -translate-x-14/31 mt-1">
                    <div class="w-2 h-2 rounded-full bg-violet-700"/>
                </div>
                <div class="ml-8 mt-0">
                    {children}
                </div>
            </div>
        </div>
    }
}

#[autoprops]
#[function_component]
fn Header(text: AttrValue) -> Html {
    html! {
        <h2 class="text-2xl mb-2 mt-0 p-0">{text}</h2>
    }
}
