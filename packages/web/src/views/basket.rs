use dioxus::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
#[cfg(feature = "web")]
use wasm_bindgen::JsValue;

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

const AGGRID_JS: Asset = asset!("/assets/ag-grid-community.min.js");

#[component]
pub fn GridComponent() -> Element {
    #[cfg(not(feature = "web"))]
    rsx! {}

    #[cfg(feature = "web")]
    rsx! {
        document::Script { src: AGGRID_JS }

        div {
            //class: "ag-theme-alpine",
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

                let mut columns = HashMap::<String, String>::new();
                columns.insert("headerName".into(), "Name".into());
                columns.insert("field".into(), "name".into());

                let columns = vec![
                    ColumnDef {
                        header_name: Some("Index".into()),
                        field: Some("index".into()),
                        ..Default::default()
                    },
                    ColumnDef {
                        header_name: Some("Name".into()),
                        field: Some("name".into()),
                        ..Default::default()
                    },
                    ColumnDef {
                        header_name: Some("Age".into()),
                        field: Some("age".into()),
                        ..Default::default()
                    },
                    ColumnDef {
                        header_name: Some("Birth City".into()),
                        field: Some("birthCity".into()),
                        ..Default::default()
                    },
                ];

                let rows = (0..100000)
                    .map(|i| Row {
                        index: i,
                        name: "Alice".into(),
                        age: 30,
                        birth_city: "Paris".into(),
                    })
                    .collect::<Vec<_>>();

                // //vec![
                //     Row {
                //         name: "Alice".into(),
                //         age: 30,
                //         birth_city: "Paris".into(),
                //     },
                //     Row {
                //         name: "Bob".into(),
                //         age: 500,
                //         birth_city: "Lyon".into(),
                //     },
                // ];

                let gridOption = GridOptions {
                    theme: THEME_ALPINE.with(|v| v.clone()),
                    column_defs: Some(columns.clone()),
                    row_data: Some(rows),
                    default_col_def: Some(ColumnDef {
                        sortable: Some(true),
                        filter: Some(true),
                        resizable: Some(true),
                        ..Default::default()
                    }),
                };
                loadAgGridAndCreate(
                    &element,
                    serde_wasm_bindgen::to_value(&gridOption).unwrap(),
                );
                return;
                let agGridOptions = AgGridOptions::new();
                agGridOptions.set_column_defs(&serde_wasm_bindgen::to_value(&columns).unwrap());
                agGridOptions
                    .set_row_data(&serde_wasm_bindgen::to_value(&rows.clone()).unwrap());
                agGridOptions
                    .set_default_col_def(
                        &serde_wasm_bindgen::to_value(
                                &ColumnDef {
                                    sortable: Some(true),
                                    filter: Some(true),
                                    resizable: Some(true),
                                    ..Default::default()
                                },
                            )
                            .unwrap(),
                    );
            },
        }
    }
}

#[cfg(feature = "web")]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct GridOptions {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    theme: JsValue,
    column_defs: Option<Vec<ColumnDef>>,
    row_data: Option<Vec<Row>>,
    default_col_def: Option<ColumnDef>,
}

#[cfg(feature = "web")]
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct ColumnDef {
    header_name: Option<String>,
    field: Option<String>,
    sortable: Option<bool>,
    filter: Option<bool>,
    resizable: Option<bool>,
}

