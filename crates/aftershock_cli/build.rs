use std::env;

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use dotenvy::dotenv;

include!("src/command.rs");

fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let outdir = match env::var_os("COMPLETION_OUTDIR") {
        None => match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,}
        Some(outdir) => outdir
    };
    let mut cli = Cli::command();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cli, "aftershock_cli", &outdir)?;
    }
    println!("cargo:warning=completion file is generated: {outdir:?}");

    Ok(())
}