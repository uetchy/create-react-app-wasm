use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen]
extern "C" {
  // map [rust] fn alert(s: &str) => [ts] global.alert(s: string)
  fn alert(s: &str);

  // map [rust] jsLog(s: &str) => [ts] console.log(s: string)
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn jsLog(s: &str);
}

#[wasm_bindgen(module = "/../web/src/utils/text.ts")]
extern "C" {
  // map [rust] modLog(s: &str) => [ts] modLog(s: string) in @web/src/utils/text.rs
  fn modLog(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (jsLog(&format_args!($($t)*).to_string()))
}

// map [ts] greet(s: string, array: string[]): string => [rust] green(s: &str, array: JsValue) -> String
#[wasm_bindgen]
pub fn greet(s: &str, array: JsValue) -> String {
  // map [ts] string[] => [rust] Vec<String>
  web_sys::console::log_1(&"👍 web_sys::console::log_1".into());
  let elements: Vec<String> = array.into_serde().unwrap();
  modLog(s);
  for i in elements.iter() {
    modLog(i);
  }
  console_log!("🦄 console_log!");

  "Hello from Rust".into()
}
