use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, web, Error, HttpRequest};

/// Handlers for favicons. Some browsers and operating systems automatically look for
/// them at a specific location.

#[get("/{filename:android-chrome-[0-9x]{7}.png}")]
async fn android_chrome_png(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    try_open_file(req)
}

#[get("/{filename:apple-touch-icon.png}")]
async fn apple_touch_icon(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    try_open_file(req)
}

#[get("/{filename:browserconfig.xml}")]
async fn browserconfig_xml(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    try_open_file(req)
}

#[get("/{filename:favicon.ico}")]
async fn favicon_ico(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    try_open_file(req)
}
#[get("/{filename:favicon-[1-6x]{5}.png}")]
async fn favicon_x_ico(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    try_open_file(req)
}

#[get("/{filename:mstile-[0-7x]+.png}")]
async fn mstile_png(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    try_open_file(req)
}

#[get("/{filename:safari-pinned-tab.svg}")]
async fn safari_svg(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    try_open_file(req)
}

#[get("/{filename:site.webmanifest}")]
async fn site_webmanifest(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    try_open_file(req)
}

fn try_open_file(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: String = req.match_info().query("filename").parse().unwrap();
    let path = format!("static/favicon/{}", path);
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(android_chrome_png)
        .service(apple_touch_icon)
        .service(browserconfig_xml)
        .service(favicon_ico)
        .service(favicon_x_ico)
        .service(mstile_png)
        .service(safari_svg)
        .service(site_webmanifest);
}
