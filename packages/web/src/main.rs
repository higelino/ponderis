use dioxus::prelude::*;

use views::{Basket, Blog, Home, Navbar};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/basket/:id")]
    Basket { id: i32 },
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/styling/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon" }
        //document::Link { rel: "stylesheet" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        //document::Script { src: AGGRID_JS }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            div { class: "flex-none",
                ul { class: "menu menu-horizontal px-1",
                    li {
                        Link { to: Route::Home {}, "Home" }
                    }
                    li {
                        Link { to: Route::Basket { id: 1 }, "Basket" }
                    }
                    li {
                        Link { to: Route::Blog { id: 1 }, "Blog" }
                    }
                    li {
                        a { "Link" }
                    }
                    li {
                        details {
                            summary { "Parent" }
                            ul { class: "bg-base-100 rounded-t-none p-2",
                                li {
                                    a { "Link 1" }
                                }
                                li {
                                    a { "Link 2" }
                                }
                            }
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
