use serde::{Deserialize, Serialize};

/// A single set entry from an exercise's history across all workouts
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseHistoryEntry {
    /// The workout ID this entry belongs to
    pub workout_id: String,
    /// The title of the workout
    pub workout_title: String,
    /// ISO 8601 timestamp of when the workout started
    pub workout_start_time: String,
    /// ISO 8601 timestamp of when the workout ended
    pub workout_end_time: String,
    /// The exercise template ID
    pub exercise_template_id: String,
    /// Weight lifted in kilograms
    pub weight_kg: Option<f64>,
    /// Number of repetitions
    pub reps: Option<i64>,
    /// Distance in meters
    pub distance_meters: Option<i64>,
    /// Duration in seconds
    pub duration_seconds: Option<i64>,
    /// RPE (Rating of Perceived Exertion)
    pub rpe: Option<f64>,
    /// Custom metric for the set (e.g. steps or floors)
    pub custom_metric: Option<f64>,
    /// The type of set: warmup, normal, failure, dropset
    pub set_type: String,
}

/// Response for GET /v1/exercise_history/{exerciseTemplateId}
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseHistoryResponse {
    /// All matching set entries for the given exercise, ordered by date
    pub exercise_history: Vec<ExerciseHistoryEntry>,
}
