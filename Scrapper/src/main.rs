use log::{debug, error, info, trace, warn};
use once_cell::sync::Lazy;
use scraper::node::Element;
use scraper::{Html, Selector};
use tokio::task;

use crate::logging::log_manager;

pub mod logging;

static SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("a.tile__product").unwrap());

#[tokio::main]
async fn main() {
    log_manager::initialize_logger();

    let urls = vec![
        "https://e-okularnicy.pl/10-oprawy?page=1",
        "https://e-okularnicy.pl/10-oprawy?page=2",
    ];

    let mut join_handles = Vec::with_capacity(urls.len());
    for ele in urls {
        let join_handle = task::spawn(process_url(ele));
        join_handles.push(join_handle);
    }

    let mut product_links: Vec<String> = vec![];
    for join_handle in join_handles {
        let Ok((url, await_result)) = join_handle.await else {
            error!("Could not await the join handle.");
            continue;
        };

        let Ok(mut url_product_links) = await_result else {
            warn!("Product link scrapping failed.");
            continue;
        };

        debug!(
            "URL {} returned {} product links",
            url,
            url_product_links.len()
        );

        product_links.append(&mut url_product_links);
    }

    info!("Found {} product URLs", product_links.len());
    for ele in product_links {
        trace!("Product link: {}", ele);
    }
}

/// Scraps the page URL for product URLs.
async fn process_url(url: &str) -> (&str, Result<Vec<String>, ()>) {
    debug!("Began processing URL {}", url);

    let Ok(response) = reqwest::get(url).await else {
        error!("Could not get response for URL {}", url);
        return (url, Err(()));
    };

    let Ok(response_text) = response.text().await else {
        error!("Could not get text from response from URL {}", url);
        return (url, Err(()));
    };

    let document = Html::parse_document(&response_text);
    let element_urls = document
        .select(&SELECTOR)
        .map(|selected| selected.value())
        .filter_map(|element| match get_product_url_from_element(element) {
            Ok(url) => Some(url),
            Err(_) => {
                warn!("Could not fetch URL for product.");
                None
            }
        })
        .collect();

    (url, Ok(element_urls))
}

fn get_product_url_from_element(element: &Element) -> Result<String, ()> {
    let a = element.attr("href");
    let Some(product_url) = a else {
        warn!("Could not find href");
        return Err(());
    };

    Ok(product_url.to_owned())
}
