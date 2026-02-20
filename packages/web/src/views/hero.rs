use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        div {
            class: "hero min-h-screen bg-base-200",
            div {
                class: "hero-content text-center",
                div {
                    class: "max-w-md",
                    img { src: HEADER_SVG, class: "mb-8 rounded-lg shadow-2xl" }
                    h1 { class: "text-5xl font-bold", "Ponderis" }
                    p { class: "py-6", "Build cool things with Dioxus and DaisyUI!" }
                    div { class: "flex flex-col gap-2",
                        a { class: "btn btn-primary", href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                        a { class: "btn btn-secondary", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                        a { class: "btn btn-accent", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                        a { class: "btn btn-ghost", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
                    }
                }
            }
        }
    }
}
