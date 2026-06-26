use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ExerciseTemplatesArgs {
    #[command(subcommand)]
    pub command: ExerciseTemplatesCommand,
}

#[derive(Debug, Subcommand)]
pub enum ExerciseTemplatesCommand {
    /// Get a paginated list of exercise templates available on the account
    List(ListArgs),
    /// Get a single exercise template by ID
    Get(GetArgs),
    /// Create a new custom exercise template
    Create(CreateArgs),
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Page number (must be 1 or greater)
    #[arg(long, default_value = "1")]
    pub page: i64,

    /// Number of items per page (max 100)
    #[arg(long, default_value = "5")]
    pub page_size: i64,
}

#[derive(Debug, Args)]
pub struct GetArgs {
    /// The ID of the exercise template
    pub exercise_template_id: String,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The title of the exercise template (e.g. "Bench Press")
    #[arg(long)]
    pub title: String,

    /// The tracking type of the exercise. One of: weight_reps, reps_only, bodyweight_reps, bodyweight_assisted_reps, duration, weight_duration, distance_duration, short_distance_weight
    #[arg(long)]
    pub exercise_type: String,

    /// The equipment category. One of: none, barbell, dumbbell, kettlebell, machine, plate, resistance_band, suspension, other
    #[arg(long)]
    pub equipment_category: String,

    /// The primary muscle group. One of: abdominals, shoulders, biceps, triceps, forearms, quadriceps, hamstrings, calves, glutes, abductors, adductors, lats, upper_back, traps, lower_back, chest, cardio, neck, full_body, other
    #[arg(long)]
    pub muscle_group: String,

    /// Additional muscle groups as a comma-separated list (e.g. "biceps,triceps"). Same values as --muscle-group.
    #[arg(long, value_delimiter = ',')]
    pub other_muscles: Vec<String>,
}
