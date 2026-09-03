use crate::meta::DocumentMeta;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct Document {
    pub meta: DocumentMeta,
    pub raw: String,
}

impl Document {
    pub fn from_raw(raw: &str) -> Self {
        Self::from_raw_recursive(raw, None)
    }

    pub fn from_raw_recursive(raw: &str, base_dir: Option<&Path>) -> Self {
        let mut visited = HashSet::new();
        Self::parse_inner(raw, base_dir, &mut visited, None)
    }

    fn parse_inner(
        raw: &str,
        base_dir: Option<&Path>,
        visited: &mut HashSet<PathBuf>,
        parent_vars: Option<&HashMap<String, String>>,
    ) -> Self {
        let (meta, body_raw) = extract_front_matter(raw);
        let mut doc = Self {
            meta,
            raw: body_raw,
        };

        doc.merge_parent_vars(parent_vars);
        doc.interpolate_vars();

        if let Some(dir) = base_dir {
            doc.resolve_inclusions(dir, visited);
        }

        doc
    }

    fn merge_parent_vars(&mut self, parent_vars: Option<&HashMap<String, String>>) {
        if let Some(p_vars) = parent_vars {
            let local_vars = self.meta.vars.get_or_insert_with(HashMap::new);
            for (k, v) in p_vars {
                local_vars.entry(k.clone()).or_insert_with(|| v.clone());
            }
        }
    }

    pub fn interpolate_vars(&mut self) {
        if let Some(ref title) = self.meta.title {
            self.raw = self.raw.replace("{{title}}", title);
        }
        if let Some(ref subtitle) = self.meta.subtitle {
            self.raw = self.raw.replace("{{subtitle}}", subtitle);
        }
        if let Some(ref date) = self.meta.date {
            self.raw = self.raw.replace("{{date}}", date);
        }
        if let Some(author) = self.meta.authorship.as_ref().and_then(|a| a.first()) {
            self.raw = self.raw.replace("{{author}}", author);
            self.raw = self.raw.replace("{{authorship}}", author);
        }

        if let Some(ref vars) = self.meta.vars {
            for (key, val) in vars {
                let pattern = format!("{{{{{}}}}}", key);
                self.raw = self.raw.replace(&pattern, val);
            }
        }
    }

    fn resolve_inclusions(&mut self, base_dir: &Path, visited: &mut HashSet<PathBuf>) {
        let mut output_lines = Vec::new();

        for line in self.raw.lines() {
            if let Some(rel_path) = extract_inclusion_path(line) {
                let target_path = resolve_file_extension(base_dir, rel_path);

                if visited.contains(&target_path) {
                    output_lines.push(format!("<!-- Circular inclusion skipped: {} -->", rel_path));
                    continue;
                }

                if let Ok(child_raw) = std::fs::read_to_string(&target_path) {
                    visited.insert(target_path.clone());
                    let child_dir = target_path.parent().unwrap_or(base_dir);

                    let child_doc = Self::parse_inner(
                        &child_raw,
                        Some(child_dir),
                        visited,
                        self.meta.vars.as_ref(),
                    );

                    output_lines.push(child_doc.raw);
                    continue;
                }
            }

            output_lines.push(line.to_string());
        }

        self.raw = output_lines.join("\n");
    }
}

fn extract_front_matter(raw: &str) -> (DocumentMeta, String) {
    let trimmed = raw.trim_start();

    if trimmed.starts_with("---") {
        let rest = &trimmed[3..];
        if let Some(end_pos) = rest.find("\n---").or_else(|| rest.find("\r\n---")) {
            let header_str = &rest[..end_pos];
            let body_offset = if rest[end_pos..].starts_with("\r\n---") {
                6
            } else {
                4
            };
            let body_str = rest[end_pos + body_offset..].trim_start_matches(['\r', '\n']);

            return (DocumentMeta::parse_raw(header_str), body_str.to_string());
        }
    }

    (DocumentMeta::default(), raw.to_string())
}

fn extract_inclusion_path(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.starts_with("inline_page:") || trimmed.starts_with("include:") {
        let (_, path_part) = trimmed.split_once(':')?;
        Some(path_part.trim().trim_end_matches(";;").trim())
    } else {
        None
    }
}

fn resolve_file_extension(base_dir: &Path, rel_path: &str) -> PathBuf {
    if rel_path.ends_with(".tails") {
        base_dir.join(rel_path)
    } else {
        base_dir.join(format!("{}.tails", rel_path))
    }
}
