use actix_web::HttpRequest;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_time() -> u128 {
    let start = SystemTime::now();

    start.duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn get_cookie(req: HttpRequest, name: &str) -> String {
    let cookie: Vec<&str> = req
        .headers()
        .get("cookie")
        .unwrap()
        .to_str()
        .unwrap()
        .split("&")
        .collect();

    let auth_token: Vec<&str> = cookie
        .into_iter()
        .filter(|each| {
            let body: Vec<&str> = each.split("=").collect();

            body[0] == name
        })
        .collect();

    let cookie_part: Vec<&str> = auth_token[0].split("=").collect();

    cookie_part[1].to_owned()
}
