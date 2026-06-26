use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct RoutinesArgs {
    #[command(subcommand)]
    pub command: RoutinesCommand,
}

#[derive(Debug, Subcommand)]
pub enum RoutinesCommand {
    /// Get a paginated list of routines
    List(ListArgs),
    /// Get a routine by its ID
    Get(GetArgs),
    /// Create a new routine
    Create(CreateArgs),
    /// Update an existing routine (replaces all exercises)
    Update(UpdateArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Page number (must be 1 or greater)
    #[arg(long, default_value = "1")]
    pub page: i64,

    /// Number of items per page (max 10)
    #[arg(long, default_value = "5")]
    pub page_size: i64,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// The ID of the routine
    pub routine_id: String,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The title of the routine
    #[arg(long)]
    pub title: String,

    /// The folder ID to place the routine in. Omit to use the default "My Routines" folder.
    #[arg(long)]
    pub folder_id: Option<f64>,

    /// Additional notes for the routine
    #[arg(long)]
    pub notes: Option<String>,

    /// Exercises as a JSON array. Each element: { "exercise_template_id": string, "superset_id": int|null, "rest_seconds": int|null, "notes": string|null, "sets": [{ "type": "warmup"|"normal"|"failure"|"dropset", "weight_kg": number|null, "reps": int|null, "distance_meters": int|null, "duration_seconds": int|null, "custom_metric": number|null, "rep_range": { "start": number, "end": number }|null }] }
    #[arg(long)]
    pub exercises: String,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// The ID of the routine to update
    pub routine_id: String,

    /// The title of the routine
    #[arg(long)]
    pub title: String,

    /// Additional notes for the routine
    #[arg(long)]
    pub notes: Option<String>,

    /// Exercises as a JSON array (replaces all existing exercises). Each element: { "exercise_template_id": string, "superset_id": int|null, "rest_seconds": int|null, "notes": string|null, "sets": [{ "type": "warmup"|"normal"|"failure"|"dropset", "weight_kg": number|null, "reps": int|null, "distance_meters": int|null, "duration_seconds": int|null, "custom_metric": number|null, "rep_range": { "start": number, "end": number }|null }] }
    #[arg(long)]
    pub exercises: String,
}
