use serde::{Deserialize, Serialize};

/// The tracking type of an exercise
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseType {
    /// Tracks weight and reps (e.g. Bench Press)
    WeightReps,
    /// Tracks reps only (e.g. Pull-ups without weight)
    RepsOnly,
    /// Tracks bodyweight reps (e.g. Push-ups)
    BodyweightReps,
    /// Tracks bodyweight-assisted reps (e.g. Assisted Pull-ups)
    BodyweightAssistedReps,
    /// Tracks duration only (e.g. Plank)
    Duration,
    /// Tracks weight and duration (e.g. Farmer's Carry)
    WeightDuration,
    /// Tracks distance and duration (e.g. Running)
    DistanceDuration,
    /// Tracks short distance and weight (e.g. Sled Push)
    ShortDistanceWeight,
}

/// Primary or secondary muscle group targeted by an exercise
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MuscleGroup {
    Abdominals,
    Shoulders,
    Biceps,
    Triceps,
    Forearms,
    Quadriceps,
    Hamstrings,
    Calves,
    Glutes,
    Abductors,
    Adductors,
    Lats,
    UpperBack,
    Traps,
    LowerBack,
    Chest,
    Cardio,
    Neck,
    FullBody,
    Other,
}

/// Equipment category for an exercise
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EquipmentCategory {
    None,
    Barbell,
    Dumbbell,
    Kettlebell,
    Machine,
    Plate,
    ResistanceBand,
    Suspension,
    Other,
}

/// An exercise template (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseTemplate {
    /// The exercise template ID
    pub id: String,
    /// The exercise title (e.g. "Bench Press (Barbell)")
    pub title: String,
    /// The exercise tracking type (e.g. weight_reps, duration, distance_duration)
    #[serde(rename = "type")]
    pub exercise_type: String,
    /// The primary muscle group targeted by this exercise
    pub primary_muscle_group: String,
    /// Secondary muscle groups targeted by this exercise
    pub secondary_muscle_groups: Vec<String>,
    /// Whether this is a custom (user-created) exercise
    pub is_custom: bool,
}

/// Paginated list of exercise templates (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExerciseTemplatesResponse {
    /// Current page number
    pub page: i64,
    /// Total number of pages
    pub page_count: i64,
    /// The exercise templates on this page
    pub exercise_templates: Vec<ExerciseTemplate>,
}

/// Request body for POST /v1/exercise_templates
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomExerciseRequestBody {
    pub exercise: CreateCustomExerciseInner,
}

/// Inner body for creating a custom exercise template
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomExerciseInner {
    /// The title of the exercise template
    pub title: String,
    /// The tracking type of the exercise: weight_reps, reps_only, bodyweight_reps, bodyweight_assisted_reps, duration, weight_duration, distance_duration, short_distance_weight
    pub exercise_type: ExerciseType,
    /// The equipment category: none, barbell, dumbbell, kettlebell, machine, plate, resistance_band, suspension, other
    pub equipment_category: EquipmentCategory,
    /// The primary muscle group: abdominals, shoulders, biceps, triceps, forearms, quadriceps, hamstrings, calves, glutes, abductors, adductors, lats, upper_back, traps, lower_back, chest, cardio, neck, full_body, other
    pub muscle_group: MuscleGroup,
    /// Additional muscle groups targeted by this exercise
    pub other_muscles: Vec<MuscleGroup>,
}

/// Response for POST /v1/exercise_templates
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomExerciseResponse {
    /// The ID of the newly created exercise template
    pub id: i64,
}
