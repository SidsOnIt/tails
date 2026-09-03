pub mod document;
pub mod types;

// Re-export core types for clean library ergonomics
pub use document::Document;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_index_tails_pipeline() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");

        // Point base_dir directly to src/test_docs where index, overview, and footer live!
        let docs_dir = Path::new(manifest_dir).join("src").join("test_docs");

        let raw_file = std::fs::read_to_string(docs_dir.join("index.tails"))
            .expect("Failed to read index.tails from src/test_docs/");

        let doc = Document::from_raw_recursive(&raw_file, Some(&docs_dir));

        println!("\n==================== META ====================");
        println!("{:#?}", doc.meta);

        println!("\n=================== CONTENT ==================");
        println!("{}", doc.raw);
        println!("==============================================\n");
    }
}
