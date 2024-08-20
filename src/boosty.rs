use std::{future::Future, pin::Pin};

use futures::{future::join_all, io::Window};
use imgdl_rs::boosty::{auth::Auth, types::PlayerUrls};
use colored::Colorize;

use crate::{utils, BoxedFuture};

pub fn get_best_quality(urls: &Vec<PlayerUrls>) -> Option<&PlayerUrls> {
    let mut opt = None;
    
    for url in urls {
        match &*url.content_type {
            "ultra_hd" => return Some(url),
            "full_hd" => {
                opt = Some(url);
                break;
            },
            another => {
                #[cfg(test)]
                println!("Another content type: {}", another);
            }
        }
    }

    opt
}

pub async fn download_boosty_blog(blog: String, path: String, access_token: Option<String>, limit: i64, skip: i64, photo_only: bool, video_only: bool) {
    let auth = access_token.map(Auth::new);
    println!("Downloading content from {} to {}", blog.purple(), path.green());
    let response = imgdl_rs::boosty::request::Client::fetch_posts(
        blog.clone(), limit + skip, auth.clone()).await.unwrap();
    println!("Total count: {}, limit: {}", response.len() - skip as usize, limit);
    
    std::fs::create_dir_all(path.clone()).unwrap();

    let mut skipped = 0;
        
    let image_futures: Vec<_> = response.iter().flat_map(|post| {
        // Process data for paid or free posts
        let data_futures: Vec<BoxedFuture> = post.data.as_ref().map_or_else(Vec::new, |data| {
            data.iter().flat_map(|content| {
                let mut futures: Vec<BoxedFuture> = Vec::new();

                if let Some(url) = &content.url {
                    if url.starts_with("https://images.boosty.to/image/") && !video_only {
                        futures.push(Box::pin(utils::download_img_boosty(url.clone(), path.clone())));
                    } else if content.content_type == "ok_video" && !photo_only {
                        if let Some(player_urls) = &content.player_urls {
                            if let Some(player) = get_best_quality(player_urls) {
                                if skipped == skip {
                                    println!("Content type: {}", player.content_type);
                                    futures.push(Box::pin(utils::download_video(player.url.clone(), path.clone())));
                                } else {
                                    println!("Skipping {}", player.url);
                                    skipped += 1;   
                                }
                            }
                        }
                    }
                }

                futures
            }).collect()
        });

        // Process teaser data for paid posts only if !video_only
        let teaser_futures: Vec<BoxedFuture> = if !video_only {
            post.teaser.iter().filter_map(|teaser| {
                teaser.url.as_ref().map(|url| {
                    Box::pin(utils::download_img_boosty(url.clone(), path.clone())) as Pin<Box<dyn Future<Output = Result<(), utils::Error>> + Send>>
                })
            }).collect()
        } else {
            Vec::new()
        };

        data_futures.into_iter().chain(teaser_futures.into_iter()).collect::<Vec<_>>()
    }).collect();

    join_all(image_futures).await.into_iter().for_each(|result| {
        if let Err(e) = result {
            eprintln!("Error occurred: {:?}", e);
        }
    });
}

pub async fn download_boosty_post(blog: String, post: String, path: String, access_token: Option<String>, photo_only: bool, video_only: bool) {
    let auth = access_token.map(Auth::new);

    let post = imgdl_rs::boosty::request::Client::fetch_post(
        blog, post, auth
    ).await.unwrap();

    let image_futures: Vec<_> = {
        // Process data for paid or free posts
        let data_futures: Vec<BoxedFuture> = post.data.as_ref().map_or_else(Vec::new, |data| {
            data.iter().flat_map(|content| {
                let mut futures: Vec<BoxedFuture> = Vec::new();

                if let Some(url) = &content.url {
                    if url.starts_with("https://images.boosty.to/image/") && !video_only {
                        futures.push(Box::pin(utils::download_img_boosty(url.clone(), path.clone())));
                    } else if content.content_type == "ok_video" && !photo_only {
                        if let Some(player_urls) = &content.player_urls {
                            if let Some(player) = get_best_quality(player_urls) {
                                futures.push(Box::pin(utils::download_video(player.url.clone(), path.clone())));
                            }
                        }
                    }
                }

                futures
            }).collect()
        });

        // Process teaser data for paid posts only if !video_only
        let teaser_futures: Vec<BoxedFuture> = if !video_only {
            post.teaser.iter().filter_map(|teaser| {
                teaser.url.as_ref().map(|url| {
                    Box::pin(utils::download_img_boosty(url.clone(), path.clone())) as Pin<Box<dyn Future<Output = Result<(), utils::Error>> + Send>>
                })
            }).collect()
        } else {
            Vec::new()
        };

        data_futures.into_iter().chain(teaser_futures.into_iter()).collect::<Vec<_>>()
    };

    join_all(image_futures).await.into_iter().for_each(|result| {
        if let Err(e) = result {
            eprintln!("Error occurred: {:?}", e);
        }
    });
}
