mod cookie;
mod joke;

use cookie::*;
use joke::*;

use std::collections::HashSet;

extern crate serde;
use gloo_console::log;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use wasm_cookies as cookies;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let cookie = use_state(|| CookieProps::acquire_cookie());

    let joke = use_state(|| Err(gloo_net::Error::GlooError("uninit".to_string())));
    let get_joke = joke.clone();
    use_effect_with((), move |()| {
        wasm_bindgen_futures::spawn_local(async move {
            let joke = JokeStruct::get_joke().await;
            get_joke.set(joke);
        });
        || ()
    });

    html! {
    <>
        <h1>{ "Knock-Knock" }</h1>
        if false {
            <div>
                <Cookie cookie={cookie.cookie.clone()} />
            </div>
        }
        if let Ok(ref joke) = *joke {
            <Joke joke={joke.clone()}/>
        }
        if let Err(ref error) = *joke {
            <div>
                <p>{error.to_string().clone()}</p>
            </div>
        }
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
