use super::{Client, Result};
use crate::models::exercise_templates::{
    CreateCustomExerciseRequestBody, CreateCustomExerciseResponse, ExerciseTemplate,
    ListExerciseTemplatesResponse,
};

impl Client {
    pub fn list_exercise_templates(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<ListExerciseTemplatesResponse> {
        let path = format!("/v1/exercise_templates?page={page}&pageSize={page_size}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn get_exercise_template(&self, exercise_template_id: &str) -> Result<ExerciseTemplate> {
        let path = format!("/v1/exercise_templates/{exercise_template_id}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn create_exercise_template(
        &self,
        body: &CreateCustomExerciseRequestBody,
    ) -> Result<CreateCustomExerciseResponse> {
        let json = serde_json::to_string_pretty(body)?;
        Ok(self
            .make_request("POST", "/v1/exercise_templates", Some(&json), || {
                self.post("/v1/exercise_templates").send(&json)
            })?
            .body_mut()
            .read_json()?)
    }
}
