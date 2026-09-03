use crate::document::meta::DocumentMeta;

/// Splits front-matter metadata (`---` block) from the document body string.
pub fn extract(raw: &str) -> (DocumentMeta, String) {
    let trimmed = raw.trim_start();

    if let Some(rest) = trimmed.strip_prefix("---") {
        if let Some((header, body)) = rest.split_once("\n---") {
            let body_str = body.strip_prefix('\r').unwrap_or(body);
            let body_str = body_str.strip_prefix('\n').unwrap_or(body_str);
            return (DocumentMeta::parse_raw(header), body_str.to_string());
        }
    }

    (DocumentMeta::default(), raw.to_string())
}
