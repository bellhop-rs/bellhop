pub mod assets;
pub mod types;

use rocket::response::content::Html;

use url::Url;

#[derive(Debug, Serialize)]
pub struct Pages {
    #[serde(with = "url_serde")]
    next: Option<Url>,

    #[serde(with = "url_serde")]
    prev: Option<Url>,
}

#[derive(Debug, Serialize)]
pub struct Paged<T> {
    items: Vec<T>,
    pages: Pages,
}

impl<T> Paged<T> {
    fn new(items: Vec<T>) -> Self {
        Self {
            items,
            pages: Pages {
                next: None,
                prev: None,
            },
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/redoc_static.rs"));

#[get("/")]
pub fn docs() -> Html<&'static str> {
    Html(HTML)
}
