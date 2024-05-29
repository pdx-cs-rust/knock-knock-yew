use crate::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CookieProps {
    pub cookie: String,
}

impl CookieProps {
    pub fn acquire_cookie() -> Self {
        let cookie_options = cookies::CookieOptions::default()
            .expires_after(core::time::Duration::from_secs(
                52 * 7 * 24 * 60 * 60
            ));
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
pub fn cookie(CookieProps { cookie }: &CookieProps) -> Html {
    html! {
        <div>
            <p>{cookie.clone()}</p>
        </div>
    }
}
