pub mod document;
pub mod meta;
pub mod types;

// Re-export core types for clean library ergonomics
pub use document::Document;
pub use meta::DocumentMeta;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_index_tails_pipeline() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let src_dir = Path::new(manifest_dir).join("src");

        let raw_file = include_str!("index.tails");
        let doc = Document::from_raw_recursive(raw_file, Some(&src_dir));

        println!("\n==================== META ====================");
        println!("{:#?}", doc.meta);

        println!("\n=================== CONTENT ==================");
        println!("{}", doc.raw);
        println!("==============================================\n");
    }
}
