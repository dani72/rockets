mod utils;
mod engine;
mod components;

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlImageElement};

use crate::engine::*;
use crate::components::*;

pub fn clone_sprite( image: &HtmlImageElement) -> HtmlImageElement{
    let document = window().unwrap().document().unwrap();
    let img1 = document.create_element("img").unwrap().dyn_into::<HtmlImageElement>().unwrap();
    img1.set_src( &image.src());

    return img1;
}

// For better error messages in case of panics
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
