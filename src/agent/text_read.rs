use log::info;
use rig::completion::ToolDefinition;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::collectors::documents::read_text_file;

/// Arguments for the TextRead tool
#[derive(Deserialize)]
pub struct TextReadArgs {
    file_path: String,
}

/// Output from the TextRead tool
#[derive(Serialize)]
pub struct TextReadOutput {
    path: String,
    content: String,
}

/// Error type for TextRead tool
#[derive(Debug, thiserror::Error)]
pub enum TextReadError {
    #[error("Failed to read text file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Blocking task failed: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

/// TextRead tool for reading local text/Markdown files
pub struct TextRead;

impl rig::tool::Tool for TextRead {
    const NAME: &'static str = "text_read";
    type Error = TextReadError;
    type Args = TextReadArgs;
    type Output = TextReadOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name(),
            description: "Reads a local text or Markdown file".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "The local file path to the text/Markdown file"
                    }
                },
                "required": ["file_path"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        info!("Reading text file {} ...", args.file_path);

        let path = args.file_path.clone();
        let content = tokio::task::spawn_blocking(move || read_text_file(&path)).await??;
        Ok(TextReadOutput {
            path: args.file_path,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use ::rig::tool::Tool;

    use super::*;

    #[test]
    fn test_text_read_args_deserialization() {
        let json_data = r#"{"file_path": "/example.txt"}"#;
        let args: TextReadArgs = serde_json::from_str(json_data).unwrap();
        assert_eq!(args.file_path, "/example.txt");
    }

    #[tokio::test]
    async fn test_text_read_call_file_not_found() {
        let args = TextReadArgs {
            file_path: "/nonexistent/path/to/file.txt".to_string(),
        };
        let sut = TextRead;
        let result = sut.call(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_text_read_call_success() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), "Test content of the file.").unwrap();

        let args = TextReadArgs {
            file_path: temp_file.path().to_str().unwrap().to_string(),
        };
        let sut = TextRead;
        let result = sut.call(args).await.unwrap();

        assert_eq!(result.content, "Test content of the file.");
    }
}
