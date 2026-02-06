use log::info;
use rig::completion::ToolDefinition;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::collectors::pdf::read_pdf;

/// Arguments for the PdfRead tool
#[derive(Deserialize)]
pub struct PdfReadArgs {
    file_path: String,
}

/// Output from the PdfRead tool
#[derive(Serialize)]
pub struct PdfReadOutput {
    path: String,
    title: Option<String>,
    content: String,
}

/// Error type for PdfRead tool
#[derive(Debug, thiserror::Error)]
pub enum PdfReadError {
    #[error("Failed to read PDF: {0}")]
    ReadError(#[from] anyhow::Error),
    #[error("Blocking task failed: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

/// PdfRead tool for extracting text from local PDF files
pub struct PdfRead;

impl rig::tool::Tool for PdfRead {
    const NAME: &'static str = "pdf_read";
    type Error = PdfReadError;
    type Args = PdfReadArgs;
    type Output = PdfReadOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name(),
            description: "Reads a local PDF file and extracts its text content".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The local file path to the PDF"
                    }
                },
                "required": ["file_path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        info!("Reading PDF {} ...", args.file_path);

        let pdf_content = tokio::task::spawn_blocking(move || read_pdf(&args.file_path)).await??;
        Ok(PdfReadOutput {
            path: pdf_content.path,
            title: pdf_content.title,
            content: pdf_content.text,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::rig::tool::Tool;

    #[test]
    fn test_pdf_read_args_deserialize() {
        let json = r#"{"file_path": "/tmp/test.pdf"}"#;
        let args: PdfReadArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.file_path, "/tmp/test.pdf");
    }

    #[test]
    fn test_pdf_read_output_serialize() {
        let output = PdfReadOutput {
            path: "/tmp/test.pdf".to_string(),
            title: Some("Test PDF".to_string()),
            content: "Hello PDF".to_string(),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("Test PDF"));
    }

    #[tokio::test]
    async fn test_pdf_read_call_file_not_found() {
        let args = PdfReadArgs {
            file_path: "/nonexistent/path/to/file.pdf".to_string(),
        };
        let sut = PdfRead;
        let result = sut.call(args).await;
        assert!(result.is_err());
    }
}
