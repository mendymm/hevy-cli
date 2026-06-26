use serde::{Deserialize, Serialize};

/// Basic info about a Hevy user
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// The user ID
    pub id: String,
    /// The user's display name
    pub name: String,
    /// The user's public profile URL (e.g. "https://hevy.com/user/johndoe")
    pub url: String,
}

/// Response for GET /v1/user/info
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub data: UserInfo,
}
