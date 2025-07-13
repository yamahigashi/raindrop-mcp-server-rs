use anyhow::Result;
use raindrop_mcp_server_rs::mcp::McpServer;
use rmcp::{ServiceExt, transport::stdio};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize environment variables
    dotenv::dotenv().ok();

    // Initialize tracing to stderr
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting Raindrop MCP server...");

    // Create MCP server and serve with STDIO transport
    let service = McpServer::new()?
        .serve(stdio())
        .await
        .inspect_err(|e| error!("Service error: {:?}", e))?;

    info!("MCP server initialized, waiting for connections...");

    // Wait for the service to complete
    service.waiting().await?;

    info!("MCP server shutdown complete");
    Ok(())
}
