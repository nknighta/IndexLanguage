use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

fn log(s: &String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

