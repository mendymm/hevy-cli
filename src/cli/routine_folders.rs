use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct RoutineFoldersArgs {
    #[command(subcommand)]
    pub command: RoutineFoldersCommand,
}

#[derive(Debug, Subcommand)]
pub enum RoutineFoldersCommand {
    /// Get a paginated list of routine folders
    List(ListArgs),
    /// Get a single routine folder by ID
    Get(GetArgs),
    /// Create a new routine folder (inserted at index 0; existing folders shift down)
    Create(CreateArgs),
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
    /// The ID of the routine folder
    pub folder_id: String,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The title of the routine folder
    #[arg(long)]
    pub title: String,
}
