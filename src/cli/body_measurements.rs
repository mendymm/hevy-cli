use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct BodyMeasurementsArgs {
    #[command(subcommand)]
    pub command: BodyMeasurementsCommand,
}

#[derive(Debug, Subcommand)]
pub enum BodyMeasurementsCommand {
    /// Get a paginated list of body measurements, ordered newest first
    List(ListArgs),
    /// Get a single body measurement by date
    Get(GetArgs),
    /// Create a body measurement entry for a given date (returns 409 if an entry already exists)
    Create(MeasurementFields),
    /// Update an existing body measurement for a given date (all fields overwritten; omitted fields set to null)
    Update(UpdateArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Page number (must be 1 or greater)
    #[arg(long, default_value = "1")]
    pub page: i64,

    /// Number of items per page (max 10)
    #[arg(long, default_value = "10")]
    pub page_size: i64,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// The date of the measurement (YYYY-MM-DD, e.g. "2024-08-14")
    pub date: String,
}

/// Measurement fields shared by create and update
#[derive(Debug, Args)]
pub struct MeasurementFields {
    /// The date of the measurement (YYYY-MM-DD, e.g. "2024-08-14")
    #[arg(long)]
    pub date: String,

    /// Body weight in kilograms
    #[arg(long)]
    pub weight_kg: Option<f64>,

    /// Lean body mass in kilograms
    #[arg(long)]
    pub lean_mass_kg: Option<f64>,

    /// Body fat percentage
    #[arg(long)]
    pub fat_percent: Option<f64>,

    /// Neck circumference in centimeters
    #[arg(long)]
    pub neck_cm: Option<f64>,

    /// Shoulder circumference in centimeters
    #[arg(long)]
    pub shoulder_cm: Option<f64>,

    /// Chest circumference in centimeters
    #[arg(long)]
    pub chest_cm: Option<f64>,

    /// Left bicep circumference in centimeters
    #[arg(long)]
    pub left_bicep_cm: Option<f64>,

    /// Right bicep circumference in centimeters
    #[arg(long)]
    pub right_bicep_cm: Option<f64>,

    /// Left forearm circumference in centimeters
    #[arg(long)]
    pub left_forearm_cm: Option<f64>,

    /// Right forearm circumference in centimeters
    #[arg(long)]
    pub right_forearm_cm: Option<f64>,

    /// Abdomen circumference in centimeters
    #[arg(long)]
    pub abdomen: Option<f64>,

    /// Waist circumference in centimeters
    #[arg(long)]
    pub waist: Option<f64>,

    /// Hip circumference in centimeters
    #[arg(long)]
    pub hips: Option<f64>,

    /// Left thigh circumference in centimeters
    #[arg(long)]
    pub left_thigh: Option<f64>,

    /// Right thigh circumference in centimeters
    #[arg(long)]
    pub right_thigh: Option<f64>,

    /// Left calf circumference in centimeters
    #[arg(long)]
    pub left_calf: Option<f64>,

    /// Right calf circumference in centimeters
    #[arg(long)]
    pub right_calf: Option<f64>,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// The date of the measurement to update (YYYY-MM-DD, e.g. "2024-08-14")
    pub date: String,

    /// Body weight in kilograms
    #[arg(long)]
    pub weight_kg: Option<f64>,

    /// Lean body mass in kilograms
    #[arg(long)]
    pub lean_mass_kg: Option<f64>,

    /// Body fat percentage
    #[arg(long)]
    pub fat_percent: Option<f64>,

    /// Neck circumference in centimeters
    #[arg(long)]
    pub neck_cm: Option<f64>,

    /// Shoulder circumference in centimeters
    #[arg(long)]
    pub shoulder_cm: Option<f64>,

    /// Chest circumference in centimeters
    #[arg(long)]
    pub chest_cm: Option<f64>,

    /// Left bicep circumference in centimeters
    #[arg(long)]
    pub left_bicep_cm: Option<f64>,

    /// Right bicep circumference in centimeters
    #[arg(long)]
    pub right_bicep_cm: Option<f64>,

    /// Left forearm circumference in centimeters
    #[arg(long)]
    pub left_forearm_cm: Option<f64>,

    /// Right forearm circumference in centimeters
    #[arg(long)]
    pub right_forearm_cm: Option<f64>,

    /// Abdomen circumference in centimeters
    #[arg(long)]
    pub abdomen: Option<f64>,

    /// Waist circumference in centimeters
    #[arg(long)]
    pub waist: Option<f64>,

    /// Hip circumference in centimeters
    #[arg(long)]
    pub hips: Option<f64>,

    /// Left thigh circumference in centimeters
    #[arg(long)]
    pub left_thigh: Option<f64>,

    /// Right thigh circumference in centimeters
    #[arg(long)]
    pub right_thigh: Option<f64>,

    /// Left calf circumference in centimeters
    #[arg(long)]
    pub left_calf: Option<f64>,

    /// Right calf circumference in centimeters
    #[arg(long)]
    pub right_calf: Option<f64>,
}
