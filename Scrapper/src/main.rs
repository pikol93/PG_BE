use crate::logging::log_manager;
use crate::listing_site_processing::process_listing_urls_to_product_urls;

pub mod logging;
pub mod listing_site_processing;

#[tokio::main]
async fn main() {
    log_manager::initialize_logger();

    let urls = vec![
        "https://e-okularnicy.pl/10-oprawy?page=1".to_string(),
        "https://e-okularnicy.pl/10-oprawy?page=2".to_string(),
    ];

    process_listing_urls_to_product_urls(urls).await;
}