mod api;
mod cli;
mod models;

use clap::{Parser, Subcommand};
use cli::body_measurements::{BodyMeasurementsArgs, BodyMeasurementsCommand};
use cli::exercise_history::ExerciseHistoryArgs;
use cli::exercise_templates::{ExerciseTemplatesArgs, ExerciseTemplatesCommand};
use cli::routine_folders::{RoutineFoldersArgs, RoutineFoldersCommand};
use cli::routines::{RoutinesArgs, RoutinesCommand};
use cli::user::UserArgs;
use cli::workouts::{WorkoutsArgs, WorkoutsCommand};
use models::body_measurements::{BodyMeasurement, PutBodyMeasurement};
use models::exercise_templates::{
    CreateCustomExerciseInner, CreateCustomExerciseRequestBody, MuscleGroup,
};
use models::routine_folders::{PostRoutineFolderInner, PostRoutineFolderRequestBody};
use models::routines::{
    PostRoutineExercise, PostRoutineInner, PostRoutinesRequestBody, PutRoutineInner,
    PutRoutinesRequestBody,
};
use models::workouts::{PostWorkoutExercise, PostWorkoutInner, PostWorkoutsRequestBody};

/// CLI for the Hevy workout tracker API (https://api.hevyapp.com).
/// Requires a Hevy Pro account. Get your API key at https://hevy.com/settings?developer.
#[derive(Debug, Parser)]
#[command(name = "hevy", version)]
pub struct Cli {
    /// Hevy API key. Can also be set via the HEVY_API_KEY environment variable.
    #[arg(long, env = "HEVY_API_KEY", hide_env_values = true)]
    pub api_key: String,

    /// Print the HTTP/1.1 request to stderr before sending
    #[arg(long, global = true, default_value = "false")]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Log and manage completed workouts
    Workouts(WorkoutsArgs),
    /// Manage workout routines (templates for workouts)
    Routines(RoutinesArgs),
    /// Browse and create exercise templates
    ExerciseTemplates(ExerciseTemplatesArgs),
    /// Manage routine folders for organizing routines
    RoutineFolders(RoutineFoldersArgs),
    /// Get the history of a specific exercise across all workouts
    ExerciseHistory(ExerciseHistoryArgs),
    /// Log and manage body measurements
    BodyMeasurements(BodyMeasurementsArgs),
    /// Get info about the authenticated user
    UserInfo(UserArgs),
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let client = api::Client::new(&cli.api_key, cli.debug);

    let output = match cli.command {
        Commands::Workouts(args) => handle_workouts(&client, args)?,
        Commands::ExerciseTemplates(args) => handle_exercise_templates(&client, args)?,
        Commands::Routines(args) => handle_routines(&client, args)?,
        Commands::RoutineFolders(args) => handle_routine_folders(&client, args)?,
        Commands::ExerciseHistory(args) => handle_exercise_history(&client, args)?,
        Commands::BodyMeasurements(args) => handle_body_measurements(&client, args)?,
        Commands::UserInfo(_) => serde_json::to_string_pretty(&client.get_user_info()?)?,
    };

    println!("{output}");
    Ok(())
}

