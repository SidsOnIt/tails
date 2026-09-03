use indexmap::IndexMap;
use std::collections::HashMap;

#[non_exhaustive]
pub enum TextStyle {
    Regular,
    Bold,
    Italic,
    ULine,
    Strike,
    Link(String),
}

pub struct TextToken {
    pub style: TextStyle,
    pub value: String,
}

pub struct TextBody(pub Vec<TextToken>);

#[non_exhaustive]
pub enum AlertStyle {
    Info,
    Success,
    Warning,
    Failure,
}

pub struct AlertToken {
    pub style: AlertStyle,
    pub body: TextBody,
}

#[non_exhaustive]
pub enum ListStyle {
    Plain,
    Abc,
    Num,
    Bullet,
    Arrow,
    Task,
    Other,
}

pub struct List {
    pub name: String,
    pub style: ListStyle,
    pub leaves: Vec<String>,
    pub branches: Vec<List>,
}

#[non_exhaustive]
pub enum LeafStyle {
    File,
    Folder,
    Note,
}

pub struct Leaf {
    pub style: LeafStyle,
    pub value: String,
}

pub struct Tree {
    pub name: String,
    pub leaves: Vec<String>,
    pub branches: Vec<Tree>,
}

pub struct Table {
    pub name: String,
    pub columns: IndexMap<String, Vec<String>>,
}

#[non_exhaustive]
pub enum DocumentToken {
    Title(TextBody),
    SubTitle(TextBody),
    Section(TextBody),
    Text(TextBody),
    SubText(TextBody),
    Quote(TextBody),
    Alert(AlertToken),
    LinkBtn(String),
    Code(String),
    Table(Table),
    Tree(Tree),
    List(List),
    YouTubeEmbed(String),
    VideoEmbed(String),
    Image(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rating {
    UnderOr13,
    Rng14_15,
    Rng16_17,
    OverOr18,
}

impl Rating {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "UNDEROR13" | "13-" | "G" | "PG" | "PG-13" | "PG13" => Some(Rating::UnderOr13),
            "RNG14_15" | "14-15" | "14_15" | "M" | "MA15+" => Some(Rating::Rng14_15),
            "RNG16_17" | "16-17" | "16_17" | "R" | "16+" => Some(Rating::Rng16_17),
            "OVEROR18" | "18+" | "NC-17" | "NC17" | "ADULT" => Some(Rating::OverOr18),
            _ => None,
        }
    }
}

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
        let mut current_block: Option<&str> = None;

        for line in header_str.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Handle list items like "- tag1" or "- author1"
            if line.starts_with('-') {
                let item = line.trim_start_matches('-').trim().to_string();
                match current_block {
                    Some("tags") => {
                        meta.tags.get_or_insert_with(Vec::new).push(item);
                    }
                    Some("author") | Some("authorship") => {
                        meta.authorship.get_or_insert_with(Vec::new).push(item);
                    }
                    _ => {}
                }
                continue;
            }

            // Key-value line splitting on `:`
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim();
                let val = val.trim().trim_matches('"').trim_matches('\'');

                if val.is_empty() {
                    current_block = Some(key);
                    continue;
                }

                match key {
                    "title" => meta.title = Some(val.to_string()),
                    "subtitle" => meta.subtitle = Some(val.to_string()),
                    "date" => meta.date = Some(val.to_string()),
                    "author" | "authorship" => {
                        meta.authorship = Some(vec![val.to_string()]);
                    }
                    "tags" => {
                        let parsed_tags = val
                            .trim_matches('[')
                            .trim_matches(']')
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                        meta.tags = Some(parsed_tags);
                    }
                    "rating" => meta.rating = Rating::parse(val),
                    _ => {
                        if current_block == Some("vars") || current_block == Some("variables") {
                            meta.vars
                                .get_or_insert_with(HashMap::new)
                                .insert(key.to_string(), val.to_string());
                        }
                    }
                }
            }
        }

        meta
    }
}

pub struct Document {
    pub meta: DocumentMeta,
    pub raw: String,
}

impl Document {
    pub fn from_raw(raw: &str) -> Self {
        let trimmed = raw.trim_start();

        if trimmed.starts_with("---") {
            let rest = &trimmed[3..];
            if let Some(end_pos) = rest.find("\n---").or_else(|| rest.find("\r\n---")) {
                let header_str = &rest[..end_pos];

                let body_start = if rest[end_pos..].starts_with("\r\n---") {
                    end_pos + 6
                } else {
                    end_pos + 4
                };

                let body_str = rest[body_start..]
                    .trim_start_matches('\r')
                    .trim_start_matches('\n');

                let meta = DocumentMeta::parse_raw(header_str);
                let mut doc = Self {
                    meta,
                    raw: body_str.to_string(),
                };

                doc.interpolate_vars();
                return doc;
            }
        }

        let mut doc = Self {
            meta: DocumentMeta::default(),
            raw: raw.to_string(),
        };

        doc.interpolate_vars();
        doc
    }

    /// Replaces metadata placeholders (e.g. `{{title}}`, `{{author}}`, `{{var_name}}`)
    /// in `self.raw` with their parsed values.
    pub fn interpolate_vars(&mut self) {
        // 1. Substitute variables from the `vars` block
        if let Some(ref vars) = self.meta.vars {
            for (key, val) in vars {
                let pattern = ["{{", key.as_str(), "}}"].concat();
                self.raw = self.raw.replace(&pattern, val);
            }
        }

        // 2. Substitute top-level metadata fields
        if let Some(ref title) = self.meta.title {
            self.raw = self.raw.replace("{{title}}", title);
        }

        if let Some(ref subtitle) = self.meta.subtitle {
            self.raw = self.raw.replace("{{subtitle}}", subtitle);
        }

        if let Some(ref date) = self.meta.date {
            self.raw = self.raw.replace("{{date}}", date);
        }

        if let Some(ref authorship) = self.meta.authorship {
            if let Some(primary_author) = authorship.first() {
                self.raw = self.raw.replace("{{author}}", primary_author);
                self.raw = self.raw.replace("{{authorship}}", primary_author);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scratch_tails() {
        let raw_file = include_str!("scratch.tails");

        let doc = Document::from_raw(raw_file);

        println!("\n==================== META ====================");
        println!("{:#?}", doc.meta);

        println!("\n=================== CONTENT ==================");
        println!("{}", doc.raw);
        println!("==============================================\n");
    }
}
