use serde::{Deserialize, Serialize};

/// A body measurement entry for a specific date
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMeasurement {
    /// The date of the measurement (YYYY-MM-DD, e.g. "2024-08-14")
    pub date: String,
    /// Body weight in kilograms
    pub weight_kg: Option<f64>,
    /// Lean body mass in kilograms
    pub lean_mass_kg: Option<f64>,
    /// Body fat percentage
    pub fat_percent: Option<f64>,
    /// Neck circumference in centimeters
    pub neck_cm: Option<f64>,
    /// Shoulder circumference in centimeters
    pub shoulder_cm: Option<f64>,
    /// Chest circumference in centimeters
    pub chest_cm: Option<f64>,
    /// Left bicep circumference in centimeters
    pub left_bicep_cm: Option<f64>,
    /// Right bicep circumference in centimeters
    pub right_bicep_cm: Option<f64>,
    /// Left forearm circumference in centimeters
    pub left_forearm_cm: Option<f64>,
    /// Right forearm circumference in centimeters
    pub right_forearm_cm: Option<f64>,
    /// Abdomen circumference in centimeters
    pub abdomen: Option<f64>,
    /// Waist circumference in centimeters
    pub waist: Option<f64>,
    /// Hip circumference in centimeters
    pub hips: Option<f64>,
    /// Left thigh circumference in centimeters
    pub left_thigh: Option<f64>,
    /// Right thigh circumference in centimeters
    pub right_thigh: Option<f64>,
    /// Left calf circumference in centimeters
    pub left_calf: Option<f64>,
    /// Right calf circumference in centimeters
    pub right_calf: Option<f64>,
}

/// Paginated list of body measurements (response)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBodyMeasurementsResponse {
    /// Current page number
    pub page: i64,
    /// Total number of pages
    pub page_count: i64,
    /// The body measurements on this page, ordered newest first
    pub body_measurements: Vec<BodyMeasurement>,
}

/// Request body for PUT /v1/body_measurements/{date}.
/// All fields are overwritten; omitted fields are set to null.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBodyMeasurement {
    /// Body weight in kilograms
    pub weight_kg: Option<f64>,
    /// Lean body mass in kilograms
    pub lean_mass_kg: Option<f64>,
    /// Body fat percentage
    pub fat_percent: Option<f64>,
    /// Neck circumference in centimeters
    pub neck_cm: Option<f64>,
    /// Shoulder circumference in centimeters
    pub shoulder_cm: Option<f64>,
    /// Chest circumference in centimeters
    pub chest_cm: Option<f64>,
    /// Left bicep circumference in centimeters
    pub left_bicep_cm: Option<f64>,
    /// Right bicep circumference in centimeters
    pub right_bicep_cm: Option<f64>,
    /// Left forearm circumference in centimeters
    pub left_forearm_cm: Option<f64>,
    /// Right forearm circumference in centimeters
    pub right_forearm_cm: Option<f64>,
    /// Abdomen circumference in centimeters
    pub abdomen: Option<f64>,
    /// Waist circumference in centimeters
    pub waist: Option<f64>,
    /// Hip circumference in centimeters
    pub hips: Option<f64>,
    /// Left thigh circumference in centimeters
    pub left_thigh: Option<f64>,
    /// Right thigh circumference in centimeters
    pub right_thigh: Option<f64>,
    /// Left calf circumference in centimeters
    pub left_calf: Option<f64>,
    /// Right calf circumference in centimeters
    pub right_calf: Option<f64>,
}
