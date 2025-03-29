use aftershock_cli::command::Cli;
use aftershock_cli::command::Commands;
use aftershock_cli::requests::*;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let kind = cli.kind;
    match cli.command {
        Commands::Add { path } => println!("{}", add(kind, path)),
        Commands::List => println!("{}", list(kind)),
        Commands::View { id } => println!("{}", view(kind, id)),
        Commands::Delete { id } => println!("{}", delete(kind, id)),
        Commands::Update { path, id } => todo!(),
        Commands::Publish { id } => println!("{}", publish(kind, id)),
    }
}
