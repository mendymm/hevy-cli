use serde::{Deserialize, Serialize};

/// The type of a set
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    Warmup,
    Normal,
    Failure,
    Dropset,
}

/// A rep range for a set in a routine
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepRange {
    /// Starting rep count for the range
    pub start: Option<f64>,
    /// Ending rep count for the range
    pub end: Option<f64>,
}

/// A set within a routine exercise (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineSet {
    /// Index indicating the order of the set in the routine
    pub index: f64,
    /// The type of set: normal, warmup, dropset, failure
    #[serde(rename = "type")]
    pub set_type: String,
    /// Weight lifted in kilograms
    pub weight_kg: Option<f64>,
    /// Number of reps for the set
    pub reps: Option<f64>,
    /// Range of reps for the set, if applicable
    pub rep_range: Option<RepRange>,
    /// Number of meters for the set
    pub distance_meters: Option<f64>,
    /// Number of seconds for the set
    pub duration_seconds: Option<f64>,
    /// RPE (Relative perceived exertion) value for the set
    pub rpe: Option<f64>,
    /// Custom metric for the set (currently used for floors or steps)
    pub custom_metric: Option<f64>,
}

/// An exercise within a routine (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineExercise {
    /// Index indicating the order of the exercise in the routine
    pub index: f64,
    /// Title of the exercise
    pub title: String,
    /// The rest time in seconds between sets of the exercise
    pub rest_seconds: Option<serde_json::Value>,
    /// Routine notes on the exercise
    pub notes: Option<String>,
    /// The id of the exercise template. Can be used to fetch the exercise template.
    pub exercise_template_id: String,
    /// The id of the superset this exercise belongs to. Null if not in a superset.
    pub supersets_id: Option<f64>,
    /// The sets of this exercise
    pub sets: Vec<RoutineSet>,
}

/// A routine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routine {
    /// The routine ID
    pub id: String,
    /// The routine title
    pub title: String,
    /// The routine folder ID. Null if in the default "My Routines" folder.
    pub folder_id: Option<f64>,
    /// ISO 8601 timestamp of when the routine was last updated
    pub updated_at: String,
    /// ISO 8601 timestamp of when the routine was created
    pub created_at: String,
    /// The exercises in this routine
    pub exercises: Vec<RoutineExercise>,
}

/// Paginated list of routines (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRoutinesResponse {
    /// Current page number
    pub page: i64,
    /// Total number of pages
    pub page_count: i64,
    /// The routines on this page
    pub routines: Vec<Routine>,
}

/// Response wrapper for a single routine (GET /v1/routines/{routineId})
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoutineResponse {
    pub routine: Routine,
}

/// A set within a routine exercise (request)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineSet {
    /// The type of the set: warmup, normal, failure, dropset
    #[serde(rename = "type")]
    pub set_type: SetType,
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
    /// Range of reps for the set, if applicable
    pub rep_range: Option<RepRange>,
}

/// An exercise within a routine (request)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineExercise {
    /// The ID of the exercise template
    pub exercise_template_id: String,
    /// The ID of the superset. Null if not in a superset.
    pub superset_id: Option<i64>,
    /// The rest time in seconds between sets
    pub rest_seconds: Option<i64>,
    /// Additional notes for the exercise
    pub notes: Option<String>,
    /// The sets of this exercise
    pub sets: Vec<PostRoutineSet>,
}

/// Inner body for creating a routine
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineInner {
    /// The title of the routine
    pub title: String,
    /// The folder id to add the routine to. Null inserts into the default "My Routines" folder.
    pub folder_id: Option<f64>,
    /// Additional notes for the routine
    pub notes: Option<String>,
    /// The exercises in the routine
    pub exercises: Vec<PostRoutineExercise>,
}

/// Request body for POST /v1/routines
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutinesRequestBody {
    pub routine: PostRoutineInner,
}

/// Inner body for updating a routine
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutRoutineInner {
    /// The title of the routine
    pub title: String,
    /// Additional notes for the routine
    pub notes: Option<String>,
    /// The exercises in the routine (replaces all existing exercises)
    pub exercises: Vec<PostRoutineExercise>,
}

/// Request body for PUT /v1/routines/{routineId}
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutRoutinesRequestBody {
    pub routine: PutRoutineInner,
}
