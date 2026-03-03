use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        //document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar", class: "navbar bg-secondary shadow-sm",

            div { class: "flex-none",
                button { class: "btn btn-square btn-ghost",
                    svg {
                        class: "inline-block h-5 w-5 stroke-current",
                        fill: "none",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M4 6h16M4 12h16M4 18h16",
                            "strokeLinecap": "round",
                            "strokeLinejoin": "round",
                            "strokeWidth": "2",
                        }
                    }
                }
            }
            div { class: "flex-1",
                a { class: "btn btn-ghost text-xl", "Ponderis" }
            }

            div { class: "navbar-content", {children} }



            div { class: "flex-none",
                button { class: "btn btn-square btn-ghost",
                    svg {
                        class: "inline-block h-5 w-5 stroke-current",
                        fill: "none",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            d: "M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z",
                            "strokeLinecap": "round",
                            "strokeLinejoin": "round",
                            "strokeWidth": "2",
                        }
                    }
                }
            }
        }
    }
}
