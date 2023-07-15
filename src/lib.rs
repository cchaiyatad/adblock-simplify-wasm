extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

extern crate adblock;
extern crate js_sys;
extern crate serde_json;

use adblock::engine::Engine;
use adblock::lists::{FilterSet, ParseOptions};

#[wasm_bindgen]
pub struct AdsChecker {
    engine: Engine,
}

#[wasm_bindgen]
impl AdsChecker {
    pub fn new(rules_js: &JsValue, debug_info: bool) -> AdsChecker {
        let iterator = js_sys::try_iter(rules_js).unwrap().unwrap();

        let mut rules = Vec::new();
        for x in iterator {
            let x = x.unwrap();

            if x.is_string() {
                let rule = x.as_string().unwrap();
                rules.push(rule);
            }
        }

        let mut filter_set = FilterSet::new(debug_info);
        filter_set.add_filters(&rules, ParseOptions::default());

        let engine = Engine::from_filter_set(filter_set, true);
        AdsChecker { engine }
    }

    pub fn check_network_urls(&self, url: &str, source_url: &str, request_type: &str) -> String {
        let result: adblock::blocker::BlockerResult =
            self.engine
                .check_network_urls(&url, &source_url, &request_type);

        serde_json::to_string(&result).expect("Failed to serialize object to JSON")
    }
}
