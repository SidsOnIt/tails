mod front_matter;
mod inclusion;
mod interpolation;
pub mod meta;

pub use meta::DocumentMeta;

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
        let (mut meta, body_raw) = front_matter::extract(raw);

        interpolation::merge_scopes(&mut meta, parent_vars);
        let interpolated_body = interpolation::replace_vars(&body_raw, &meta);

        let mut doc = Self {
            meta,
            raw: interpolated_body,
        };

        if let Some(dir) = base_dir {
            doc.resolve_inclusions(dir, visited);
        }

        doc
    }

    fn resolve_inclusions(&mut self, current_dir: &Path, visited: &mut HashSet<PathBuf>) {
        let mut output = String::with_capacity(self.raw.len());

        for line in self.raw.lines() {
            if let Some(rel_path) = inclusion::extract_path(line) {
                let target_path = inclusion::resolve_path(current_dir, rel_path);

                // Insert returns false if the item was already in the set!
                if !visited.insert(target_path.clone()) {
                    output.push_str(&format!(
                        "<!-- Circular inclusion skipped: {} -->\n",
                        rel_path
                    ));
                    continue;
                }

                if let Ok(child_raw) = std::fs::read_to_string(&target_path) {
                    let child_dir = target_path.parent().unwrap_or(current_dir);

                    let child_doc = Self::parse_inner(
                        &child_raw,
                        Some(child_dir),
                        visited,
                        self.meta.vars.as_ref(),
                    );

                    output.push_str(&child_doc.raw);
                    output.push('\n');
                    continue;
                }
            }

            output.push_str(line);
            output.push('\n');
        }

        self.raw = output;
    }
}
