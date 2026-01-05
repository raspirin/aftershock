use aftershock_cli::command::Cli;
use aftershock_cli::command::Commands;
use aftershock_cli::command::KindCommands;
use aftershock_cli::requests::*;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        KindCommands::Article { command } => {
            let kind = "post".to_string();
            match command {
                Commands::Add { path } => println!("{}", add(kind, path)),
                Commands::List => println!("{}", list(kind)),
                Commands::View { id } => println!("{}", view(kind, id)),
                Commands::Delete { id } => println!("{}", delete(kind, id)),
                Commands::Update { path, id } => println!("{}", update(kind, path, id)),
                Commands::Publish { id } => println!("{}", publish(kind, id)),
            }
        }
        KindCommands::Page { command } => {
            let kind = "page".to_string();
            match command {
                Commands::Add { path } => println!("{}", add(kind, path)),
                Commands::List => println!("{}", list(kind)),
                Commands::View { id } => println!("{}", view(kind, id)),
                Commands::Delete { id } => println!("{}", delete(kind, id)),
                Commands::Update { path, id } => println!("{}", update(kind, path, id)),
                Commands::Publish { id } => println!("{}", publish(kind, id)),
            }
        }
    }
}
