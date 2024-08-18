boostydownloader
================
A simple application to bulk-download content from Boosty and Gelbooru.
Made with my own library imgdl-rs (https://github.com/crptmem/imgdl-rs)

Installation
=====
  $ cargo install --git https://github.com/leofaraf/boostydownloader-ultra-hd.git

Usage
=====
 Boosty:
  $ boostydownload boosty --blog <BLOG_NAME>
  Without authorization:
    $ boostydownload boosty --blog USERNAME
  With authorization:
    $ boostydownload boosty --blog USERNAME --access-token ACCESS_TOKEN
  If requested blog have more than 300 posts:
    $ boostydownload boosty --blog USERNAME --limit POSTS_COUNT
  If required to skip certain count of posts:
    $ boostydownload boosty --blog USERNAME --skip SKIP_COUNT
  If required to download single post:
    $ boostydownload boosty --blog USERNAME --post POST_ID --access-token ACCESS_TOKEN
 Gelbooru:
  $ boostydownload gelbooru --url <SITE_ROOT_URL> --tags <TAGS> --proxy [PROXY]
By default content is downloaded to `$PWD/img`. You can change path by `--path` argument.
Obtaining `POST_ID` and `USERNAME`. Go to a post in browser, copy values from URL by this pattern:
https://boosty.to/{USERNAME}/posts/{POST_ID}

Obtaining access token
======================
Go to https://boosty.to, open developer tools in your browser,
go to Storage (Application) -> Cookies. You need `auth`, click on it and in
right panel click RMB on `accessToken` and copy it.

