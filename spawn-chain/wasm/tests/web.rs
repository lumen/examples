//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use std::sync::Once;

use wasm_bindgen::{JsCast, JsValue};

use wasm_bindgen_futures::JsFuture;

use wasm_bindgen_test::*;

use js_sys::Reflect;

use spawn_chain::start;

wasm_bindgen_test_configure!(run_in_browser);

mod run {
    use super::*;

    use spawn_chain::run;

    #[wasm_bindgen_test]
    async fn with_1() {
        eq(1).await;
    }

    #[wasm_bindgen_test]
    async fn with_2() {
        eq(2).await;
    }

    #[wasm_bindgen_test]
    async fn with_4() {
        eq(4).await;
    }

    #[wasm_bindgen_test]
    async fn with_8() {
        eq(8).await;
    }

    #[wasm_bindgen_test]
    async fn with_16() {
        eq(16).await;
    }

    async fn eq(n: usize) {
        super::eq(run, n).await;
    }
}

mod log_to_console {
    use super::*;

    use spawn_chain::log_to_console;

    #[wasm_bindgen_test]
    async fn with_1() {
        eq(1).await;
    }

    #[wasm_bindgen_test]
    async fn with_2() {
        eq(2).await;
    }

    #[wasm_bindgen_test]
    async fn with_4() {
        eq(4).await;
    }

    #[wasm_bindgen_test]
    async fn with_8() {
        eq(8).await;
    }

    #[wasm_bindgen_test]
    async fn with_16() {
        eq(16).await;
    }

    async fn eq(n: usize) {
        super::eq(log_to_console, n).await;
    }
}

mod log_to_dom {
    use super::*;

    use spawn_chain::log_to_dom;

    #[wasm_bindgen_test]
    async fn with_1() {
        eq(1).await;
    }

    #[wasm_bindgen_test]
    async fn with_2() {
        eq(2).await;
    }

    #[wasm_bindgen_test]
    async fn with_4() {
        eq(4).await;
    }

    #[wasm_bindgen_test]
    async fn with_8() {
        eq(8).await;
    }

    #[wasm_bindgen_test]
    async fn with_16() {
        eq(16).await;
    }

    async fn eq(n: usize) {
        super::eq(log_to_dom, n).await;
    }
}

static START: Once = Once::new();

async fn eq(f: fn(usize) -> js_sys::Promise, n: usize) {
    start_once();

    let promise = f(n);
    let resolved = JsFuture::from(promise).await.unwrap();

    assert!(
        js_sys::Array::is_array(&resolved),
        "{:?} is not an array",
        resolved
    );

    let resolved_array: js_sys::Array = resolved.dyn_into().unwrap();

    assert_eq!(resolved_array.length(), 2);

    let resolved_time = Reflect::get(&resolved_array, &0.into()).unwrap();

    assert!(js_sys::Number::is_integer(&resolved_time));

    let n_js_value: JsValue = (n as i32).into();
    assert_eq!(
        Reflect::get(&resolved_array, &1.into()).unwrap(),
        n_js_value
    );
}

fn start_once() {
    START.call_once(|| {
        start();
    })
}
