use super::{Client, Result};
use crate::models::routines::{
    GetRoutineResponse, ListRoutinesResponse, PostRoutinesRequestBody, PutRoutinesRequestBody,
    Routine,
};

impl Client {
    pub fn list_routines(&self, page: i64, page_size: i64) -> Result<ListRoutinesResponse> {
        let path = format!("/v1/routines?page={page}&pageSize={page_size}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn get_routine(&self, routine_id: &str) -> Result<GetRoutineResponse> {
        let path = format!("/v1/routines/{routine_id}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn create_routine(&self, body: &PostRoutinesRequestBody) -> Result<Routine> {
        let json = serde_json::to_string_pretty(body)?;
        Ok(self
            .make_request("POST", "/v1/routines", Some(&json), || {
                self.post("/v1/routines").send(&json)
            })?
            .body_mut()
            .read_json()?)
    }

    pub fn update_routine(
        &self,
        routine_id: &str,
        body: &PutRoutinesRequestBody,
    ) -> Result<Routine> {
        let path = format!("/v1/routines/{routine_id}");
        let json = serde_json::to_string_pretty(body)?;
        Ok(self
            .make_request("PUT", &path, Some(&json), || self.put(&path).send(&json))?
            .body_mut()
            .read_json()?)
    }
}
