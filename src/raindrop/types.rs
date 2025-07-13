use serde::{Deserialize, Serialize};

// User types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: i64,
    pub email: Option<String>,
    #[serde(rename = "email_MD5")]
    pub email_md5: Option<String>,
    pub full_name: Option<String>,
    pub pro: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro_expire: Option<String>,
    pub registered: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<bool>,
    // pub config: Option<UserConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<FilesInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<SocialConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<SocialConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vkontakte: Option<SocialConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google: Option<SocialConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropbox: Option<BackupConnection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdrive: Option<BackupConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub title: String,
    pub hidden: bool,
    pub sort: i32,
    pub collections: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    #[serde(default)]
    pub broken_level: Option<BrokenLevel>,
    #[serde(default)]
    pub font_color: Option<String>,
    #[serde(default)]
    pub font_size: Option<i32>,
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default)]
    pub last_collection: Option<i64>,
    #[serde(default)]
    pub raindrops_sort: Option<String>,
    #[serde(default)]
    pub raindrops_view: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BrokenLevel {
    Basic,
    Default,
    Strict,
    Off,
}

// Additional user-related types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesInfo {
    pub used: i64,
    pub size: i64,
    pub last_check_point: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialConnection {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConnection {
    pub enabled: bool,
}

// Collection types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "_id")]
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub public: Option<bool>,
    pub view: CollectionView,
    pub sort: i32,
    pub cover: Option<Vec<String>>,
    pub count: i32,
    pub expanded: Option<bool>,
    pub parent: Option<ParentRef>,
    pub user: UserRef,
    pub created: String,
    pub last_update: String,
    pub creator_ref: Option<CreatorRef>,
    pub collaborators: Option<Vec<Collaborator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CollectionView {
    List,
    Simple,
    Grid,
    Masonry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessInfo {
    pub level: i32,
    pub draggable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentRef {
    #[serde(rename = "$id")]
    pub id: i64,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRef {
    #[serde(rename = "$id")]
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorRef {
    #[serde(rename = "_id")]
    pub id: i64,
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    #[serde(rename = "_id")]
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub role: CollaboratorRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CollaboratorRole {
    Owner,
    Viewer,
    Editor,
}

// Bookmark types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    #[serde(rename = "_id")]
    pub id: i64,
    pub title: String,
    pub excerpt: Option<String>,
    pub note: Option<String>,
    #[serde(rename = "type")]
    pub bookmark_type: BookmarkType,
    pub tags: Vec<String>,
    pub cover: Option<String>,
    pub link: String,
    pub domain: String,
    pub created: String,
    pub last_update: String,
    pub media: Option<Vec<Media>>,
    pub user: UserRef,
    pub collection: CollectionRef,
    pub important: bool,
    pub highlights: Option<Vec<Highlight>>,
    pub reminder: Option<Reminder>,
    pub broken: bool,
    pub cache: Option<CacheInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BookmarkType {
    Link,
    Article,
    Image,
    Video,
    Document,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub link: String,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Pdf,
    Doc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "type")]
    pub file_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRef {
    #[serde(rename = "$id")]
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub data: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub status: CacheStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CacheStatus {
    Ready,
    Retry,
    Failed,
    InvalidOrigin,
    InvalidTimeout,
    InvalidSize,
}

// Highlight types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HighlightColor {
    Blue,
    Brown,
    Cyan,
    Gray,
    Green,
    Indigo,
    Orange,
    Pink,
    Purple,
    Red,
    Teal,
    Yellow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Highlight {
    #[serde(rename = "_id")]
    pub id: String,
    pub text: String,
    pub note: Option<String>,
    pub color: Option<HighlightColor>,
    pub created: String,
    pub last_update: Option<String>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub link: Option<String>,
    pub domain: Option<String>,
    pub excerpt: Option<String>,
    pub raindrop: RaindropRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaindropRef {
    #[serde(rename = "_id")]
    pub id: i64,
    pub title: Option<String>,
    pub link: Option<String>,
    pub collection: Option<CollectionRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightContent {
    pub uri: String,
    pub text: String,
    pub metadata: HighlightMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightMetadata {
    pub id: String,
    pub note: String,
    pub created: String,
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub link: String,
    pub raindrop: Option<RaindropRef>,
}

// Search and filter types
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    pub search: Option<String>,
    pub collection: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub page: Option<i32>,
    #[serde(rename = "perpage")]
    pub per_page: Option<i32>,
    pub sort: Option<String>,
    pub important: Option<bool>,
    pub media: Option<MediaFilter>,
    pub word: Option<String>,
    pub please_parse: Option<bool>,
    pub noparse: Option<bool>,
    pub since: Option<String>, // ISO string date
    pub created: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaFilter {
    Image,
    Video,
    Document,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    #[serde(rename = "$gte")]
    pub gte: Option<String>,
    #[serde(rename = "$lte")]
    pub lte: Option<String>,
}

// Statistics types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    pub count: i32,
    pub last_bookmark_created: String,
    pub last_bookmark_updated: String,
    pub today: i32,
    pub tags: i32,
    pub collections: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionStats {
    pub count: i32,
    pub last_bookmark_created: String,
    pub last_bookmark_updated: String,
    pub oldest: BookmarkStat,
    pub newest: BookmarkStat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkStat {
    pub id: i64,
    pub created: String,
    pub title: String,
    pub link: String,
}

// Import/Export types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOptions {
    pub format: Option<ImportFormat>,
    pub mode: Option<ImportMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportFormat {
    Html,
    Csv,
    Pocket,
    Instapaper,
    Netscape,
    Readwise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImportMode {
    Add,
    Replace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatus {
    pub status: ProcessStatus,
    pub progress: Option<i32>,
    pub imported: Option<i32>,
    pub duplicates: Option<i32>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub collection: Option<i64>,
    pub format: ExportFormat,
    pub broken: Option<bool>,
    pub duplicates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Csv,
    Html,
    Pdf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportStatus {
    pub status: ProcessStatus,
    pub progress: Option<i32>,
    pub url: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessStatus {
    InProgress,
    Ready,
    Error,
}

// API Response wrappers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResponse {
    pub item: Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionsResponse {
    pub items: Vec<Collection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarkResponse {
    pub item: Bookmark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookmarksResponse {
    pub items: Vec<Bookmark>,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagsResponse {
    pub items: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "_id")]
    pub id: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightsResponse {
    pub items: Vec<Highlight>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn test_user_serialization() {
        let user = User {
            id: 12345,
            email: Some("test@example.com".to_string()),
            email_md5: Some("13a0a20681d8781912e5314150694bf7".to_string()),
            full_name: Some("Test User".to_string()),
            pro: true,
            pro_expire: Some("2028-09-27T22:00:00.000Z".to_string()),
            registered: Some("2023-01-01T00:00:00Z".to_string()),
            password: Some(true),
            groups: None,
            avatar: None,
            files: None,
            facebook: None,
            twitter: None,
            vkontakte: None,
            google: None,
            dropbox: None,
            gdrive: None,
        };

        let json = serde_json::to_value(&user).unwrap();
        assert_eq!(json["_id"], 12345);
        assert_eq!(json["email"], "test@example.com");
        assert_eq!(json["email_MD5"], "13a0a20681d8781912e5314150694bf7");
        assert_eq!(json["fullName"], "Test User");
        assert_eq!(json["pro"], true);
        assert_eq!(json["proExpire"], "2028-09-27T22:00:00.000Z");
    }

    #[test]
    fn test_collection_deserialization() {
        let json = json!({
            "_id": 123,
            "title": "My Collection",
            "description": "Test collection",
            "color": "#FF0000",
            "public": true,
            "view": "list",
            "sort": 123,
            "cover": ["https://example.com/cover.jpg"],
            "count": 42,
            "expanded": true,
            "parent": {
                "$id": 456,
                "title": "Parent Collection"
            },
            "user": {
                "$id": 789
            },
            "created": "2023-01-01T00:00:00Z",
            "lastUpdate": "2023-01-02T00:00:00Z",
            "creatorRef": {
                "_id": 789,
                "fullName": "Creator Name"
            },
            "collaborators": [{
                "_id": 999,
                "email": "collab@example.com",
                "name": "Collaborator",
                "role": "editor"
            }]
        });

        let collection: Collection = serde_json::from_value(json).unwrap();
        assert_eq!(collection.id, 123);
        assert_eq!(collection.title, "My Collection");
        assert_eq!(collection.count, 42);
        assert!(collection.public.unwrap());
        assert!(collection.parent.is_some());
        assert_eq!(collection.parent.unwrap().id, 456);
    }

    #[test]
    fn test_bookmark_types() {
        let json = json!({
            "_id": 1001,
            "title": "Test Bookmark",
            "excerpt": "This is a test",
            "note": "My note",
            "type": "link",
            "tags": ["test", "example"],
            "cover": "https://example.com/cover.jpg",
            "link": "https://example.com",
            "domain": "example.com",
            "created": "2023-01-01T00:00:00Z",
            "lastUpdate": "2023-01-02T00:00:00Z",
            "media": [],
            "user": { "$id": 123 },
            "collection": { "$id": 456 },
            "important": true,
            "highlights": [],
            "reminder": null,
            "broken": false,
            "cache": {
                "status": "ready",
                "size": 1024,
                "created": "2023-01-01T00:00:00Z"
            }
        });

        let bookmark: Bookmark = serde_json::from_value(json).unwrap();
        assert_eq!(bookmark.id, 1001);
        assert_eq!(bookmark.title, "Test Bookmark");
        assert_eq!(bookmark.tags.len(), 2);
        assert!(bookmark.important);
        assert!(!bookmark.broken);
        matches!(bookmark.bookmark_type, BookmarkType::Link);
    }

    #[test]
    fn test_search_params_default() {
        let params = SearchParams::default();
        assert!(params.search.is_none());
        assert!(params.collection.is_none());
        assert!(params.tags.is_none());
        assert!(params.page.is_none());
        assert!(params.per_page.is_none());
    }

    #[test]
    fn test_enum_serialization() {
        // Test CollectionView
        assert_eq!(
            serde_json::to_string(&CollectionView::List).unwrap(),
            "\"list\""
        );
        assert_eq!(
            serde_json::to_string(&CollectionView::Grid).unwrap(),
            "\"grid\""
        );

        // Test BookmarkType
        assert_eq!(
            serde_json::to_string(&BookmarkType::Link).unwrap(),
            "\"link\""
        );
        assert_eq!(
            serde_json::to_string(&BookmarkType::Video).unwrap(),
            "\"video\""
        );

        // Test ProcessStatus
        assert_eq!(
            serde_json::to_string(&ProcessStatus::InProgress).unwrap(),
            "\"in-progress\""
        );
        assert_eq!(
            serde_json::to_string(&ProcessStatus::Ready).unwrap(),
            "\"ready\""
        );
    }

    #[test]
    fn test_api_response_wrappers() {
        let collection = Collection {
            id: 123,
            title: "Test".to_string(),
            description: None,
            color: None,
            public: Some(false),
            view: CollectionView::List,
            sort: 0,
            cover: None,
            count: 0,
            expanded: None,
            parent: None,
            user: UserRef { id: 456 },
            created: "2023-01-01T00:00:00Z".to_string(),
            last_update: "2023-01-01T00:00:00Z".to_string(),
            creator_ref: None,
            collaborators: None,
            access: None,
        };

        let response = CollectionResponse {
            item: collection.clone(),
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["item"]["_id"], 123);
        assert_eq!(json["item"]["title"], "Test");
    }

    #[test]
    fn test_user_deserialization_from_api() {
        let json = json!({
            "result": true,
            "user": {
                "_id": 32,
                "config": {
                    "broken_level": "strict",
                    "font_color": "",
                    "font_size": 0,
                    "lang": "ru_RU",
                    "last_collection": 8492393,
                    "raindrops_sort": "-lastUpdate",
                    "raindrops_view": "list"
                },
                "dropbox": {
                    "enabled": true
                },
                "email": "some@email.com",
                "email_MD5": "13a0a20681d8781912e5314150694bf7",
                "files": {
                    "used": 6766094,
                    "size": 10_000_000_000u64,
                    "lastCheckPoint": "2020-01-26T23:53:19.676Z"
                },
                "fullName": "Mussabekov Rustem",
                "gdrive": {
                    "enabled": true
                },
                "groups": [
                    {
                        "title": "My Collections",
                        "hidden": false,
                        "sort": 0,
                        "collections": [8364483, 8364403, 66]
                    }
                ],
                "password": true,
                "pro": true,
                "proExpire": "2028-09-27T22:00:00.000Z",
                "registered": "2014-09-30T07:51:15.406Z"
            }
        });

        let response: UserResponse = serde_json::from_value(json).unwrap();
        let user = response.user;

        assert_eq!(user.id, 32);
        assert_eq!(user.email, Some("some@email.com".to_string()));
        assert_eq!(
            user.email_md5,
            Some("13a0a20681d8781912e5314150694bf7".to_string())
        );
        assert_eq!(user.full_name, Some("Mussabekov Rustem".to_string()));
        assert!(user.pro);
        assert_eq!(
            user.pro_expire,
            Some("2028-09-27T22:00:00.000Z".to_string())
        );
        assert!(user.password.unwrap());

        // Config field removed - test temporarily disabled

        let groups = user.groups.unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "My Collections");
        assert_eq!(groups[0].collections.len(), 3);
    }
}