fn handle_workouts(
    client: &api::Client,
    args: WorkoutsArgs,
) -> Result<String, Box<dyn std::error::Error>> {
    match args.command {
        WorkoutsCommand::List(a) => Ok(serde_json::to_string_pretty(
            &client.list_workouts(a.page, a.page_size)?,
        )?),
        WorkoutsCommand::Count => Ok(serde_json::to_string_pretty(&client.get_workout_count()?)?),
        WorkoutsCommand::Get(a) => Ok(serde_json::to_string_pretty(
            &client.get_workout(&a.workout_id)?,
        )?),
        WorkoutsCommand::Create(a) => {
            let exercises: Vec<PostWorkoutExercise> = serde_json::from_str(&a.exercises)?;
            let body = PostWorkoutsRequestBody {
                workout: PostWorkoutInner {
                    title: a.title,
                    description: a.description,
                    start_time: a.start_time,
                    end_time: a.end_time,
                    is_private: a.is_private,
                    exercises,
                },
            };
            Ok(serde_json::to_string_pretty(
                &client.create_workout(&body)?,
            )?)
        }
        WorkoutsCommand::Update(a) => {
            let exercises: Vec<PostWorkoutExercise> = serde_json::from_str(&a.exercises)?;
            let body = PostWorkoutsRequestBody {
                workout: PostWorkoutInner {
                    title: a.title,
                    description: a.description,
                    start_time: a.start_time,
                    end_time: a.end_time,
                    is_private: a.is_private,
                    exercises,
                },
            };
            Ok(serde_json::to_string_pretty(
                &client.update_workout(&a.workout_id, &body)?,
            )?)
        }
        WorkoutsCommand::Events(a) => Ok(serde_json::to_string_pretty(
            &client.get_workout_events(a.page, a.page_size, &a.since)?,
        )?),
    }
}

fn handle_exercise_templates(
    client: &api::Client,
    args: ExerciseTemplatesArgs,
) -> Result<String, Box<dyn std::error::Error>> {
    match args.command {
        ExerciseTemplatesCommand::List(a) => Ok(serde_json::to_string_pretty(
            &client.list_exercise_templates(a.page, a.page_size)?,
        )?),
        ExerciseTemplatesCommand::Get(a) => Ok(serde_json::to_string_pretty(
            &client.get_exercise_template(&a.exercise_template_id)?,
        )?),
        ExerciseTemplatesCommand::Create(a) => {
            let body = CreateCustomExerciseRequestBody {
                exercise: CreateCustomExerciseInner {
                    title: a.title,
                    exercise_type: serde_json::from_value(serde_json::Value::String(a.exercise_type))
                        .map_err(|_| "invalid exercise_type; valid values: weight_reps, reps_only, bodyweight_reps, bodyweight_assisted_reps, duration, weight_duration, distance_duration, short_distance_weight")?,
                    equipment_category: serde_json::from_value(serde_json::Value::String(a.equipment_category))
                        .map_err(|_| "invalid equipment_category; valid values: none, barbell, dumbbell, kettlebell, machine, plate, resistance_band, suspension, other")?,
                    muscle_group: serde_json::from_value(serde_json::Value::String(a.muscle_group))
                        .map_err(|_| "invalid muscle_group; valid values: abdominals, shoulders, biceps, triceps, forearms, quadriceps, hamstrings, calves, glutes, abductors, adductors, lats, upper_back, traps, lower_back, chest, cardio, neck, full_body, other")?,
                    other_muscles: a.other_muscles
                        .iter()
                        .map(|m| serde_json::from_value::<MuscleGroup>(serde_json::Value::String(m.clone()))
                            .map_err(|_| format!("invalid muscle group: {m}")))
                        .collect::<std::result::Result<Vec<MuscleGroup>, _>>()?,
                },
            };
            Ok(serde_json::to_string_pretty(
                &client.create_exercise_template(&body)?,
            )?)
        }
    }
}

fn handle_routines(
    client: &api::Client,
    args: RoutinesArgs,
) -> Result<String, Box<dyn std::error::Error>> {
    match args.command {
        RoutinesCommand::List(a) => Ok(serde_json::to_string_pretty(
            &client.list_routines(a.page, a.page_size)?,
        )?),
        RoutinesCommand::Get(a) => Ok(serde_json::to_string_pretty(
            &client.get_routine(&a.routine_id)?,
        )?),
        RoutinesCommand::Create(a) => {
            let exercises: Vec<PostRoutineExercise> = serde_json::from_str(&a.exercises)?;
            let body = PostRoutinesRequestBody {
                routine: PostRoutineInner {
                    title: a.title,
                    folder_id: a.folder_id,
                    notes: a.notes,
                    exercises,
                },
            };
            Ok(serde_json::to_string_pretty(
                &client.create_routine(&body)?,
            )?)
        }
        RoutinesCommand::Update(a) => {
            let exercises: Vec<PostRoutineExercise> = serde_json::from_str(&a.exercises)?;
            let body = PutRoutinesRequestBody {
                routine: PutRoutineInner {
                    title: a.title,
                    notes: a.notes,
                    exercises,
                },
            };
            Ok(serde_json::to_string_pretty(
                &client.update_routine(&a.routine_id, &body)?,
            )?)
        }
    }
}

