use wasm_bindgen::prelude::*;
use async_std::task;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    task::block_on(async {
        surf::get("https://httpbin.org/get").await.unwrap();
    });

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}
