use serde::Serialize;

#[derive(Debug, Serialize)]

pub enum SysError {
    SomeError(String),
}

impl From<String> for SysError {
    fn from(s: String) -> Self {
        SysError::SomeError(s)
    }
}

impl From<wasm_bindgen::JsValue> for SysError {
    fn from(js_value: wasm_bindgen::JsValue) -> Self {
        SysError::SomeError(js_value.as_string().unwrap())
    }
}
