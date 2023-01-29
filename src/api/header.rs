use http::HeaderMap;

pub const DISABLE_NOTIFICATION_HEADER: &str = "X-TD-DISABLE-NOTIFICATIONS";

pub fn disable_notifications_header(headers: &mut HeaderMap, disable: bool) {
    headers.insert(
        DISABLE_NOTIFICATION_HEADER,
        disable.to_string().parse().unwrap(),
    );
}
