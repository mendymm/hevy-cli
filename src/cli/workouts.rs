use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct WorkoutsArgs {
    #[command(subcommand)]
    pub command: WorkoutsCommand,
}

#[derive(Debug, Subcommand)]
pub enum WorkoutsCommand {
    /// Get a paginated list of workouts
    List(ListArgs),
    /// Get the total number of workouts on the account
    Count,
    /// Get a single workout's complete details by ID
    Get(GetArgs),
    /// Create a new workout
    Create(CreateArgs),
    /// Update an existing workout
    Update(UpdateArgs),
    /// Get workout events (updates or deletes) since a given date, for syncing a local cache
    Events(EventsArgs),
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
    /// The ID of the workout
    pub workout_id: String,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The title of the workout
    #[arg(long)]
    pub title: String,

    /// A description of the workout
    #[arg(long)]
    pub description: Option<String>,

    /// ISO 8601 timestamp of when the workout started (e.g. "2024-08-14T12:00:00Z")
    #[arg(long)]
    pub start_time: String,

    /// ISO 8601 timestamp of when the workout ended (e.g. "2024-08-14T13:00:00Z")
    #[arg(long)]
    pub end_time: String,

    /// Mark the workout as private
    #[arg(long)]
    pub is_private: Option<bool>,

    /// Exercises as a JSON array. Each element: { "exercise_template_id": string, "superset_id": int|null, "notes": string|null, "sets": [{ "type": "warmup"|"normal"|"failure"|"dropset", "weight_kg": number|null, "reps": int|null, "distance_meters": int|null, "duration_seconds": int|null, "custom_metric": number|null, "rpe": 6|7|7.5|8|8.5|9|9.5|10|null }] }
    #[arg(long)]
    pub exercises: String,
}

#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// The ID of the workout to update
    pub workout_id: String,

    /// The title of the workout
    #[arg(long)]
    pub title: String,

    /// A description of the workout
    #[arg(long)]
    pub description: Option<String>,

    /// ISO 8601 timestamp of when the workout started (e.g. "2024-08-14T12:00:00Z")
    #[arg(long)]
    pub start_time: String,

    /// ISO 8601 timestamp of when the workout ended (e.g. "2024-08-14T13:00:00Z")
    #[arg(long)]
    pub end_time: String,

    /// Mark the workout as private
    #[arg(long)]
    pub is_private: Option<bool>,

    /// Exercises as a JSON array (replaces all existing exercises). Each element: { "exercise_template_id": string, "superset_id": int|null, "notes": string|null, "sets": [{ "type": "warmup"|"normal"|"failure"|"dropset", "weight_kg": number|null, "reps": int|null, "distance_meters": int|null, "duration_seconds": int|null, "custom_metric": number|null, "rpe": 6|7|7.5|8|8.5|9|9.5|10|null }] }
    #[arg(long)]
    pub exercises: String,
}

#[derive(Debug, Args)]
pub struct EventsArgs {
    /// Page number (must be 1 or greater)
    #[arg(long, default_value = "1")]
    pub page: i64,

    /// Number of items per page (max 10)
    #[arg(long, default_value = "5")]
    pub page_size: i64,

    /// Return only events after this ISO 8601 timestamp (e.g. "2024-01-01T00:00:00Z"). Defaults to all events.
    #[arg(long, default_value = "1970-01-01T00:00:00Z")]
    pub since: String,
}
