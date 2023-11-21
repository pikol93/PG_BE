use log::{debug, error, info, trace, warn};
use once_cell::sync::Lazy;
use scraper::{node::Element, Html, Selector};
use tokio::task;

static SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("a.tile__product").unwrap());

pub async fn process_listing_urls_to_product_urls(urls: &Vec<String>) -> Vec<String> {
    let mut join_handles = Vec::with_capacity(urls.len());
    for url in urls {
        let url_clone = url.clone();
        let join_handle = task::spawn(process_url(url_clone));
        join_handles.push((url, join_handle));
    }

    let mut product_links: Vec<String> = vec![];
    for (url, join_handle) in join_handles {
        let Ok(await_result) = join_handle.await else {
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
    for ele in &product_links {
        trace!("Product link: {}", ele);
    }

    product_links
}

/// Scraps the page URL for product URLs.
async fn process_url(url: String) -> Result<Vec<String>, ()> {
    debug!("Began processing URL {}", url);

    let Ok(response) = reqwest::get(&url).await else {
        error!("Could not get response for URL {}", url);
        return Err(());
    };

    let Ok(response_text) = response.text().await else {
        error!("Could not get text from response from URL {}", url);
        return Err(());
    };

    let document = Html::parse_document(&response_text);
    let element_urls = document
        .select(&SELECTOR)
        .map(|selected| selected.value())
        .filter_map(|element| match get_product_url_from_element(element) {
            Ok(url) => Some(url.to_owned()),
            Err(_) => {
                warn!("Could not fetch URL for product.");
                None
            }
        })
        .collect();

    Ok(element_urls)
}

fn get_product_url_from_element(element: &Element) -> Result<&str, ()> {
    let a = element.attr("href");
    let Some(product_url) = a else {
        warn!("Could not find href");
        return Err(());
    };

    Ok(product_url)
}
