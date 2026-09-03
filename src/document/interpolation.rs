use crate::document::meta::DocumentMeta;
use std::collections::HashMap;

/// Inherits parent variables into the local scope without overwriting local overrides.
pub fn merge_scopes(local_meta: &mut DocumentMeta, parent_vars: Option<&HashMap<String, String>>) {
    if let Some(p_vars) = parent_vars {
        let local_vars = local_meta.vars.get_or_insert_with(HashMap::new);
        for (k, v) in p_vars {
            local_vars.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
}

/// Replaces metadata and custom `vars:` macros within the raw body text.
pub fn replace_vars(raw: &str, meta: &DocumentMeta) -> String {
    let mut updated = raw.to_string();

    if let Some(ref title) = meta.title {
        updated = updated.replace("{{title}}", title);
    }
    if let Some(ref subtitle) = meta.subtitle {
        updated = updated.replace("{{subtitle}}", subtitle);
    }
    if let Some(ref date) = meta.date {
        updated = updated.replace("{{date}}", date);
    }
    if let Some(author) = meta.authorship.as_ref().and_then(|a| a.first()) {
        updated = updated.replace("{{author}}", author);
        updated = updated.replace("{{authorship}}", author);
    }

    if let Some(ref vars) = meta.vars {
        for (key, val) in vars {
            let pattern = format!("{{{{{}}}}}", key);
            updated = updated.replace(&pattern, val);
        }
    }

    updated
}
