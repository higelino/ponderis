use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

#[component]
pub fn Basket(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet" }

        div {
            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }
            div {
                class: "ag-theme-alpine",
                style: "height: 500px; width: 100%;",
                GridComponent {}
            }
        }
    }
}

#[cfg(not(feature = "web"))]
#[component]
pub fn GridComponent() -> Element {
    rsx! {}
}

const AGGRID_JS: Asset = asset!("/assets/ag-grid-community.min.js");

#[cfg(feature = "web")]
#[component]
pub fn GridComponent() -> Element {
    rsx! {
        document::Script { src: AGGRID_JS }

        div {
            class: "ag-theme-alpine",
            style: "height: 500px; width: 100%;",

            onmounted: move |event| {
                // DOM element (web target)

                // Execute JS with the element passed as argument
                // createGrid(
                //     &element,
                //     JsValue::from_serde(&columns).unwrap(),
                //     JsValue::from_serde(&rows).unwrap(),
                // );
                use dioxus::web::WebEventExt;
                use wasm_bindgen::JsCast;

                let web_event = event.as_web_event();
                let element: web_sys::HtmlElement = web_event
                    .dyn_into::<web_sys::HtmlElement>()
                    .expect("Expected HtmlElement");

                let columns = serde_json::json!(
                    [{ "headerName" : "Name", "field" : "name" }, { "headerName" : "Age", "field"
                    : "age" }]
                );
                let rows = serde_json::json!(
                    [{ "name" : "Alice", "age" : 30 }, { "name" : "Bob", "age" : 25 }]
                );
                loadAgGridAndCreate(
                    &element,
                    serde_wasm_bindgen::to_value(&columns).unwrap(),
                    serde_wasm_bindgen::to_value(&rows).unwrap(),
                );
            },
        }
    }
}

//const AGGRID_JS: Asset = asset!("/assets/ag-grid-community.umd.min.js");

#[cfg(feature = "web")]
//#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
export function loadAgGridAndCreate(el, columns, rows) {
    console.log('loadAgGridAndCreate');

    // if (!window.agGrid) {
    //     let script = document.createElement('script');
    //     script.src = 'https://cdn.jsdelivr.net/npm/ag-grid-community/dist/ag-grid-community.umd.min.js';
    //     script.onload = () => {
    //         createGrid(el, columns, rows);
    //     };
    //     document.head.appendChild(script);
    // } else {
    //     createGrid(el, columns, rows);
    // }

    createGrid(el, columns, rows);

    function createGrid(el, columns, rows) {
        console.log(el);
        agGrid.Grid(el, {    // 👈 window.agGrid.Grid
            columnDefs: columns,
            rowData: rows,
            defaultColDef: { sortable: true, filter: true, resizable: true }
        });
    }
}
")]
extern "C" {
    fn loadAgGridAndCreate(
        el: &web_sys::HtmlElement,
        columns: wasm_bindgen::JsValue,
        rows: wasm_bindgen::JsValue,
    );
}
