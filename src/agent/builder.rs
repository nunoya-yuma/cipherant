use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig::providers::{gemini, ollama, openai};

use super::mcp::McpToolSet;
use super::research_tool::ResearchTool;
use super::{PdfRead, TextRead, WebFetch, WebSearch};

const PREAMBLE: &str =
    "You are a research assistant that helps users gather and summarize information from the web";

/// Create an Ollama-based research agent
pub fn create_ollama_agent(
    model: &str,
    web_fetch: WebFetch,
    mcp_tools: Vec<McpToolSet>,
) -> Agent<ollama::CompletionModel> {
    let client = ollama::Client::builder()
        .api_key(Nothing)
        .build()
        .expect("Failed to create Ollama client");

    let mut builder = client
        .agent(model)
        .preamble(PREAMBLE)
        .default_max_turns(10)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead);

    for mcp in mcp_tools {
        builder = builder.rmcp_tools(mcp.tools, mcp.sink);
    }

    builder.build()
}

/// Create a Gemini-based research agent
pub fn create_gemini_agent(
    api_key: &str,
    model: &str,
    web_fetch: WebFetch,
    mcp_tools: Vec<McpToolSet>,
) -> Agent<gemini::completion::CompletionModel> {
    let client = gemini::Client::new(api_key).expect("Failed to create Gemini client");

    let mut builder = client
        .agent(model)
        .preamble(PREAMBLE)
        .default_max_turns(10)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead);

    for mcp in mcp_tools {
        builder = builder.rmcp_tools(mcp.tools, mcp.sink);
    }

    builder.build()
}

/// Create an OpenAI-based research agent
pub fn create_openai_agent(
    api_key: &str,
    model: &str,
    web_fetch: WebFetch,
    mcp_tools: Vec<McpToolSet>,
) -> Agent<ResponsesCompletionModel> {
    let client: rig::client::Client<openai::OpenAIResponsesExt> =
        openai::Client::new(api_key).expect("Failed to create OpenAI client");

    let mut builder = client
        .agent(model)
        .preamble(PREAMBLE)
        .default_max_turns(10)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead);

    for mcp in mcp_tools {
        builder = builder.rmcp_tools(mcp.tools, mcp.sink);
    }

    builder.build()
}

/// System prompt that defines the RouterAgent's tool-selection strategy.
///
/// The router sees all tools (research_tool, web_search, web_fetch, pdf_read, text_read)
/// and must choose the right one based on the user's intent:
/// - Deep investigation      → research_tool
/// - Quick lookup            → web_search
/// - Specific URL            → web_fetch
/// - PDF document            → pdf_read
/// - Text/Markdown document  → text_read
/// - General chat            → no tool
const ROUTER_PREAMBLE: &str = "\
You are an intelligent assistant that routes user requests to the most appropriate tool.\n\
\n\
Available tools and when to use them:\n\
- research_tool: Use for in-depth research requiring multiple sources. \
  Triggers a full investigation across web pages and returns a structured report. \
  Use when the user wants thorough analysis, comparisons, or comprehensive understanding.\n\
- web_search: Use for quick factual lookups, current events, or brief information needs \
  that don't require reading full pages.\n\
- web_fetch: Use when the user provides a specific URL to read or when you need \
  to retrieve a known page.\n\
- pdf_read: Use when the user provides a path to a PDF file to read.\n\
- text_read: Use to read a local plain-text or Markdown file (found via the list of local \
  documents below, or given directly). Use pdf_read instead if the file is a PDF.\n\
- Additional MCP tools may be available depending on configuration. \
  Use them when they match the user's request more precisely than the built-in tools above.\n\
\n\
For general conversation, questions you can answer from your knowledge, or simple \
clarifications — respond directly without using any tool.";

/// Build the full router preamble by embedding the list of locally available
/// document file paths (from `collectors::documents::list_documents`) into
/// `ROUTER_PREAMBLE`, so the LLM always knows what's available without
/// needing to call a tool to discover it.
fn build_router_preamble(documents: &[String]) -> String {
    let mut preamble = ROUTER_PREAMBLE.to_string();

    if documents.is_empty() {
        return preamble + "No local documents available\n";
    }

    preamble += "The following local documents are available for reading:\n";
    preamble += &documents
        .iter()
        .map(|file| format!("- {}\n", file))
        .collect::<String>();

    preamble
}

