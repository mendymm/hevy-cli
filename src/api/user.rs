use super::{Client, Result};
use crate::models::user::UserInfoResponse;

impl Client {
    pub fn get_user_info(&self) -> Result<UserInfoResponse> {
        Ok(self
            .make_request("GET", "/v1/user/info", None, || {
                self.get("/v1/user/info").call()
            })?
            .body_mut()
            .read_json()?)
    }
}
