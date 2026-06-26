use serde::{Deserialize, Serialize};

/// A set within a workout exercise (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSet {
    /// Index indicating the order of the set in the workout
    pub index: f64,
    /// The type of set: normal, warmup, dropset, failure
    #[serde(rename = "type")]
    pub set_type: String,
    /// Weight lifted in kilograms
    pub weight_kg: Option<f64>,
    /// Number of reps logged for the set
    pub reps: Option<f64>,
    /// Number of meters logged for the set
    pub distance_meters: Option<f64>,
    /// Number of seconds logged for the set
    pub duration_seconds: Option<f64>,
    /// RPE (Relative Perceived Exertion) value logged for the set
    pub rpe: Option<f64>,
    /// Custom metric logged for the set (currently used for floors or steps on stair machine exercises)
    pub custom_metric: Option<f64>,
}

/// An exercise within a workout (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutExercise {
    /// Index indicating the order of the exercise in the workout
    pub index: f64,
    /// Title of the exercise (e.g. "Bench Press (Barbell)")
    pub title: String,
    /// Notes on the exercise
    pub notes: Option<String>,
    /// The id of the exercise template. Can be used to fetch the exercise template.
    pub exercise_template_id: String,
    /// The id of the superset this exercise belongs to. Null if not in a superset.
    pub supersets_id: Option<f64>,
    /// The sets logged for this exercise
    pub sets: Vec<WorkoutSet>,
}

/// A completed workout (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workout {
    /// The workout ID
    pub id: String,
    /// The workout title
    pub title: String,
    /// The ID of the routine this workout was based on. Null if not from a routine.
    pub routine_id: Option<String>,
    /// A description of the workout
    pub description: Option<String>,
    /// ISO 8601 timestamp of when the workout started
    pub start_time: String,
    /// ISO 8601 timestamp of when the workout ended
    pub end_time: String,
    /// ISO 8601 timestamp of when the workout was last updated
    pub updated_at: String,
    /// ISO 8601 timestamp of when the workout was created
    pub created_at: String,
    /// The exercises logged in this workout
    pub exercises: Vec<WorkoutExercise>,
}

/// Paginated list of workouts (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkoutsResponse {
    /// Current page number
    pub page: i64,
    /// Total number of pages
    pub page_count: i64,
    /// The workouts on this page
    pub workouts: Vec<Workout>,
}

/// Total workout count (response for GET /v1/workouts/count)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutCountResponse {
    /// The total number of workouts on the account
    pub workout_count: i64,
}

/// Paginated list of workout events (response for GET /v1/workouts/events)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedWorkoutEvents {
    /// Current page number
    pub page: i64,
    /// Total number of pages
    pub page_count: i64,
    /// Workout events (each is either an updated or deleted event). Use the "type" field to discriminate.
    pub events: Vec<serde_json::Value>,
}

/// A set within a workout exercise (request)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWorkoutSet {
    /// The type of the set: warmup, normal, failure, dropset
    #[serde(rename = "type")]
    pub set_type: String,
    /// The weight in kilograms
    pub weight_kg: Option<f64>,
    /// The number of repetitions
    pub reps: Option<i64>,
    /// The distance in meters
    pub distance_meters: Option<i64>,
    /// The duration in seconds
    pub duration_seconds: Option<i64>,
    /// A custom metric for the set (currently used for steps and floors)
    pub custom_metric: Option<f64>,
    /// RPE (Rating of Perceived Exertion). Allowed values: 6, 7, 7.5, 8, 8.5, 9, 9.5, 10
    pub rpe: Option<f64>,
}

/// An exercise within a workout (request)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWorkoutExercise {
    /// The ID of the exercise template
    pub exercise_template_id: String,
    /// The ID of the superset. Null if not in a superset.
    pub superset_id: Option<i64>,
    /// Additional notes for the exercise
    pub notes: Option<String>,
    /// The sets logged for this exercise
    pub sets: Vec<PostWorkoutSet>,
}

/// Inner body for creating or updating a workout
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWorkoutInner {
    /// The title of the workout
    pub title: String,
    /// A description of the workout
    pub description: Option<String>,
    /// ISO 8601 timestamp of when the workout started (e.g. "2024-08-14T12:00:00Z")
    pub start_time: String,
    /// ISO 8601 timestamp of when the workout ended (e.g. "2024-08-14T13:00:00Z")
    pub end_time: String,
    /// Whether the workout is private
    pub is_private: Option<bool>,
    /// The exercises logged in this workout
    pub exercises: Vec<PostWorkoutExercise>,
}

/// Request body for POST /v1/workouts and PUT /v1/workouts/{workoutId}
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWorkoutsRequestBody {
    pub workout: PostWorkoutInner,
}
