use crate::{
    args::{Commands, Subcommands},
    spawn::spawn_gui,
};
use clap::Parser;
use std::error::Error;

pub mod args;
pub mod spawn;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let should_spawn_gui = args.len() <= 1 || (args.len() == 2 && args[1] == ".");

    if !should_spawn_gui {
        let commands = Commands::parse();

        let (op, a, b): (fn(f64, f64) -> f64, _, _) = match &commands.cmd {
            Subcommands::Add(args) => (app_core::equations::add, args.number_a, args.number_b),
            Subcommands::Subtract(args) => {
                (app_core::equations::subtract, args.number_a, args.number_b)
            }
            Subcommands::Multiply(args) => {
                (app_core::equations::multiply, args.number_a, args.number_b)
            }
            Subcommands::Divide(args) => {
                (app_core::equations::divide, args.number_a, args.number_b)
            }
        };

        println!("{}", op(a, b));
    } else {
        match spawn_gui(args) {
            Ok(run) => run,
            Err(e) => {
                eprintln!("Failed to wait for instance: {}", e);
                std::process::exit(1);
            }
        };
    }

    Ok(())
}
