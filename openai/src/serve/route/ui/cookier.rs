use super::DEFAULT_INDEX;
use crate::serve::EMPTY;
use axum_extra::extract::cookie;

pub fn build_cookie<'a>(
    key: &'a str,
    value: String,
    timestamp: i64,
) -> anyhow::Result<cookie::Cookie<'a>> {
    let cookie = cookie::Cookie::build(key, value.to_owned())
        .path(DEFAULT_INDEX)
        .same_site(cookie::SameSite::Lax)
        .expires(time::OffsetDateTime::from_unix_timestamp(timestamp)?)
        .secure(false)
        .http_only(false)
        .finish();
    Ok(cookie)
}

pub fn clear_cookie<'a>(key: &'a str) -> cookie::Cookie<'a> {
    let cookie = cookie::Cookie::build(key, EMPTY)
        .path(DEFAULT_INDEX)
        .same_site(cookie::SameSite::Lax)
        .max_age(time::Duration::seconds(0))
        .secure(false)
        .http_only(false)
        .finish();
    cookie
}
