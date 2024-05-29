use std::collections::HashSet;

extern crate serde;
use gloo_console::log;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use wasm_cookies as cookies;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct CookieProps {
    cookie: String,
}

impl CookieProps {
    fn setup_cookie() -> Self {
        let cookie_options = cookies::CookieOptions::default()
            .expires_after(core::time::Duration::from_secs(52 * 7 * 24 * 60 * 60));
        match cookies::get("test") {
            Some(Ok(cookie)) => {
                log!("got cookie");
                return Self { cookie: cookie };
            }
            Some(Err(e)) => {
                log!(format!("cookie error: {}", e));
            }
            None => {
                log!("did not find cookie");
            }
        }
        log!("setting cookie");
        cookies::set("test", "123", &cookie_options);
        let cookie = "123".to_string();
        // XXX Don't do this!! No secrets in logs!
        // log!(&cookie);
        Self { cookie }
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

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
struct JokeStruct {
    id: String,
    whos_there: String,
    answer_who: String,
    tags: Option<HashSet<String>>,
    source: Option<String>,
}

impl JokeStruct {
    async fn get_joke() -> Result<Self, gloo_net::Error> {
        http::Request::get("http://localhost:3000/api/v1/joke")
            .send()
            .await?
            .json()
            .await
    }
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
struct JokeProps {
    joke: JokeStruct,
}

#[function_component(Joke)]
fn joke(joke: &JokeProps) -> Html {
    html! {
        <div class="joke">
            <span class="teller">{"Knock-Knock!"}</span><br/>
            <span class="tellee">{"Who's there?"}</span><br/>
            <span class="teller">{joke.joke.whos_there.clone()}</span><br/>
            <span class="tellee">{format!("{} who?", &joke.joke.whos_there)}</span><br/>
            <span class="teller">{joke.joke.answer_who.clone()}</span><br/>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let cookie = use_state(|| CookieProps::setup_cookie());

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
        <div>
            <Cookie cookie={cookie.cookie.clone()} />
        </div>
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
