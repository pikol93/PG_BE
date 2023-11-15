use std::collections::HashMap;

use log::{error, info, trace, warn};
use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{Html, Selector};
use tokio::task;

static CURRENT_PRICE_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("div.current-price>span").unwrap());
static DETAILS_TABLE_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("ul.data-sheet").unwrap());
static DETAILS_TABLE_PAIR_CONTAINER_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("li.data-item").unwrap());
static DETAILS_TABLE_NAME_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("dt").unwrap());
static DETAILS_TABLE_VALUE_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("dd").unwrap());
static VARIANT_DETAILS_TABLE_NAMES_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("table.variant-details>thead>tr>th").unwrap());
static VARIANT_DETAILS_TABLE_VALUES_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("table.variant-details>tbody>tr>td").unwrap());
static IMAGE_CONTAINER_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse("img.d-block").unwrap());

static PRODUCT_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\D*(\d+)").unwrap());

#[derive(Default)]
struct ImageUrls {
    medium_image_url: Option<String>,
    large_image_url: Option<String>,
}

pub async fn process_product_sites_to_products(
    product_urls: &Vec<String>,
    category_name: &'static str,
    subcategory_name: &'static str,
) -> Vec<HashMap<String, String>> {
    let mut join_handles = Vec::with_capacity(product_urls.len());
    for url in product_urls {
        let url_clone = url.clone();
        let join_handle = task::spawn(process_product_site_to_product(
            url_clone,
            category_name,
            subcategory_name,
        ));
        join_handles.push((url, join_handle));
    }

    let mut product_links = Vec::with_capacity(join_handles.len());
    for (url, join_handle) in join_handles {
        let Ok(await_result) = join_handle.await else {
            error!("Could not await the join handle for URL {}", url);
            continue;
        };

        let Ok(shop_product) = await_result else {
            warn!("Shop product scrapping failed for URL {}", url);
            continue;
        };

        trace!("URL {} returned a shop product {:?}", url, shop_product);

        product_links.push(shop_product);
    }

    product_links
}

async fn process_product_site_to_product(
    product_url: String,
    category_name: &str,
    subcategory_name: &str,
) -> Result<HashMap<String, String>, ()> {
    let Some(id) = fetch_id_from_product_url(&product_url) else {
        error!("Could not fetch ID from URL {}", product_url);
        return Err(());
    };

    let Ok(response) = reqwest::get(&product_url).await else {
        error!("Could not get response for URL {}", product_url);
        return Err(());
    };

    let Ok(response_text) = response.text().await else {
        error!("Could not get text from response from URL {}", product_url);
        return Err(());
    };

    let document = Html::parse_document(&response_text);
    let mut result;
    if let Ok(details_table) = fetch_info_from_details_table(&document) {
        result = details_table;
    } else {
        error!("Could not get details table from URL {}", product_url);
        result = HashMap::new();
    }

    if let Ok(variant_details_table) = fetch_info_from_variant_details_table(&document) {
        result.extend(variant_details_table);
    } else {
        error!(
            "Could not get variant details table from URL {}",
            product_url
        );
    }

    if let Ok(price_string) = fetch_price_info(&document) {
        result.insert("price".to_owned(), price_string);
    }

    let image_urls = fetch_image_urls(&document);
    if let Some(large_image_url) = image_urls.large_image_url {
        result.insert("large_image_url".to_owned(), large_image_url);
    } else {
        error!("Could not fetch large image URL.");
    }

    if let Some(medium_image_url) = image_urls.medium_image_url {
        result.insert("medium_image_url".to_owned(), medium_image_url);
    } else {
        error!("Could not fetch medium image URL.");
    }

    result.insert("id".to_owned(), id.to_string());
    result.insert("category".to_owned(), category_name.to_owned());
    result.insert("subcategory".to_owned(), subcategory_name.to_owned());

    Ok(result)
}

fn fetch_id_from_product_url(product_url: &str) -> Option<u32> {
    let Some(a) = PRODUCT_ID_REGEX.captures(product_url) else {
        warn!("Could not find ID in product URL {}", product_url);
        return None;
    };

    let (_, id_string_array) = a.extract::<1>();
    let id_string = id_string_array[0];
    let Ok(id) = id_string.parse::<u32>() else {
        warn!("Could not parse string to u32: \"{}\" from product URL \"{}\"", id_string, product_url);
        return None;
    };

    Some(id)
}

fn fetch_info_from_details_table(document: &Html) -> Result<HashMap<String, String>, ()> {
    let details_table = document.select(&DETAILS_TABLE_SELECTOR).next();

    let Some(details_table) = details_table else {
        error!("Could not fetch details table from document.");
        return Err(());
    };

    let mut result = HashMap::<String, String>::new();
    details_table
        .select(&DETAILS_TABLE_PAIR_CONTAINER_SELECTOR)
        .filter_map(|element| {
            let name_element_option = element.select(&DETAILS_TABLE_NAME_SELECTOR).next();
            let Some(name_element) = name_element_option else {
                return None;
            };

            let value_element_option = element.select(&DETAILS_TABLE_VALUE_SELECTOR).next();
            let Some(value_element) = value_element_option else {
                return None;
            };

            Some((
                name_element.inner_html(),
                validate_value(&value_element.inner_html()),
            ))
        })
        .for_each(|(key, value)| {
            result.insert(key, value);
        });

    Ok(result)
}

