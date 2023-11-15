use log::{error, info};
use std::collections::HashMap;
use std::fs;
use tokio::task;

use product_site_processing::process_product_sites_to_products;

use crate::listing_site_processing::process_listing_urls_to_product_urls;
use crate::logging::log_manager;

pub mod listing_site_processing;
pub mod logging;
pub mod product_site_processing;

struct Category {
    name: &'static str,
    subcategories: Vec<Subcategory>,
}

struct Subcategory {
    name: &'static str,
    url: &'static str,
}

const DEFAULT_PAGE_COUNT: usize = 1;

#[tokio::main]
async fn main() {
    log_manager::initialize_logger();

    let categories = get_categories();
    let join_handles = categories
        .iter()
        .flat_map(|category| {
            category
                .subcategories
                .iter()
                .map(move |subcategory| (category, subcategory))
        })
        .map(|(category, subcategory)| {
            task::spawn(process(subcategory.url, category.name, subcategory.name))
        })
        .collect::<Vec<_>>();

    let mut scrapped_products = vec![];
    for join_handle in join_handles {
        let Ok(mut process_result) = join_handle.await else {
            error!("Process returned an error.");
            continue;
        };

        scrapped_products.append(&mut process_result);
    }

    let products_json =
        serde_json::to_string(&scrapped_products).expect("Should be able to write data to JSON.");
    fs::write("output/scrapped.txt", &products_json).expect("Should be able to write to disk.");

    info!(
        "Finished scrapping. Results: {} products scrapped.",
        scrapped_products.len()
    );
}

fn create_vec_with_pages(base_site: &str, page_count: usize) -> Vec<String> {
    let mut urls = Vec::with_capacity(page_count);
    for i in 1..=page_count {
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

async fn process(
    base_url: &str,
    category_name: &'static str,
    subcategory_name: &'static str,
) -> Vec<HashMap<String, String>> {
    let urls = create_vec_with_pages(base_url, DEFAULT_PAGE_COUNT);
    let product_urls = process_listing_urls_to_product_urls(&urls).await;
    process_product_sites_to_products(&product_urls, category_name, subcategory_name).await
}

fn get_categories() -> Vec<Category> {
    vec![
        Category {
            name: "Oprawy",
            subcategories: vec![
                Subcategory {
                    name: "Damskie",
                    url: "https://e-okularnicy.pl/16-damskie",
                },
                Subcategory {
                    name: "Męskie",
                    url: "https://e-okularnicy.pl/17-meskie",
                },
                Subcategory {
                    name: "Dziecięce",
                    url: "https://e-okularnicy.pl/18-dzieciece",
                },
                Subcategory {
                    name: "Sportowe",
                    url: "https://e-okularnicy.pl/19-sportowe",
                },
            ],
        },
        Category {
            name: "Soczewki okularowe",
            subcategories: vec![
                Subcategory {
                    name: "Ogólne",
                    url: "https://e-okularnicy.pl/21-ogolne",
                },
                Subcategory {
                    name: "Do komputera",
                    url: "https://e-okularnicy.pl/22-do-komputera",
                },
                Subcategory {
                    name: "Dla kierowców",
                    url: "https://e-okularnicy.pl/23-dla-kierowcow",
                },
                Subcategory {
                    name: "Dla dzieci",
                    url: "https://e-okularnicy.pl/24-dla-dzieci",
                },
            ],
        },
        Category {
            name: "Okulary przeciwsłoneczne",
            subcategories: vec![
                Subcategory {
                    name: "Damskie",
                    url: "https://e-okularnicy.pl/30-damskie",
                },
                Subcategory {
                    name: "Męskie",
                    url: "https://e-okularnicy.pl/31-meskie",
                },
                Subcategory {
                    name: "Dziecięce",
                    url: "https://e-okularnicy.pl/32-dzieciece",
                },
                Subcategory {
                    name: "Sportowe",
                    url: "https://e-okularnicy.pl/33-sportowe",
                },
            ],
        },
        Category {
            name: "Okulary specjalne",
            subcategories: vec![
                Subcategory {
                    name: "Ochronne",
                    url: "https://e-okularnicy.pl/34-ochronne",
                },
                Subcategory {
                    name: "Do pływania",
                    url: "https://e-okularnicy.pl/35-do-plywania",
                },
                Subcategory {
                    name: "Do makijażu",
                    url: "https://e-okularnicy.pl/35-do-plywania",
                },
                Subcategory {
                    name: "Gogle",
                    url: "https://e-okularnicy.pl/37-gogle",
                },
            ],
        },
        Category {
            name: "Akcesoria",
            subcategories: vec![
                Subcategory {
                    name: "Pielęgnacja",
                    url: "https://e-okularnicy.pl/38-pielegnacja",
                },
                Subcategory {
                    name: "Dodatki",
                    url: "https://e-okularnicy.pl/39-dodatki",
                },
                Subcategory {
                    name: "Etui",
                    url: "https://e-okularnicy.pl/40-etui",
                },
                Subcategory {
                    name: "Plastry na oko",
                    url: "https://e-okularnicy.pl/41-plastry-na-oko",
                },
            ],
        },
    ]
}
