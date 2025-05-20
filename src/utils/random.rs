use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

pub fn random_number() -> f64 {
    random() as f64
}

pub fn random_number_max( max : f64) -> f64 {
    random() as f64 * max
}
