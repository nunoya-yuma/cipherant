use std::path::Path;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// Represents extracted content from a PDF file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfContent {
    /// The file path of the PDF
    pub path: String,
    /// The title of the PDF (from metadata, if available)
    pub title: Option<String>,
    /// The extracted text content
    pub text: String,
}

/// Read a local PDF file and extract its text content.
///
/// # Errors
/// Returns an error if the file does not exist or cannot be parsed as PDF.
pub(crate) fn read_pdf(path: &str) -> Result<PdfContent> {
    if !Path::new(path).exists() {
        bail!("{} is not found. Check whether file exists.", path)
    }

    let pdf_text = pdf_extract::extract_text(path)?;
    Ok(PdfContent {
        path: path.to_string(),
        title: None,
        text: pdf_text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_pdf_file_not_found() {
        let result = read_pdf("/nonexistent/path/to/file.pdf");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
