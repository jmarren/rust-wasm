//! Wires up the page's DOM from Rust. Runs automatically when the wasm module loads.

use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlInputElement};

use crate::{count_primes, fibonacci, greet};

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;

    web_sys::console::log_1(&JsValue::from_str("hi from rust"));

    on_click(&document, "greet-btn", {
        let document = document.clone();
        move || {
            let name = input_value(&document, "name");
            set_text(&document, "greet-out", &greet(&name));
        }
    })?;

    on_click(&document, "fib-btn", {
        let document = document.clone();
        move || {
            let n = input_value(&document, "fib-n").parse().unwrap_or(0);
            set_text(
                &document,
                "fib-out",
                &format!("fib({n}) = {}", fibonacci(n)),
            );
        }
    })?;

    on_click(&document, "prime-btn", {
        let document = document.clone();
        let performance = window.performance().ok_or("no performance")?;
        move || {
            let limit = input_value(&document, "prime-limit").parse().unwrap_or(0);
            let start = performance.now();
            let count = count_primes(limit);
            let ms = performance.now() - start;
            set_text(
                &document,
                "prime-out",
                &format!("{count} primes below {limit} in {ms:.1} ms"),
            );
        }
    })?;

    Ok(())
}

/// Attaches a click handler to the element with the given id.
fn on_click(document: &Document, id: &str, handler: impl FnMut() + 'static) -> Result<(), JsValue> {
    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| format!("missing #{id}"))?;
    let closure = Closure::<dyn FnMut()>::new(handler);
    element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    // The listener lives for the whole page, so hand ownership to JS.
    closure.forget();
    Ok(())
}

fn input_value(document: &Document, id: &str) -> String {
    document
        .get_element_by_id(id)
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}

fn set_text(document: &Document, id: &str, text: &str) {
    if let Some(el) = document.get_element_by_id(id) {
        el.set_text_content(Some(text));
    }
}
