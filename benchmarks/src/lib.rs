use regex::Regex;
use serde::Deserialize;
use wasm_bindgen::prelude::*;


#[derive(Deserialize)]
pub struct VlmConfig {
    rules: Vec<String>,
}

pub fn vlm_config() -> VlmConfig {
    let config = r#"
    {
        "rules": [
            "allow",
            "deny"
        ]
    }
    "#;
    serde_json::from_str(config).unwrap()
}

#[wasm_bindgen]
pub fn vlm_wasm() {
    let cfg: VlmConfig = vlm_config();
    // Optionally, precompile the regexes and return some meaningful result.
    let compiled: Vec<String> = cfg.rules.into_iter().map(|rule| {
    // Compile to ensure validity.
    Regex::new(&rule).expect("Invalid regex");
    rule
}).collect();
    let result = compiled.join(",");
    JsValue::from_str(&result);
}