use clap::Args;

#[derive(Debug, Args)]
pub struct ExerciseHistoryArgs {
    /// The ID of the exercise template to retrieve history for
    pub exercise_template_id: String,

    /// Filter to entries on or after this date (ISO 8601 format, e.g. "2024-01-01T00:00:00Z")
    #[arg(long)]
    pub start_date: Option<String>,

    /// Filter to entries on or before this date (ISO 8601 format, e.g. "2024-12-31T23:59:59Z")
    #[arg(long)]
    pub end_date: Option<String>,
}
