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
    /// Add a new content
    Add {
        /// The path to the file
        path: String,
    },
    /// List current contents
    #[command(visible_alias = "ls")]
    List,
    /// View a specified content
    View {
        /// The uid of the content
        id: String,
    },
    /// Delete a specified content
    #[command(visible_alias = "del")]
    Delete {
        /// The uid of the content
        id: String,
    },
    /// Update an existing content
    #[command(visible_alias = "u")]
    Update {
        /// The path to the updated source file
        path: String,
        /// The uid of the content
        id: String,
    },
    /// Publish a content
    #[command(visible_alias = "pub")]
    Publish {
        /// The uid of the content
        id: String,
    },
}
