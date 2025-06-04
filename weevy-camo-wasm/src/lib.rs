use std::borrow::Cow;

use wasm_bindgen::prelude::*;
use weevy_camo::*;
#[wasm_bindgen]
pub struct PropRewriter {
    cfg: Config<String>,
}
#[wasm_bindgen]
impl PropRewriter {
    #[wasm_bindgen(constructor)]
    pub fn new(a: &str) -> Self {
        Self {
            cfg: Config {
                isolate: a.to_owned(),
            },
        }
    }
    pub fn rewrite(&self, a: JsValue) -> JsValue {
        let Some(s) = a.as_string() else {
            return a;
        };
        let Cow::Owned(s) = self.cfg.rewrite(&s) else {
            return a;
        };
        return JsValue::from_str(&s);
    }
    pub fn unrewrite(&self, a: JsValue) -> JsValue {
        let Some(s) = a.as_string() else {
            return a;
        };
        let s = self.cfg.unrewrite(&s);
        return JsValue::from_str(s);
    }
}
