use std::{fs, path::Path};
use crate::utils;
use colored::Colorize;

pub async fn download_gelbooru(tags: String, page: i64, path: String, all: bool, proxy: Option<String>) {
    let path = format!("{path}/{tags}");
    println!("Downloading all pictures from tags {} to {}", tags.purple(), path.green());
    let client = imgdl_rs::gelbooru::request::Client::new(proxy.as_deref()); 
    let attributes = client.fetch_attributes(&tags, page).await.unwrap();
    let page_count = if all { (attributes.count / 100) as i64 } // Divide total post count by post limit per
                                                                // page (100)
    else { 1 };

    println!("Total pages: {page_count} (use `--all` flag to download from all pages)");
    if !Path::new(&path).exists() {
        fs::create_dir(&path).unwrap()
    }

    for p in 0..page_count {
        let response = client.fetch_posts(&tags, p).await.unwrap();
        for i in response.iter() { 
            utils::download_img_gelbooru(
            i.file_url.clone(), i.image.clone(), path.clone(), proxy.clone()).await.unwrap();
        }
    }
}
