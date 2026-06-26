use serde::{Deserialize, Serialize};

/// A routine folder, used to organize routines into groups
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineFolder {
    /// The routine folder ID
    pub id: f64,
    /// The position of this folder in the folder list (0-based)
    pub index: f64,
    /// The folder title
    pub title: String,
    /// ISO 8601 timestamp of when the folder was last updated
    pub updated_at: String,
    /// ISO 8601 timestamp of when the folder was created
    pub created_at: String,
}

/// Paginated list of routine folders (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRoutineFoldersResponse {
    /// Current page number
    pub page: i64,
    /// Total number of pages
    pub page_count: i64,
    /// The routine folders on this page
    pub routine_folders: Vec<RoutineFolder>,
}

/// Inner body for creating a routine folder
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineFolderInner {
    /// The title of the routine folder
    pub title: String,
}

/// Request body for POST /v1/routine_folders.
/// The new folder is inserted at index 0; all existing folders have their indexes incremented.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineFolderRequestBody {
    pub routine_folder: PostRoutineFolderInner,
}
