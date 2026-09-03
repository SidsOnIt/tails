use std::path::{Path, PathBuf};

/// Extracts the raw relative path from an `inline_page:` or `include:` line.
pub fn extract_path(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.starts_with("inline_page:") || trimmed.starts_with("include:") {
        let (_, path_part) = trimmed.split_once(':')?;
        Some(path_part.trim().trim_end_matches(";;").trim())
    } else {
        None
    }
}

/// Appends `.tails` if no extension is present.
pub fn resolve_path(base_dir: &Path, rel_path: &str) -> PathBuf {
    if rel_path.ends_with(".tails") {
        base_dir.join(rel_path)
    } else {
        base_dir.join(format!("{}.tails", rel_path))
    }
}
