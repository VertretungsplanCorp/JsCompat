use js_sys::JsString;
use regex::Regex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;
use web_sys::{Request, RequestInit, RequestMode, Response};

// pub use vp_api::*;

#[wasm_bindgen]
pub struct Server {
    url: String,
}

#[wasm_bindgen]
impl Server {
    #[wasm_bindgen(constructor)]
    pub fn new(url: &str) -> Self {
        let re = Regex::new(r"^(https?://[^/]+)").unwrap();
        let clean_url = re.captures(url).map(|caps| caps[1].to_string()).unwrap();
        Self { url: clean_url }
    }
    pub async fn get(&self, path: &str) -> Result<Response, JsValue> {
        let url = format!("{}{}", self.url, path);
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(&url, &opts)?;
        Self::request(request).await
    }
    pub async fn request(request: Request) -> Result<Response, JsValue> {
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        Ok(resp_value.dyn_into().unwrap())
    }
    #[wasm_bindgen(getter)]
    pub async fn ping(&self) -> JsString {
        let response: Response = self.get("/ping").await.unwrap();
        let promise: JsFuture = response.text().expect("no text").into();
        promise.await.unwrap().into()
    }
    #[wasm_bindgen]
    pub async fn getKlasse(&self, klasse: &str) -> JsString {
        let response: Response = self
            .get(&format!("/get_klasse?klasse={klasse}"))
            .await
            .unwrap();
        let promise: JsFuture = response.json().expect("no json").into();
        promise.await.unwrap().into()
    }
}
