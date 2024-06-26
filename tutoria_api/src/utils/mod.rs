use std::path::Path;

use lopdf::Document;

pub use self::error::{ Error, Result };

pub mod error;
pub mod time;

pub fn convert_file_for_string(buffer: &[u8]) -> Result<Vec<String>> {
    let doc = Document::load_mem(buffer).map_err(|err| Error::PDFError(err.to_string()))?;
    let pages = doc.get_pages();
    let mut texts = Vec::new();

    for (i, _) in pages.iter().enumerate() {
        let page_number = (i + 1) as u32;
        let text = doc.extract_text(&[page_number]);
        texts.push(text.unwrap_or_default());
    }

    Ok(texts)
}