fn handle_routine_folders(
    client: &api::Client,
    args: RoutineFoldersArgs,
) -> Result<String, Box<dyn std::error::Error>> {
    match args.command {
        RoutineFoldersCommand::List(a) => Ok(serde_json::to_string_pretty(
            &client.list_routine_folders(a.page, a.page_size)?,
        )?),
        RoutineFoldersCommand::Get(a) => Ok(serde_json::to_string_pretty(
            &client.get_routine_folder(&a.folder_id)?,
        )?),
        RoutineFoldersCommand::Create(a) => {
            let body = PostRoutineFolderRequestBody {
                routine_folder: PostRoutineFolderInner { title: a.title },
            };
            Ok(serde_json::to_string_pretty(
                &client.create_routine_folder(&body)?,
            )?)
        }
    }
}

fn handle_exercise_history(
    client: &api::Client,
    args: ExerciseHistoryArgs,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp = client.get_exercise_history(
        &args.exercise_template_id,
        args.start_date.as_deref(),
        args.end_date.as_deref(),
    )?;
    Ok(serde_json::to_string_pretty(&resp)?)
}

fn handle_body_measurements(
    client: &api::Client,
    args: BodyMeasurementsArgs,
) -> Result<String, Box<dyn std::error::Error>> {
    match args.command {
        BodyMeasurementsCommand::List(a) => Ok(serde_json::to_string_pretty(
            &client.list_body_measurements(a.page, a.page_size)?,
        )?),
        BodyMeasurementsCommand::Get(a) => Ok(serde_json::to_string_pretty(
            &client.get_body_measurement(&a.date)?,
        )?),
        BodyMeasurementsCommand::Create(a) => {
            let body = BodyMeasurement {
                date: a.date,
                weight_kg: a.weight_kg,
                lean_mass_kg: a.lean_mass_kg,
                fat_percent: a.fat_percent,
                neck_cm: a.neck_cm,
                shoulder_cm: a.shoulder_cm,
                chest_cm: a.chest_cm,
                left_bicep_cm: a.left_bicep_cm,
                right_bicep_cm: a.right_bicep_cm,
                left_forearm_cm: a.left_forearm_cm,
                right_forearm_cm: a.right_forearm_cm,
                abdomen: a.abdomen,
                waist: a.waist,
                hips: a.hips,
                left_thigh: a.left_thigh,
                right_thigh: a.right_thigh,
                left_calf: a.left_calf,
                right_calf: a.right_calf,
            };
            client.create_body_measurement(&body)?;
            Ok(serde_json::to_string_pretty(
                &serde_json::json!({ "success": true }),
            )?)
        }
        BodyMeasurementsCommand::Update(a) => {
            let body = PutBodyMeasurement {
                weight_kg: a.weight_kg,
                lean_mass_kg: a.lean_mass_kg,
                fat_percent: a.fat_percent,
                neck_cm: a.neck_cm,
                shoulder_cm: a.shoulder_cm,
                chest_cm: a.chest_cm,
                left_bicep_cm: a.left_bicep_cm,
                right_bicep_cm: a.right_bicep_cm,
                left_forearm_cm: a.left_forearm_cm,
                right_forearm_cm: a.right_forearm_cm,
                abdomen: a.abdomen,
                waist: a.waist,
                hips: a.hips,
                left_thigh: a.left_thigh,
                right_thigh: a.right_thigh,
                left_calf: a.left_calf,
                right_calf: a.right_calf,
            };
            client.update_body_measurement(&a.date, &body)?;
            Ok(serde_json::to_string_pretty(
                &serde_json::json!({ "success": true }),
            )?)
        }
    }
}
