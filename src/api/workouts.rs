use super::{Client, Result};
use crate::models::workouts::{
    ListWorkoutsResponse, PaginatedWorkoutEvents, PostWorkoutsRequestBody, Workout,
    WorkoutCountResponse,
};

impl Client {
    pub fn list_workouts(&self, page: i64, page_size: i64) -> Result<ListWorkoutsResponse> {
        let path = format!("/v1/workouts?page={page}&pageSize={page_size}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn get_workout_count(&self) -> Result<WorkoutCountResponse> {
        Ok(self
            .make_request("GET", "/v1/workouts/count", None, || {
                self.get("/v1/workouts/count").call()
            })?
            .body_mut()
            .read_json()?)
    }

    pub fn get_workout(&self, workout_id: &str) -> Result<Workout> {
        let path = format!("/v1/workouts/{workout_id}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }

    pub fn create_workout(&self, body: &PostWorkoutsRequestBody) -> Result<Workout> {
        let json = serde_json::to_string_pretty(body)?;
        Ok(self
            .make_request("POST", "/v1/workouts", Some(&json), || {
                self.post("/v1/workouts").send(&json)
            })?
            .body_mut()
            .read_json()?)
    }

    pub fn update_workout(
        &self,
        workout_id: &str,
        body: &PostWorkoutsRequestBody,
    ) -> Result<Workout> {
        let path = format!("/v1/workouts/{workout_id}");
        let json = serde_json::to_string_pretty(body)?;
        Ok(self
            .make_request("PUT", &path, Some(&json), || self.put(&path).send(&json))?
            .body_mut()
            .read_json()?)
    }

    pub fn get_workout_events(
        &self,
        page: i64,
        page_size: i64,
        since: &str,
    ) -> Result<PaginatedWorkoutEvents> {
        let path = format!("/v1/workouts/events?page={page}&pageSize={page_size}&since={since}");
        Ok(self
            .make_request("GET", &path, None, || self.get(&path).call())?
            .body_mut()
            .read_json()?)
    }
}