#[cfg(feature = "web")]
impl Into<wasm_bindgen::JsValue> for ColumnDef {
    fn into(self) -> wasm_bindgen::JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Row {
    index: i32,
    name: String,
    age: i32,
    birth_city: String,
}

//const AGGRID_JS: Asset = asset!("/assets/ag-grid-community.umd.min.js");

#[cfg(feature = "web")]
//#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
export function loadAgGridAndCreate(el, gridOptions) {
    console.log('loadAgGridAndCreate');
    console.log(el);
    console.log(gridOptions);

    class Person {
    // Public fields
        name;
        age = 0;

        // constructor(name, age) {
        //     this.name = name;
        //     this.age = age;
        // }

        greet() {
            console.log(`Hi, I'm ${this.name} and I'm ${this.age}.`);
        }
    }

    const person = new Person();
    person.greet();
    console.log(person);

    if (!window.agGrid) {
        let script = document.createElement('script');
        script.src = 'https://cdn.jsdelivr.net/npm/ag-grid-community/dist/ag-grid-community.umd.min.js';
        script.onload = () => {
            console.log('grid loaded');
            let g = agGrid.createGrid(el, gridOptions);
            console.log('grid created');
            console.log(g);
        };
        document.head.appendChild(script);
    } else {
        let g = agGrid.createGrid(el, gridOptions);
        console.log('grid created');
        console.log(g);
        let go = g.getGridOption();
        console.log(go);
    }

    return;

    //createGrid(el, columns, rows);

    function createGrid(el, columns, rows) {
        const myTheme = agGrid.themeAlpine;
        // myTheme.withParams({
        //     /* Low spacing = very compact */
        //     spacing: 2,
        //     /* Changes the color of the grid text */
        //     foregroundColor: 'rgb(14, 68, 145)',
        //     /* Changes the color of the grid background */
        //     backgroundColor: 'rgb(241, 247, 255)',
        //     /* Changes the header color of the top row */
        //     headerBackgroundColor: 'rgb(228, 237, 250)',
        //     /* Changes the hover color of the row*/
        //     rowHoverColor: 'rgb(216, 226, 255)',
        // });

        const c = [{ 'headerName' : 'Name', 'field' : 'name' }, { 'headerName' : 'Age', 'field': 'age' }]
        const d = [{ 'name' : 'Alice', 'age' : 30 }, { 'name' : 'Bob', 'age' : 25 }]
        console.log(c);
        console.log(d);
        const gridOption = {    // 👈 window.agGrid.Grid
            theme: myTheme,
            columnDefs: columns,
            rowData: rows,
            defaultColDef: { sortable: true, filter: true, resizable: true }
        }
        agGrid.createGrid(el, gridOption);
    }
}
")]
extern "C" {
    fn loadAgGridAndCreate(el: &web_sys::HtmlElement, grid_options: wasm_bindgen::JsValue);
}

#[cfg(feature = "web")]
//#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
#[wasm_bindgen::prelude::wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_namespace = "agGrid", js_name = "GridOptions")]
    type AgGridOptions;
    #[wasm_bindgen(constructor)]
    fn new() -> AgGridOptions;
    #[wasm_bindgen(method, setter)]
    fn set_theme(this: &AgGridOptions, theme: &str);
    #[wasm_bindgen(method, setter)]
    fn set_column_defs(this: &AgGridOptions, column_defs: &wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter)]
    fn set_row_data(this: &AgGridOptions, row_data: &wasm_bindgen::JsValue);
    #[wasm_bindgen(method, setter)]
    fn set_default_col_def(this: &AgGridOptions, default_col_def: &wasm_bindgen::JsValue);
}

// #[cfg(feature = "web")]
// //#[wasm_bindgen::prelude::wasm_bindgen(inline_js = "
// #[wasm_bindgen::prelude::wasm_bindgen()]
// extern "C" {
//     #[wasm_bindgen(js_name = "agGrid")]
//     type AgGrid;
//     #[wasm_bindgen(static_field, js_name = "themeAlpine")]
//     fn theme_alpine() -> wasm_bindgen::JsValue;
// }

#[cfg(feature = "web")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(
        thread_local_v2,
        js_namespace = "agGrid",
        js_name = "themeAlpine"
    )]
    static THEME_ALPINE: wasm_bindgen::JsValue;

    #[wasm_bindgen::prelude::wasm_bindgen(
        thread_local_v2,
        js_namespace = "agGrid",
        js_name = "themeBalham"
    )]
    static THEME_BALHAM: wasm_bindgen::JsValue;

    #[wasm_bindgen::prelude::wasm_bindgen(
        thread_local_v2,
        js_namespace = "agGrid",
        js_name = "themeMaterial"
    )]
    static THEME_MATERIAL: wasm_bindgen::JsValue;

    #[wasm_bindgen::prelude::wasm_bindgen(
        thread_local_v2,
        js_namespace = "agGrid",
        js_name = "themeQuartz"
    )]
    static THEME_QUARTZ: wasm_bindgen::JsValue;
}
