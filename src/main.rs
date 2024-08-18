use std::{error::Error, fs, path::Path};
use boosty::{download_boosty_blog, download_boosty_post};
use clap::Parser;
use cli::{Args, Commands};
use gelbooru::download_gelbooru;
use imgdl_rs::boosty::auth::Auth;

use futures::future::join_all;
use futures::Future;
use std::pin::Pin;

use colored::Colorize;

mod utils;
mod cli;
mod boosty;
mod gelbooru;

type BoxedFuture = Pin<Box<dyn Future<Output = Result<(), utils::Error>> + Send>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.cmd {
        Commands::Boosty { blog, post, path, access_token, limit, skip, photo_only, video_only } => {
            if let Some(post) = post {
                download_boosty_post(blog, post, path, access_token, photo_only, video_only).await;
            } else {
                download_boosty_blog(blog, path, access_token, limit, skip, photo_only, video_only).await;
            }
        },
        Commands::Gelbooru { path, tags, page, all, proxy } => {
            download_gelbooru(tags, page, path, all, proxy).await;
        }
    }

    Ok(())
}


