use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

fn log(s: &String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
pub fn main(name: &str) {
    if name == "--help" {
        log(&"index lang is a programming language for the web📚".to_string());
        log(&"-h --help         tool help".to_string());
        log(&"-v --version      compiler version\n".to_string());

        log(&"compiler commands".to_string());
        log(&"init              init project\n".to_string());

        log(&"compiler options".to_string());
        log(&"-d                directory\n".to_string());

        return;
    } else if name == "--version" || name == "-v" {
        log(&"index lang v0.0.13".to_string());
        return;
    } 
}

