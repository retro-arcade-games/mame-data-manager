use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;
use url::Url;

/**
 * Get the data source URL for a given URL and matching string
 */
pub fn get_data_source(url: &str, matching: &str) -> Result<String, Box<dyn Error>> {
    // Download the HTML
    let client = Client::new();
    let response = client.get(url).send()?;
    let body = response.text()?;
    
    // Parse the HTML
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();
    
    // Find the matching source
    let mut source: Option<String> = None;
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href.contains(matching) && (href.ends_with("zip") || href.ends_with("7z")) {
                source = Some(href.to_string());
            }
        }
    }
    
    // If a source was found, return it
    if let Some(mut source) = source {
        if !source.starts_with("http") {
            let url_obj = Url::parse(url)?;
            let base = format!("{}://{}", url_obj.scheme(), url_obj.host_str().unwrap());
            let slash = if !url_obj.path().ends_with('/') && !source.starts_with('/') {
                "/"
            } else {
                ""
            };
            source = format!("{}{}{}", base, slash, source);
        }
        Ok(source)
    } else {
        Err("No matching source found".into())
    }
}
