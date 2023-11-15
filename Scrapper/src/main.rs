use log::info;
use product_site_processing::process_product_sites_to_products;

use crate::listing_site_processing::process_listing_urls_to_product_urls;
use crate::logging::log_manager;

pub mod listing_site_processing;
pub mod logging;
pub mod product_site_processing;
pub mod shop_product;

#[tokio::main]
async fn main() {
    log_manager::initialize_logger();

    let urls = vec![
        "https://e-okularnicy.pl/10-oprawy?page=1".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=2".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=3".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=4".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=5".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=6".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=7".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=8".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=9".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=10".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=11".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=12".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=13".to_string(),
    ];

    let product_urls = process_listing_urls_to_product_urls(urls).await;
    let shop_products = process_product_sites_to_products(product_urls).await;

    info!("Shop products: {:#?}", shop_products);
}
