use rmcp::{
    RoleServer,
    handler::server::{
        ServerHandler,
        tool::{Parameters, ToolRouter},
    },
    model::ErrorData as McpError,
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::sync::Arc;
use tracing::{debug, info};

use crate::{
    error::RaindropMcpError,
    raindrop::{client::RaindropClient, types::SearchParams},
};

#[derive(Clone)]
pub struct McpServer {
    client: Arc<RaindropClient>,
    tool_router: ToolRouter<Self>,
}

// Parameter structures for tools
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct EmptyParams {}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct CreateCollectionParams {
    title: String,
    #[serde(default)]
    public: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct UpdateCollectionParams {
    id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct DeleteCollectionParams {
    id: i64,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ShareCollectionParams {
    id: i64,
    level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    emails: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GetBookmarksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "perpage")]
    per_page: Option<i32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GetBookmarkParams {
    id: i64,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct CreateBookmarkParams {
    link: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excerpt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    note: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct UpdateBookmarkParams {
    id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excerpt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    important: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct DeleteBookmarkParams {
    id: i64,
}

// Tag management parameters
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GetTagsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<i64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct RenameTagParams {
    old_name: String,
    new_name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct DeleteTagParams {
    name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct DeleteTagsParams {
    names: Vec<String>,
}

// Search parameters
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct SearchBookmarksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    important: Option<bool>,
}

// Batch operation parameters
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct BatchUpdateBookmarksParams {
    ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    important: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct BatchDeleteBookmarksParams {
    ids: Vec<i64>,
}

// Highlight parameters
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GetHighlightsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    bookmark_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    collection_id: Option<i64>,
}

// Export parameters
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ExportBookmarksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}

// Tool implementations using macros
#[tool_router]
impl McpServer {
    // Collection tools
    #[tool(description = "Retrieves all collections from Raindrop.io")]
    async fn get_collections(&self, _params: Parameters<EmptyParams>) -> String {
        debug!("Getting all collections");
        match self.client.get_collections().await {
            Ok(collections) => serde_json::to_string(&collections)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Gets a specific collection by ID")]
    async fn get_collection(
        &self,
        Parameters(params): Parameters<DeleteCollectionParams>,
    ) -> String {
        debug!("Getting collection: {}", params.id);
        match self.client.get_collection(params.id).await {
            Ok(collection) => serde_json::to_string(&collection)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Creates a new collection")]
    async fn create_collection(
        &self,
        Parameters(params): Parameters<CreateCollectionParams>,
    ) -> String {
        info!("Creating collection: {}", params.title);
        match self
            .client
            .create_collection(params.title, params.public)
            .await
        {
            Ok(collection) => serde_json::to_string(&collection)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Updates an existing collection")]
    async fn update_collection(
        &self,
        Parameters(params): Parameters<UpdateCollectionParams>,
    ) -> String {
        info!("Updating collection: {}", params.id);
        let mut updates = serde_json::Map::new();

        if let Some(title) = params.title {
            updates.insert("title".to_string(), serde_json::Value::String(title));
        }
        if let Some(public) = params.public {
            updates.insert("public".to_string(), serde_json::Value::Bool(public));
        }
        if let Some(view) = params.view {
            updates.insert("view".to_string(), serde_json::Value::String(view));
        }
        if let Some(sort) = params.sort {
            updates.insert("sort".to_string(), serde_json::Value::String(sort));
        }

        match self
            .client
            .update_collection(params.id, serde_json::Value::Object(updates))
            .await
        {
            Ok(collection) => serde_json::to_string(&collection)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Deletes a collection")]
    async fn delete_collection(
        &self,
        Parameters(params): Parameters<DeleteCollectionParams>,
    ) -> String {
        info!("Deleting collection: {}", params.id);
        match self.client.delete_collection(params.id).await {
            Ok(_) => serde_json::json!({"success": true}).to_string(),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Shares a collection with others")]
    async fn share_collection(
        &self,
        Parameters(params): Parameters<ShareCollectionParams>,
    ) -> String {
        info!(
            "Sharing collection {} with level: {}",
            params.id, params.level
        );
        match self
            .client
            .share_collection(params.id, &params.level, params.emails)
            .await
        {
            Ok(result) => serde_json::to_string(&result)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    // Bookmark tools
    #[tool(description = "Retrieves bookmarks with optional filtering")]
    async fn get_bookmarks(&self, Parameters(params): Parameters<GetBookmarksParams>) -> String {
        debug!("Getting bookmarks with filters");
        let search_params = SearchParams {
            collection: params.collection,
            search: params.search,
            page: params.page,
            per_page: params.per_page,
            ..Default::default()
        };
        match self.client.get_bookmarks(search_params).await {
            Ok(bookmarks) => serde_json::to_string(&bookmarks)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Gets a specific bookmark by ID")]
    async fn get_bookmark(&self, Parameters(params): Parameters<GetBookmarkParams>) -> String {
        debug!("Getting bookmark: {}", params.id);
        match self.client.get_bookmark(params.id).await {
            Ok(bookmark) => serde_json::to_string(&bookmark)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Creates a new bookmark")]
    async fn create_bookmark(
        &self,
        Parameters(params): Parameters<CreateBookmarkParams>,
    ) -> String {
        info!("Creating bookmark: {}", params.link);
        match self
            .client
            .create_bookmark(
                params.link,
                params.collection.unwrap_or(0), // Default to "All" collection
                Some(params.title),
                params.excerpt,
                params.tags,
                None, // important field not included in params
            )
            .await
        {
            Ok(bookmark) => serde_json::to_string(&bookmark)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Updates an existing bookmark")]
    async fn update_bookmark(
        &self,
        Parameters(params): Parameters<UpdateBookmarkParams>,
    ) -> String {
        info!("Updating bookmark: {}", params.id);
        let id = params.id;
        let mut updates = match serde_json::to_value(params) {
            Ok(val) => val,
            Err(e) => return format!(r#"{{"error": "Serialization error: {e}"}}"#),
        };
        if let Some(obj) = updates.as_object_mut() {
            obj.remove("id");
        }

        match self.client.update_bookmark(id, updates).await {
            Ok(bookmark) => serde_json::to_string(&bookmark)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Deletes a bookmark")]
    async fn delete_bookmark(
        &self,
        Parameters(params): Parameters<DeleteBookmarkParams>,
    ) -> String {
        info!("Deleting bookmark: {}", params.id);
        match self.client.delete_bookmark(params.id, false).await {
            // false = move to trash, not permanent
            Ok(_) => serde_json::json!({"success": true}).to_string(),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    // Search and batch operations
    #[tool(description = "Search bookmarks with advanced filtering")]
    async fn search_bookmarks(
        &self,
        Parameters(params): Parameters<SearchBookmarksParams>,
    ) -> String {
        debug!("Searching bookmarks with advanced filters");
        let search_params = SearchParams {
            collection: params.collection,
            search: params.search,
            tags: params.tags,
            page: params.page,
            per_page: params.per_page,
            sort: params.sort,
            important: params.important,
            ..Default::default()
        };
        match self.client.get_bookmarks(search_params).await {
            Ok(results) => serde_json::to_string(&results)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Batch update multiple bookmarks")]
    async fn batch_update_bookmarks(
        &self,
        Parameters(params): Parameters<BatchUpdateBookmarksParams>,
    ) -> String {
        info!("Batch updating {} bookmarks", params.ids.len());
        let mut updates = serde_json::Map::new();

        if let Some(collection) = params.collection {
            updates.insert(
                "collection".to_string(),
                serde_json::Value::Number(collection.into()),
            );
        }
        if let Some(tags) = params.tags {
            updates.insert("tags".to_string(), serde_json::to_value(tags).unwrap());
        }
        if let Some(important) = params.important {
            updates.insert("important".to_string(), serde_json::Value::Bool(important));
        }

        match self
            .client
            .batch_update_bookmarks(params.ids, serde_json::Value::Object(updates))
            .await
        {
            Ok(result) => serde_json::to_string(&result)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Batch delete multiple bookmarks")]
    async fn batch_delete_bookmarks(
        &self,
        Parameters(params): Parameters<BatchDeleteBookmarksParams>,
    ) -> String {
        info!("Batch deleting {} bookmarks", params.ids.len());
        match self.client.batch_delete_bookmarks(params.ids, false).await {
            Ok(result) => serde_json::to_string(&result)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    // Tag tools
    #[tool(description = "Get all tags or tags from a specific collection")]
    async fn get_tags(&self, Parameters(params): Parameters<GetTagsParams>) -> String {
        debug!("Getting tags");
        match params.collection {
            Some(collection_id) => match self.client.get_tags(Some(collection_id)).await {
                Ok(tags) => serde_json::to_string(&tags)
                    .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
                Err(e) => format!(r#"{{"error": "{e}"}}"#),
            },
            None => match self.client.get_tags(None).await {
                Ok(tags) => serde_json::to_string(&tags)
                    .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
                Err(e) => format!(r#"{{"error": "{e}"}}"#),
            },
        }
    }

    #[tool(description = "Get all tags across all collections")]
    async fn get_all_tags(&self, _params: Parameters<EmptyParams>) -> String {
        debug!("Getting all tags");
        match self.client.get_tags(None).await {
            Ok(tags) => serde_json::to_string(&tags)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Rename a tag across all bookmarks")]
    async fn rename_tag(&self, Parameters(params): Parameters<RenameTagParams>) -> String {
        info!(
            "Renaming tag from '{}' to '{}'",
            params.old_name, params.new_name
        );
        match self
            .client
            .rename_tag(params.old_name.clone(), params.new_name.clone(), None)
            .await
        {
            Ok(result) => serde_json::to_string(&result)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Delete a single tag from all bookmarks")]
    async fn delete_tag(&self, Parameters(params): Parameters<DeleteTagParams>) -> String {
        info!("Deleting tag: {}", params.name);
        match self.client.delete_tags(vec![params.name.clone()], None).await {
            Ok(_) => serde_json::json!({"success": true, "message": format!("Tag '{name}' deleted", name = params.name)}).to_string(),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Delete multiple tags from all bookmarks")]
    async fn delete_tags(&self, Parameters(params): Parameters<DeleteTagsParams>) -> String {
        info!("Deleting {} tags", params.names.len());
        match self.client.delete_tags(params.names, None).await {
            Ok(result) => serde_json::to_string(&result)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    // Highlight tools
    #[tool(description = "Get highlights with optional filtering")]
    async fn get_highlights(&self, Parameters(params): Parameters<GetHighlightsParams>) -> String {
        debug!("Getting highlights");
        let bookmark_id = params
            .bookmark_id
            .ok_or_else(|| "bookmark_id is required".to_string());
        match bookmark_id {
            Ok(id) => match self.client.get_highlights(id).await {
                Ok(highlights) => serde_json::to_string(&highlights)
                    .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
                Err(e) => format!(r#"{{"error": "{e}"}}"#),
            },
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Get all highlights across all bookmarks")]
    async fn get_all_highlights(&self, _params: Parameters<EmptyParams>) -> String {
        debug!("Getting all highlights");
        match self.client.get_all_highlights(None, None).await {
            Ok(highlights) => serde_json::to_string(&highlights)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    // User tools
    #[tool(description = "Get user account information")]
    async fn get_user_info(&self, _params: Parameters<EmptyParams>) -> String {
        debug!("Getting user info");
        match self.client.get_user_info().await {
            Ok(user) => serde_json::to_string(&user)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Get user account statistics")]
    async fn get_user_stats(&self, _params: Parameters<EmptyParams>) -> String {
        debug!("Getting user stats");
        match self.client.get_user_stats(None).await {
            Ok(stats) => serde_json::to_string(&stats)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    // Utility tools
    #[tool(description = "Empty the trash (permanently delete all trashed bookmarks)")]
    async fn empty_trash(&self, _params: Parameters<EmptyParams>) -> String {
        info!("Emptying trash");
        match self.client.empty_trash().await {
            Ok(_) => serde_json::json!({"success": true, "message": "Trash emptied successfully"})
                .to_string(),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Export bookmarks in various formats")]
    async fn export_bookmarks(
        &self,
        Parameters(params): Parameters<ExportBookmarksParams>,
    ) -> String {
        info!("Exporting bookmarks");
        use crate::raindrop::types::{ExportFormat, ExportOptions};
        let format = params
            .format
            .as_ref()
            .map(|f| match f.as_str() {
                "html" => ExportFormat::Html,
                "csv" => ExportFormat::Csv,
                "pdf" => ExportFormat::Pdf,
                _ => ExportFormat::Html,
            })
            .unwrap_or(ExportFormat::Html);

        let options = ExportOptions {
            collection: params
                .collection_ids
                .as_ref()
                .and_then(|ids| ids.first().copied()),
            format,
            broken: None,
            duplicates: None,
        };

        match self.client.export_bookmarks(options).await {
            Ok(result) => serde_json::to_string(&result)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Check the status of an ongoing import operation")]
    async fn get_import_status(&self, _params: Parameters<EmptyParams>) -> String {
        debug!("Getting import status");
        match self.client.get_import_status().await {
            Ok(status) => serde_json::to_string(&status)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }

    #[tool(description = "Check the status of an ongoing export operation")]
    async fn get_export_status(&self, _params: Parameters<EmptyParams>) -> String {
        debug!("Getting export status");
        match self.client.get_export_status().await {
            Ok(status) => serde_json::to_string(&status)
                .unwrap_or_else(|e| format!(r#"{{"error": "Serialization error: {e}"}}"#)),
            Err(e) => format!(r#"{{"error": "{e}"}}"#),
        }
    }
}

impl McpServer {
    pub fn new() -> std::result::Result<Self, RaindropMcpError> {
        let client = Arc::new(RaindropClient::new()?);

        Ok(Self {
            client,
            tool_router: Self::tool_router(),
        })
    }

    // Helper to convert our error to MCP error
    fn to_mcp_error(err: RaindropMcpError) -> McpError {
        McpError {
            code: ErrorCode(err.to_mcp_error_code()),
            message: err.to_string().into(),
            data: None,
        }
    }
}

#[tool_handler]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "A Raindrop.io MCP server for managing bookmarks and collections".into(),
            ),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
            ..Default::default()
        }
    }

    async fn list_resources(
        &self,
        _: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                Resource::new(
                    RawResource {
                        uri: "raindrop://collections/all".into(),
                        name: "All Collections".into(),
                        description: Some("List of all Raindrop collections".into()),
                        mime_type: Some("application/json".into()),
                        size: None,
                    },
                    None,
                ),
                Resource::new(
                    RawResource {
                        uri: "raindrop://tags/all".into(),
                        name: "All Tags".into(),
                        description: Some("List of all tags across all bookmarks".into()),
                        mime_type: Some("application/json".into()),
                        size: None,
                    },
                    None,
                ),
                Resource::new(
                    RawResource {
                        uri: "raindrop://highlights/all".into(),
                        name: "All Highlights".into(),
                        description: Some("List of all highlights across all bookmarks".into()),
                        mime_type: Some("application/json".into()),
                        size: None,
                    },
                    None,
                ),
                Resource::new(
                    RawResource {
                        uri: "raindrop://user/info".into(),
                        name: "User Info".into(),
                        description: Some("Current user account information".into()),
                        mime_type: Some("application/json".into()),
                        size: None,
                    },
                    None,
                ),
                Resource::new(
                    RawResource {
                        uri: "raindrop://user/stats".into(),
                        name: "User Statistics".into(),
                        description: Some("User account statistics".into()),
                        mime_type: Some("application/json".into()),
                        size: None,
                    },
                    None,
                ),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> std::result::Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "raindrop://collections/all" => match self.client.get_collections().await {
                Ok(collections) => Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(
                        format!(
                            "Found {} collections\n\nData: {}",
                            collections.len(),
                            serde_json::to_string_pretty(&collections)
                                .unwrap_or_else(|_| "Error serializing data".to_string())
                        ),
                        "raindrop://collections/all",
                    )],
                }),
                Err(e) => Err(Self::to_mcp_error(e)),
            },
            "raindrop://tags/all" => match self.client.get_tags(None).await {
                Ok(tags) => Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(
                        format!(
                            "Found {} tags\n\nData: {}",
                            tags.len(),
                            serde_json::to_string_pretty(&tags)
                                .unwrap_or_else(|_| "Error serializing data".to_string())
                        ),
                        "raindrop://tags/all",
                    )],
                }),
                Err(e) => Err(Self::to_mcp_error(e)),
            },
            "raindrop://highlights/all" => match self.client.get_all_highlights(None, None).await {
                Ok(highlights) => Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(
                        format!(
                            "Found {} highlights\n\nData: {}",
                            highlights.len(),
                            serde_json::to_string_pretty(&highlights)
                                .unwrap_or_else(|_| "Error serializing data".to_string())
                        ),
                        "raindrop://highlights/all",
                    )],
                }),
                Err(e) => Err(Self::to_mcp_error(e)),
            },
            "raindrop://user/info" => match self.client.get_user_info().await {
                Ok(user) => Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(
                        format!(
                            "User information retrieved successfully\n\nData: {}",
                            serde_json::to_string_pretty(&user)
                                .unwrap_or_else(|_| "Error serializing data".to_string())
                        ),
                        "raindrop://user/info",
                    )],
                }),
                Err(e) => Err(Self::to_mcp_error(e)),
            },
            "raindrop://user/stats" => match self.client.get_user_stats(None).await {
                Ok(stats) => Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(
                        format!(
                            "User statistics retrieved successfully\n\nData: {}",
                            serde_json::to_string_pretty(&stats)
                                .unwrap_or_else(|_| "Error serializing data".to_string())
                        ),
                        "raindrop://user/stats",
                    )],
                }),
                Err(e) => Err(Self::to_mcp_error(e)),
            },
            _ => Err(McpError::invalid_params(
                format!("Unknown resource ID: {uri}"),
                None,
            )),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts: vec![
                Prompt::new(
                    "bookmark-summary",
                    Some("Generate a summary of bookmarks in a specific collection"),
                    Some(vec![PromptArgument {
                        name: "collectionId".to_string(),
                        description: Some("ID of the collection to summarize".to_string()),
                        required: Some(true),
                    }]),
                ),
                Prompt::new(
                    "organize-unsorted",
                    Some("Suggest organization for unsorted bookmarks"),
                    Some(vec![PromptArgument {
                        name: "limit".to_string(),
                        description: Some(
                            "Maximum number of bookmarks to analyze (default: 50)".to_string(),
                        ),
                        required: Some(false),
                    }]),
                ),
                Prompt::new(
                    "weekly-digest",
                    Some("Create a digest of bookmarks added this week"),
                    None,
                ),
                Prompt::new(
                    "tag-suggestions",
                    Some("Provide tag optimization suggestions"),
                    Some(vec![PromptArgument {
                        name: "collectionId".to_string(),
                        description: Some("ID of the collection to analyze (optional)".to_string()),
                        required: Some(false),
                    }]),
                ),
                Prompt::new(
                    "duplicate-finder",
                    Some("Find potential duplicate bookmarks"),
                    None,
                ),
            ],
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        match name.as_str() {
            "bookmark-summary" => {
                let collection_id = arguments
                    .and_then(|args| args.get("collectionId")?.as_i64())
                    .ok_or_else(|| {
                        McpError::invalid_params("Missing required parameter: collectionId", None)
                    })?;

                // Fetch collection info to include in prompt
                let collection = self
                    .client
                    .get_collection(collection_id)
                    .await
                    .map_err(|_| McpError::internal_error("Failed to fetch collection", None))?;

                Ok(GetPromptResult {
                    description: Some(format!("Summary for collection: {}", collection.title)),
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(format!(
                            "Please analyze the bookmarks in the '{}' collection (ID: {}). \
                             Provide: 1) Overview of main topics, 2) Key resources, \
                             3) Content patterns, 4) Organization suggestions.",
                            collection.title, collection_id
                        )),
                    }],
                })
            }
            "organize-unsorted" => {
                let limit = arguments
                    .and_then(|args| args.get("limit")?.as_i64())
                    .unwrap_or(50) as i32;

                Ok(GetPromptResult {
                    description: Some(
                        "Organization suggestions for unsorted bookmarks".to_string(),
                    ),
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(format!(
                            "Analyze the {limit} most recent unsorted bookmarks and suggest: \
                             1) Appropriate collections/folders, 2) Relevant tags, \
                             3) Grouping by topic or type, 4) Priority for review."
                        )),
                    }],
                })
            }
            "weekly-digest" => Ok(GetPromptResult {
                description: Some("Weekly bookmark digest".to_string()),
                messages: vec![PromptMessage {
                    role: PromptMessageRole::User,
                    content: PromptMessageContent::text(
                        "Create a weekly digest of bookmarks added in the past 7 days. \
                             Include: 1) Total count and breakdown by collection, \
                             2) Key themes and topics, 3) Most important additions, \
                             4) Recommendations for organization or follow-up."
                            .to_string(),
                    ),
                }],
            }),
            "tag-suggestions" => {
                let collection_id = arguments.and_then(|args| args.get("collectionId")?.as_i64());

                let prompt_text = if let Some(id) = collection_id {
                    format!(
                        "Analyze tags in collection ID {id} and provide: \
                         1) Unused or redundant tags, 2) Tag consolidation suggestions, \
                         3) Missing tags based on content, 4) Tag hierarchy recommendations."
                    )
                } else {
                    "Analyze all tags across the library and provide: \
                     1) Unused or redundant tags, 2) Tag consolidation suggestions, \
                     3) Popular tag patterns, 4) Tag hierarchy recommendations."
                        .to_string()
                };

                Ok(GetPromptResult {
                    description: Some("Tag optimization suggestions".to_string()),
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(prompt_text),
                    }],
                })
            }
            "duplicate-finder" => Ok(GetPromptResult {
                description: Some("Find duplicate bookmarks".to_string()),
                messages: vec![PromptMessage {
                    role: PromptMessageRole::User,
                    content: PromptMessageContent::text(
                        "Identify potential duplicate bookmarks by analyzing: \
                             1) Similar URLs (including www/non-www variants), \
                             2) Similar titles, 3) Same content in different locations, \
                             4) Recommendations for which duplicates to keep or merge."
                            .to_string(),
                    ),
                }],
            }),
            _ => Err(McpError::invalid_params(
                format!("Unknown prompt: {name}"),
                None,
            )),
        }
    }
}
