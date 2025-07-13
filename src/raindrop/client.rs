use reqwest::{Client, StatusCode};
use serde_json::json;
use std::env;
use tracing::{debug, info};

use super::types::*;
use crate::error::{RaindropMcpError, Result};

const BASE_URL: &str = "https://api.raindrop.io/rest/v1";

pub struct RaindropClient {
    client: Client,
    base_url: String,
}

impl RaindropClient {
    pub fn new() -> Result<Self> {
        let base_url = env::var("RAINDROP_BASE_URL").unwrap_or_else(|_| BASE_URL.to_string());
        Self::with_base_url(base_url)
    }

    pub fn with_base_url(base_url: String) -> Result<Self> {
        let access_token = env::var("RAINDROP_ACCESS_TOKEN")
            .map_err(|_| RaindropMcpError::EnvironmentVariable(
                "RAINDROP_ACCESS_TOKEN environment variable is required. Please check your .env file or environment settings.".to_string()
            ))?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {access_token}").parse().unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(RaindropMcpError::HttpRequest)?;

        Ok(Self { client, base_url })
    }

    // Helper method to handle API errors
    async fn handle_response<T: for<'de> serde::Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();
        let url = response.url().to_string();

        match status {
            StatusCode::OK | StatusCode::CREATED => {
                let text = response.text().await?;
                serde_json::from_str::<T>(&text).map_err(RaindropMcpError::JsonSerialization)
            }
            StatusCode::UNAUTHORIZED => Err(RaindropMcpError::Unauthorized(format!(
                "Invalid or expired access token for {url}"
            ))),
            StatusCode::NOT_FOUND => Err(RaindropMcpError::NotFound(format!(
                "Resource not found: {url}"
            ))),
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(RaindropMcpError::RaindropApi(
                    format!("API error ({status}): {error_text}"),
                    None,
                ))
            }
        }
    }

    // Collections API
    pub async fn get_collections(&self) -> Result<Vec<Collection>> {
        debug!("Fetching all collections");
        let response = self
            .client
            .get(format!("{}/collections", self.base_url))
            .send()
            .await?;

        let result: CollectionsResponse = self.handle_response(response).await?;

        // Process collections to handle type discrepancies
        let processed_collections: Vec<Collection> = result.items;

        Ok(processed_collections)
    }

    pub async fn get_collection(&self, id: i64) -> Result<Collection> {
        debug!("Fetching collection with id: {}", id);
        let response = self
            .client
            .get(format!("{}/collection/{}", self.base_url, id))
            .send()
            .await?;

        let result: CollectionResponse = self.handle_response(response).await?;
        Ok(result.item)
    }

    pub async fn get_child_collections(&self, parent_id: i64) -> Result<Vec<Collection>> {
        debug!("Fetching child collections for parent: {}", parent_id);
        let response = self
            .client
            .get(format!(
                "{}/collections/{}/childrens",
                self.base_url, parent_id
            ))
            .send()
            .await?;

        let result: CollectionsResponse = self.handle_response(response).await?;
        Ok(result.items)
    }

    pub async fn create_collection(&self, title: String, is_public: bool) -> Result<Collection> {
        info!("Creating new collection: {}", title);
        let body = json!({
            "title": title,
            "public": is_public
        });

        let response = self
            .client
            .post(format!("{}/collection", self.base_url))
            .json(&body)
            .send()
            .await?;

        let result: CollectionResponse = self.handle_response(response).await?;
        Ok(result.item)
    }

    pub async fn update_collection(
        &self,
        id: i64,
        updates: serde_json::Value,
    ) -> Result<Collection> {
        info!("Updating collection: {}", id);
        let response = self
            .client
            .put(format!("{}/collection/{}", self.base_url, id))
            .json(&updates)
            .send()
            .await?;

        let result: CollectionResponse = self.handle_response(response).await?;
        Ok(result.item)
    }

    pub async fn delete_collection(&self, id: i64) -> Result<()> {
        info!("Deleting collection: {}", id);
        let response = self
            .client
            .delete(format!("{}/collection/{}", self.base_url, id))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            _ => self
                .handle_response::<serde_json::Value>(response)
                .await
                .map(|_| ()),
        }
    }

    pub async fn share_collection(
        &self,
        id: i64,
        level: &str,
        emails: Option<Vec<String>>,
    ) -> Result<serde_json::Value> {
        info!("Sharing collection {} with level: {}", id, level);
        let mut body = json!({
            "level": level
        });

        if let Some(emails) = emails {
            body["emails"] = json!(emails);
        }

        let response = self
            .client
            .put(format!("{}/collection/{}/sharing", self.base_url, id))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    // Bookmarks API
    pub async fn get_bookmarks(&self, params: SearchParams) -> Result<BookmarksResponse> {
        debug!("Fetching bookmarks with params: {:?}", params);

        let mut query_params = vec![];

        if let Some(search) = &params.search {
            query_params.push(("search", urlencoding::encode(search).to_string()));
        }
        if let Some(collection) = params.collection {
            query_params.push(("collection", collection.to_string()));
        }
        if let Some(tags) = &params.tags {
            for tag in tags {
                query_params.push(("tag", tag.clone()));
            }
        }
        if let Some(page) = params.page {
            query_params.push(("page", page.to_string()));
        }
        if let Some(per_page) = params.per_page {
            query_params.push(("perpage", per_page.to_string()));
        }
        if let Some(sort) = &params.sort {
            query_params.push(("sort", sort.clone()));
        }
        if let Some(important) = params.important {
            query_params.push(("important", important.to_string()));
        }

        let collection_id = params.collection.unwrap_or(0);
        let response = self
            .client
            .get(format!("{}/raindrops/{}", self.base_url, collection_id))
            .query(&query_params)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn get_bookmark(&self, id: i64) -> Result<Bookmark> {
        debug!("Fetching bookmark with id: {}", id);
        let response = self
            .client
            .get(format!("{}/raindrop/{}", self.base_url, id))
            .send()
            .await?;

        let result: BookmarkResponse = self.handle_response(response).await?;
        Ok(result.item)
    }

    pub async fn create_bookmark(
        &self,
        link: String,
        collection_id: i64,
        title: Option<String>,
        excerpt: Option<String>,
        tags: Option<Vec<String>>,
        important: Option<bool>,
    ) -> Result<Bookmark> {
        info!("Creating bookmark: {}", link);
        let mut body = json!({
            "link": link,
            "collection": { "$id": collection_id }
        });

        if let Some(title) = title {
            body["title"] = json!(title);
        }
        if let Some(excerpt) = excerpt {
            body["excerpt"] = json!(excerpt);
        }
        if let Some(tags) = tags {
            body["tags"] = json!(tags);
        }
        if let Some(important) = important {
            body["important"] = json!(important);
        }

        let response = self
            .client
            .post(format!("{}/raindrop", self.base_url))
            .json(&body)
            .send()
            .await?;

        let result: BookmarkResponse = self.handle_response(response).await?;
        Ok(result.item)
    }

    pub async fn update_bookmark(&self, id: i64, updates: serde_json::Value) -> Result<Bookmark> {
        info!("Updating bookmark: {}", id);
        let response = self
            .client
            .put(format!("{}/raindrop/{}", self.base_url, id))
            .json(&updates)
            .send()
            .await?;

        let result: BookmarkResponse = self.handle_response(response).await?;
        Ok(result.item)
    }

    pub async fn delete_bookmark(&self, id: i64, permanent: bool) -> Result<()> {
        info!("Deleting bookmark: {} (permanent: {})", id, permanent);

        let url = if permanent {
            format!("{}/raindrop/{}", self.base_url, id)
        } else {
            format!("{}/raindrop/{}/trash", self.base_url, id)
        };

        let response = self.client.delete(url).send().await?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            _ => self
                .handle_response::<serde_json::Value>(response)
                .await
                .map(|_| ()),
        }
    }

    pub async fn batch_update_bookmarks(
        &self,
        ids: Vec<i64>,
        updates: serde_json::Value,
    ) -> Result<serde_json::Value> {
        info!("Batch updating {} bookmarks", ids.len());
        let mut body = updates;
        body["ids"] = json!(ids);

        let response = self
            .client
            .put(format!("{}/raindrops", self.base_url))
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn batch_delete_bookmarks(&self, ids: Vec<i64>, permanent: bool) -> Result<()> {
        info!(
            "Batch deleting {} bookmarks (permanent: {})",
            ids.len(),
            permanent
        );

        let body = json!({
            "ids": ids
        });

        let url = if permanent {
            format!("{}/raindrops", self.base_url)
        } else {
            format!("{}/raindrops/trash", self.base_url)
        };

        let response = self.client.delete(url).json(&body).send().await?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            _ => self
                .handle_response::<serde_json::Value>(response)
                .await
                .map(|_| ()),
        }
    }

    // Tags API
    pub async fn get_tags(&self, collection_id: Option<i64>) -> Result<Vec<Tag>> {
        debug!("Fetching tags for collection: {:?}", collection_id);

        let url = if let Some(id) = collection_id {
            format!("{}/tags/{}", self.base_url, id)
        } else {
            format!("{}/tags", self.base_url)
        };

        let response = self.client.get(url).send().await?;

        let result: TagsResponse = self.handle_response(response).await?;
        Ok(result.items)
    }

    pub async fn rename_tag(
        &self,
        old_name: String,
        new_name: String,
        collection_id: Option<i64>,
    ) -> Result<()> {
        info!("Renaming tag '{}' to '{}'", old_name, new_name);

        let url = if let Some(id) = collection_id {
            format!("{}/tags/{}", self.base_url, id)
        } else {
            format!("{}/tags", self.base_url)
        };

        let body = json!({
            "replace": old_name,
            "tag": new_name
        });

        let response = self.client.put(url).json(&body).send().await?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            _ => self
                .handle_response::<serde_json::Value>(response)
                .await
                .map(|_| ()),
        }
    }

    pub async fn delete_tags(&self, tags: Vec<String>, collection_id: Option<i64>) -> Result<()> {
        info!("Deleting {} tags", tags.len());

        let url = if let Some(id) = collection_id {
            format!("{}/tags/{}", self.base_url, id)
        } else {
            format!("{}/tags", self.base_url)
        };

        let body = json!({
            "tags": tags
        });

        let response = self.client.delete(url).json(&body).send().await?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            _ => self
                .handle_response::<serde_json::Value>(response)
                .await
                .map(|_| ()),
        }
    }

    // Highlights API
    pub async fn get_highlights(&self, raindrop_id: i64) -> Result<Vec<Highlight>> {
        debug!("Fetching highlights for raindrop: {}", raindrop_id);
        let response = self
            .client
            .get(format!(
                "{}/raindrop/{}/highlights",
                self.base_url, raindrop_id
            ))
            .send()
            .await?;

        let result: HighlightsResponse = self.handle_response(response).await?;
        Ok(result.items)
    }

    pub async fn get_all_highlights(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<Vec<Highlight>> {
        debug!(
            "Fetching all highlights (page: {:?}, per_page: {:?})",
            page, per_page
        );

        let mut query_params = vec![];
        if let Some(page) = page {
            query_params.push(("page", page.to_string()));
        }
        if let Some(per_page) = per_page {
            query_params.push(("perpage", per_page.to_string()));
        }

        let response = self
            .client
            .get(format!("{}/highlights", self.base_url))
            .query(&query_params)
            .send()
            .await?;

        let result: HighlightsResponse = self.handle_response(response).await?;
        Ok(result.items)
    }

    // User API
    pub async fn get_user_info(&self) -> Result<User> {
        debug!("Fetching user info");
        let response = self
            .client
            .get(format!("{}/user", self.base_url))
            .send()
            .await?;

        let result: UserResponse = self.handle_response(response).await?;
        Ok(result.user)
    }

    pub async fn get_user_stats(&self, collection_id: Option<i64>) -> Result<UserStats> {
        debug!("Fetching user stats for collection: {:?}", collection_id);

        let url = if let Some(id) = collection_id {
            format!("{}/stats/collection/{}", self.base_url, id)
        } else {
            format!("{}/user/stats", self.base_url)
        };

        let response = self.client.get(url).send().await?;

        self.handle_response(response).await
    }

    // Utility APIs
    pub async fn empty_trash(&self) -> Result<()> {
        info!("Emptying trash");
        let response = self
            .client
            .delete(format!("{}/raindrops/-99", self.base_url))
            .send()
            .await?;

        match response.status() {
            StatusCode::OK | StatusCode::NO_CONTENT => Ok(()),
            _ => self
                .handle_response::<serde_json::Value>(response)
                .await
                .map(|_| ()),
        }
    }

    pub async fn export_bookmarks(&self, options: ExportOptions) -> Result<serde_json::Value> {
        info!("Exporting bookmarks with format: {:?}", options.format);

        let response = self
            .client
            .post(format!("{}/export", self.base_url))
            .json(&options)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn get_import_status(&self) -> Result<ImportStatus> {
        debug!("Checking import status");
        let response = self
            .client
            .get(format!("{}/import", self.base_url))
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn get_export_status(&self) -> Result<ExportStatus> {
        debug!("Checking export status");
        let response = self
            .client
            .get(format!("{}/export", self.base_url))
            .send()
            .await?;

        self.handle_response(response).await
    }
}
