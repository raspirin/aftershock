use aftershock_cli::command::Cli;
use aftershock_cli::command::Commands;
use aftershock_cli::requests::*;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { path } => println!("{}", add(path)),
        Commands::List => println!("{}", list()),
        Commands::View { id } => println!("{}", view(id)),
        Commands::Delete { id } => println!("{}", delete(id)),
        Commands::Update { path, id } => todo!(),
        Commands::Publish { id } => println!("{}", publish(id)),
    }
}
