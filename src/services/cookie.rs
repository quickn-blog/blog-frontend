use wasm_bindgen::JsCast;

#[derive(Clone, Copy)]
pub struct CookieService;

#[derive(Clone, Copy, PartialEq)]
pub enum CookieError {
    NotFound,
}

impl CookieService {
    pub fn new() -> Self {
        Self
    }

    pub fn set(&self, name: &str, value: &str) {
        self.set_cookie(name, value, 1);
    }

    pub fn get(&self, _name: &str) -> Result<String, CookieError> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
        let cookie_str = html_document.cookie().unwrap();
        //info!("{}", cookie_str);
        let cookies: Vec<&str> = cookie_str.split(';').collect();
        if let Some(cookie) = cookies.iter().nth(0) {
            if let Some(value) = cookie.split('=').nth(1) {
                Ok(value.to_string())
            } else {
                Err(CookieError::NotFound)
            }
        } else {
            Err(CookieError::NotFound)
        }
    }

    pub fn remove(&self, name: &str) {
        self.set_cookie(name, "", -1);
    }

    fn set_cookie(&self, name: &str, value: &str, days: i32) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
        html_document
            .set_cookie(&format!(
                "{}={}; max-age={}; SameSite=Lax;",
                name,
                value,
                days * 24 * 60 * 60
            ))
            .ok();
    }
}
