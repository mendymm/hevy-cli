use super::{Client, Result};
use crate::models::exercise_history::ExerciseHistoryResponse;

impl Client {
    pub fn get_exercise_history(
        &self,
        exercise_template_id: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<ExerciseHistoryResponse> {
        let mut path = format!("/v1/exercise_history/{exercise_template_id}?");
        if let Some(s) = start_date {
            path.push_str(&format!("start_date={s}&"));
        }
        if let Some(e) = end_date {
            path.push_str(&format!("end_date={e}&"));
        }
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }
}
