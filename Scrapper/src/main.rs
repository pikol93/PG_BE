use std::collections::HashMap;

use product_site_processing::process_product_sites_to_products;

use crate::listing_site_processing::process_listing_urls_to_product_urls;
use crate::logging::log_manager;

pub mod listing_site_processing;
pub mod logging;
pub mod product_site_processing;

#[tokio::main]
async fn main() {
    log_manager::initialize_logger();

    let no_information_string = "NO_INFORMATION".to_owned();

    let shades_pages =
        create_vec_with_pages("https://e-okularnicy.pl/12-okulary-przeciwsloneczne", 2);
    let shades_results = process(&shades_pages).await;
    let _shades_heatmap = collect_heatmap(&shades_results, &no_information_string);

    dbg!(&shades_results, &_shades_heatmap);

    /*
    let fixtures_pages = create_vec_with_pages("https://e-okularnicy.pl/10-oprawy", 11);
    let fixtures_results = process(&fixtures_pages).await;
    let _fixtures_heatmap = foo(&fixtures_results, &no_information_string);
    */
}

fn create_vec_with_pages(base_site: &str, page_count: u32) -> Vec<String> {
    let mut urls = vec![];
    for i in 1..page_count {
        urls.push(base_site.to_owned() + "?page=" + &i.to_string())
    }

    urls
}

fn collect_heatmap<'a>(
    shop_products: &'a Vec<HashMap<String, String>>,
    default_product_value: &'a String,
) -> HashMap<&'a String, HashMap<&'a String, u32>> {
    shop_products
        .iter()
        .flat_map(|product_items| product_items.keys())
        .map(|product_key| {
            let count = shop_products
                .iter()
                .map(|hash_map| hash_map.get(product_key))
                .map(|product_value| product_value.unwrap_or(default_product_value))
                .fold(HashMap::<&String, u32>::new(), |mut map, product_value| {
                    *map.entry(product_value).or_default() += 1;
                    map
                });
            (product_key, count)
        })
        .collect::<HashMap<_, _>>()
}

async fn process(urls: &Vec<String>) -> Vec<HashMap<String, String>> {
    let product_urls = process_listing_urls_to_product_urls(urls).await;
    process_product_sites_to_products(&product_urls).await
}
