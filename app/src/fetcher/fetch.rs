use gloo_net::http::Request;
use std::collections::HashMap;

// Fetch contents from url
pub fn prefetch_contents(url: &str) -> HashMap<u32, String> {
    // let contents = Request::get(url).send().await.unwrap().await.unwrap();
    let mut contents = HashMap::new();
    contents.insert(0, String::from("<h1>Test</h1>"));
    contents
}
