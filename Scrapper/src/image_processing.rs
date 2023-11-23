use log::{debug, error};
use std::collections::HashMap;
use std::fs;
use tokio::task;

pub async fn fetch_images_from_products(
    output_directory: &'static str,
    scrapped_products: &[HashMap<String, String>],
) {
    let join_handles = scrapped_products
        .iter()
        .map(|product| {
            let id = product.get("id").unwrap().clone();
            let medium_image_url = product.get("medium_image_url").cloned();
            let large_image_url = product.get("large_image_url").cloned();
            task::spawn(process_product_async(
                output_directory,
                id,
                medium_image_url,
                large_image_url,
            ))
        })
        .collect::<Vec<_>>();

    for join_handle in join_handles {
        let Ok(_) = join_handle.await else {
            error!("Image processing returned an error.");
            continue;
        };
    }
}

async fn process_product_async(
    output_directory: &'static str,
    id: String,
    medium_image_url: Option<String>,
    large_image_url: Option<String>,
) {
    let path = output_directory.to_string() + &id + "/";
    fs::create_dir_all(&path).expect("Could not create an output directory.");

    if medium_image_url.is_some() {
        let image_url = medium_image_url.unwrap();
        let image_path = path.clone() + "medium.jpg";
        download_image(&image_path, &image_url).await;
    }

    if large_image_url.is_some() {
        let image_url = large_image_url.unwrap();
        let image_path = path.clone() + "large.jpg";
        download_image(&image_path, &image_url).await;
    }
}

async fn download_image(output_path: &str, image_url: &str) {
    debug!("Begin downloading image {} to {}", image_url, output_path);

    let Ok(a) = reqwest::get(image_url).await else {
        error!("Could not download an image from {}", image_url);
        return;
    };

    let Ok(bytes) = a.bytes().await else {
        error!("Could not convert downloaded image to bytes.");
        return;
    };

    fs::write(output_path, bytes).expect("Could not write image to file system.");

    debug!("Finished downloading an image to {}", output_path);
}
