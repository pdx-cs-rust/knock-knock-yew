extern crate serde;
use gloo_console::log;
extern crate wasm_bindgen_futures;
extern crate wasm_cookies;
use yew::prelude::*;
use std::sync::Arc;

#[derive(Properties, Clone, PartialEq)]
struct CookieProps {
    cookie: Arc<String>,
}

impl CookieProps {
    fn setup_cookie() -> Self {
        let cookie_options = wasm_cookies::CookieOptions::default()
            .expires_after(core::time::Duration::from_secs(52 * 7 * 24 * 60 * 60));
        let cookie = wasm_cookies::get("test");
        let cookie = if let Some(cookie) = cookie {
            cookie.unwrap()
        } else {
            log!("setting cookie");
            wasm_cookies::set("test", "123", &cookie_options);
            "123".to_string()
        };
        log!(&cookie);
        Self { cookie: Arc::new(cookie) }
    }
}    

#[function_component(Cookie)]
fn cookie(CookieProps { cookie }: &CookieProps) -> Html {
    html! {
        <div>
            <p>{cookie.clone()}</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let cookie = use_state(|| CookieProps::setup_cookie());

    html! {
    <>
        <h1>{ "Knock-Knock" }</h1>
        <div>
            <Cookie cookie={cookie.cookie.clone()} />
        </div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
