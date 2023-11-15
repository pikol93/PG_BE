use log::{warn, debug};
use reqwest::blocking;
use scraper::{Html, Selector};
use scraper::node::Element;

use crate::logging::log_manager;

pub mod logging;

fn main() {
    log_manager::initialize_logger();

    let response = blocking::get("https://e-okularnicy.pl/10-oprawy")
        .expect("Should get response from the url");
    let response_text = response.text()
        .expect("Result should be text.");

    parse_page(&response_text);
}

fn parse_page(page_string: &String) {
    let selector = Selector::parse("a.tile__product").unwrap();

    let document = Html::parse_document(&page_string);
    document
        .select(&selector)
        .map(|selected| selected.value())
        .for_each(parse_item);
}

fn parse_item(element: &Element) {
    let a = element.attr("href");
    let Some(b) = a else {
        warn!("Could not find href");
        return;
    };

    debug!("item href: {}", b);
}
