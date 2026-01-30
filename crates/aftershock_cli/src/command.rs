use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: KindCommands,
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
pub enum KindCommands {
    /// Post operations
    #[command(visible_alias = "post")]
    Article {
        #[command(subcommand)]
        command: Commands,
    },
    /// Page operations
    Page {
        #[command(subcommand)]
        command: Commands,
    },
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
pub enum Commands {
    /// Add a new post
    Add {
        #[arg(short, long)]
        path: String,
    },
    /// List current post
    List,
    /// View a specified post
    View {
        #[arg(short, long)]
        id: String,
    },
    /// Delete a post
    Delete {
        #[arg(short, long)]
        id: String,
    },
    /// Update an existing post
    Update {
        #[arg(short, long)]
        path: String,
        #[arg(short, long)]
        id: String,
    },
    /// Publish a post
    Publish {
        #[arg(short, long)]
        id: String,
    },
}
