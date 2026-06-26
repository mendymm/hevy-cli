use super::{Client, Result};
use crate::models::routine_folders::{
    ListRoutineFoldersResponse, PostRoutineFolderRequestBody, RoutineFolder,
};

impl Client {
    pub fn list_routine_folders(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<ListRoutineFoldersResponse> {
        let path = format!("/v1/routine_folders?page={page}&pageSize={page_size}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn get_routine_folder(&self, folder_id: &str) -> Result<RoutineFolder> {
        let path = format!("/v1/routine_folders/{folder_id}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn create_routine_folder(
        &self,
        body: &PostRoutineFolderRequestBody,
    ) -> Result<RoutineFolder> {
        let json = serde_json::to_string_pretty(body)?;
        Ok(self
            .make_request("POST", "/v1/routine_folders", Some(&json), || {
                self.post("/v1/routine_folders").send(&json)
            })?
            .body_mut()
            .read_json()?)
    }
}
