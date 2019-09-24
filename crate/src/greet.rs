extern crate serde;
extern crate serde_json;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn jsLog(s: &str);
}

#[wasm_bindgen(module = "/../web/src/utils/text.ts")]
extern "C" {
  fn modLog(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (jsLog(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(s: &str, array: JsValue) -> String {
  let elements: Vec<String> = array.into_serde().unwrap();
  web_sys::console::log_1(&format!("{} -> {}", "👍 web_sys::console::log_1", s).into());
  for i in elements.iter() {
    modLog(i);
  }
  console_log!("🦄 console_log!");

  "Hello from Rust".into()
}
