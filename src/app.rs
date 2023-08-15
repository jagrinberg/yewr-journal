use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::pages::journal::Journal;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        < Journal/>
    }
}
