use crate::types::Rating;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct DocumentMeta {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub date: Option<String>,
    pub authorship: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub rating: Option<Rating>,
    pub vars: Option<HashMap<String, String>>,
}

impl DocumentMeta {
    pub fn parse_raw(header_str: &str) -> Self {
        let mut meta = DocumentMeta::default();
        let mut active_list_block: Option<&str> = None;

        for line in header_str.lines().map(str::trim) {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(item) = line.strip_prefix('-') {
                meta.push_list_item(active_list_block, item.trim());
                continue;
            }

            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim();
                let val = val.trim().trim_matches(['"', '\'']);

                if val.is_empty() {
                    active_list_block = Some(key);
                } else {
                    meta.assign_key_value(key, val, active_list_block);
                }
            }
        }

        meta
    }

    fn assign_key_value(&mut self, key: &str, val: &str, current_block: Option<&str>) {
        match key {
            "title" => self.title = Some(val.to_string()),
            "subtitle" => self.subtitle = Some(val.to_string()),
            "date" => self.date = Some(val.to_string()),
            "author" | "authorship" => self.authorship = Some(vec![val.to_string()]),
            "rating" => self.rating = Rating::parse(val),
            "tags" => {
                self.tags = Some(
                    val.trim_matches(['[', ']'])
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect(),
                );
            }
            _ if current_block == Some("vars") || current_block == Some("variables") => {
                self.vars
                    .get_or_insert_with(HashMap::new)
                    .insert(key.to_string(), val.to_string());
            }
            _ => {}
        }
    }

    fn push_list_item(&mut self, active_block: Option<&str>, item: &str) {
        match active_block {
            Some("tags") => {
                self.tags
                    .get_or_insert_with(Vec::new)
                    .push(item.to_string());
            }
            Some("author") | Some("authorship") => {
                self.authorship
                    .get_or_insert_with(Vec::new)
                    .push(item.to_string());
            }
            _ => {}
        }
    }
}