fn fetch_info_from_variant_details_table(document: &Html) -> Result<HashMap<String, String>, ()> {
    let variant_detail_names = document.select(&VARIANT_DETAILS_TABLE_NAMES_SELECTOR);
    let variant_detail_values = document.select(&VARIANT_DETAILS_TABLE_VALUES_SELECTOR);

    let mut result = HashMap::new();

    let names_iter = variant_detail_names.into_iter();
    let values_iter = variant_detail_values.into_iter();

    names_iter
        .zip(values_iter)
        .for_each(|(name_element, value_element)| {
            result.insert(
                name_element.inner_html(),
                validate_value(&value_element.inner_html()),
            );
        });

    Ok(result)
}

fn fetch_price_info(document: &Html) -> Result<String, ()> {
    let mut price_select = document.select(&CURRENT_PRICE_SELECTOR);

    let price_element_option = price_select.next();
    let Some(price_element) = price_element_option else {
        error!("Could not find HTML element for price element");
        return Err(());
    };

    let result_with_unicode = price_element.inner_html();

    if result_with_unicode.is_empty() {
        error!("Returned inner HTML is empty for price element");
        return Err(());
    }

    let result = result_with_unicode.replace("&nbsp;", " ");

    Ok(result)
}

fn fetch_image_urls(document: &Html) -> ImageUrls {
    let Some(selected) = document.select(&IMAGE_CONTAINER_SELECTOR).next() else {
        return ImageUrls::default();
    };

    ImageUrls {
        medium_image_url: selected
            .value()
            .attr("data-image-medium-src")
            .map(|url| url.to_owned()),
        large_image_url: selected
            .value()
            .attr("data-image-large-src")
            .map(|url| url.to_owned()),
    }
}

fn validate_value(current: &str) -> String {
    current.replace("<br>\n", ", ")
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use scraper::Html;

    use crate::product_site_processing::{
        fetch_info_from_details_table, fetch_info_from_variant_details_table,
    };

    use super::fetch_id_from_product_url;

    const PRODUCT_PAGE_PATH: &str = "example_html/product_page.html";

    #[test]
    fn should_find_id_in_product() {
        let product_url = "https://e-okularnicy.pl/strona-glowna/4940-solano-cl-10166-oprawki-okularowe-z-nakladka-przeciwsloneczna-z-polaryzacja-clip-on.html";
        let id = fetch_id_from_product_url(product_url).expect("Should be able to fetch ID");

        assert_eq!(4940, id);
    }

    #[test]
    fn should_fetch_details_table() {
        let product_page_html = fs::read_to_string(PRODUCT_PAGE_PATH)
            .expect("Should be able to read product page from file system.");

        let document = Html::parse_document(&product_page_html);
        let details_table = fetch_info_from_details_table(&document)
            .expect("Should be able to fetch info from details table");

        let mut expected = HashMap::new();
        expected.insert("Filtr UV".to_owned(), "TAK".to_owned());
        expected.insert(
            "Termin realizacji zamówienia".to_owned(),
            "2-3 dni robocze".to_owned(),
        );
        expected.insert("Materiał".to_owned(), "ultem".to_owned());
        expected.insert("Polaryzacja".to_owned(), "TAK".to_owned());
        expected.insert("Szerokość mostka".to_owned(), "18".to_owned());
        expected.insert("Przeznaczenie".to_owned(), "męskie".to_owned());
        expected.insert("Długość zausznika".to_owned(), "145".to_owned());
        expected.insert("Kształt".to_owned(), "Prostokątny".to_owned());
        expected.insert("Typ".to_owned(), "pełna".to_owned());
        expected.insert("Szerokość soczewki".to_owned(), "54".to_owned());

        assert_eq!(expected, details_table);
    }

    #[test]
    fn should_fetch_variant_details_table() {
        let product_page_html = fs::read_to_string(PRODUCT_PAGE_PATH)
            .expect("Should be able to read product page from file system.");

        let document = Html::parse_document(&product_page_html);
        let variant_details_table = fetch_info_from_variant_details_table(&document)
            .expect("Should be able to fetch info from details table");

        let mut expected = HashMap::new();
        expected.insert("Kolor".to_owned(), "Czarny".to_owned());
        expected.insert("Dodatkowy kolor".to_owned(), "Czerwony".to_owned());
        expected.insert("Kolor soczewki".to_owned(), "Szary".to_owned());
        expected.insert(
            "Dodatkowa opcja soczewki".to_owned(),
            "Antyrefleks".to_owned(),
        );

        assert_eq!(expected, variant_details_table);
    }
}