/// Create an Ollama-based router agent with all routing tools
pub fn create_ollama_router_agent(
    model: &str,
    research_tool: ResearchTool,
    web_fetch: WebFetch,
    mcp_tools: Vec<McpToolSet>,
    documents: &[String],
) -> Agent<ollama::CompletionModel> {
    let client = ollama::Client::builder()
        .api_key(Nothing)
        .build()
        .expect("Failed to create Ollama client");

    let mut builder = client
        .agent(model)
        .preamble(&build_router_preamble(documents))
        .default_max_turns(10)
        .tool(research_tool)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .tool(TextRead);

    for mcp in mcp_tools {
        builder = builder.rmcp_tools(mcp.tools, mcp.sink);
    }

    builder.build()
}

/// Create a Gemini-based router agent with all routing tools
pub fn create_gemini_router_agent(
    api_key: &str,
    model: &str,
    research_tool: ResearchTool,
    web_fetch: WebFetch,
    mcp_tools: Vec<McpToolSet>,
    documents: &[String],
) -> Agent<gemini::completion::CompletionModel> {
    let client = gemini::Client::new(api_key).expect("Failed to create Gemini client");

    let mut builder = client
        .agent(model)
        .preamble(&build_router_preamble(documents))
        .default_max_turns(10)
        .tool(research_tool)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .tool(TextRead);

    for mcp in mcp_tools {
        builder = builder.rmcp_tools(mcp.tools, mcp.sink);
    }

    builder.build()
}

/// Create an OpenAI-based router agent with all routing tools
pub fn create_openai_router_agent(
    api_key: &str,
    model: &str,
    research_tool: ResearchTool,
    web_fetch: WebFetch,
    mcp_tools: Vec<McpToolSet>,
    documents: &[String],
) -> Agent<ResponsesCompletionModel> {
    let client: rig::client::Client<openai::OpenAIResponsesExt> =
        openai::Client::new(api_key).expect("Failed to create OpenAI client");

    let mut builder = client
        .agent(model)
        .preamble(&build_router_preamble(documents))
        .default_max_turns(10)
        .tool(research_tool)
        .tool(web_fetch)
        .tool(WebSearch)
        .tool(PdfRead)
        .tool(TextRead);

    for mcp in mcp_tools {
        builder = builder.rmcp_tools(mcp.tools, mcp.sink);
    }

    builder.build()
}

/// Get the default model name for a given provider
pub fn default_model(provider: &str) -> &'static str {
    match provider {
        "gemini" => gemini::completion::GEMINI_2_5_FLASH,
        "openai" => openai::completion::GPT_4_1_MINI,
        _ => "qwen3",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use rig::completion::Prompt;
    use rig::providers::{gemini, openai};

    #[test]
    fn test_default_model_ollama() {
        assert_eq!(default_model("ollama"), "qwen3");
    }

    #[test]
    fn test_default_model_gemini() {
        assert_eq!(default_model("gemini"), "gemini-2.5-flash");
    }

    #[test]
    fn test_default_model_openai() {
        assert_eq!(default_model("openai"), "gpt-4.1-mini");
    }

    #[test]
    fn test_default_model_unknown_fallback() {
        assert_eq!(default_model("unknown"), "qwen3");
    }

    #[test]
    fn test_build_router_preamble_includes_document_list() {
        let documents = vec![
            "/path/to/doc1.txt".to_string(),
            "/path/to/doc2.txt".to_string(),
        ];
        let preamble = build_router_preamble(&documents);
        assert!(preamble.contains("/path/to/doc1.txt"));
        assert!(preamble.contains("/path/to/doc2.txt"));
    }

    #[test]
    fn test_build_router_preamble_empty_documents() {
        let documents = vec![];
        let preamble = build_router_preamble(&documents);

        assert!(preamble.contains(ROUTER_PREAMBLE));
    }

    #[tokio::test]
    #[ignore]
    async fn test_ollama_agent_with_web_fetch() {
        let agent = create_ollama_agent("qwen3", WebFetch::new(), vec![]);
        let response = agent
            .prompt("Fetch https://example.com and **summarize** it shortly")
            .await
            .unwrap();

        println!("{}", response);
        assert!(!response.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_gemini_agent_with_web_fetch() {
        dotenv().ok();

        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY required");
        let agent = create_gemini_agent(
            &api_key,
            gemini::completion::GEMINI_2_5_FLASH,
            WebFetch::new(),
            vec![],
        );
        let response = agent
            .prompt("Fetch https://example.com and **summarize** it shortly")
            .await
            .unwrap();

        println!("{}", response);
        assert!(!response.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_openai_agent_with_web_fetch() {
        dotenv().ok();

        let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY required");
        let agent = create_openai_agent(
            &api_key,
            openai::completion::GPT_4_1_MINI,
            WebFetch::new(),
            vec![],
        );
        let response = agent
            .prompt("Fetch https://example.com and **summarize** it shortly")
            .await
            .unwrap();

        println!("{}", response);
        assert!(!response.is_empty());
    }
}
