use crate::{
    args::{Commands, Subcommands},
    spawn::spawn_gui,
};
use clap::Parser;
use std::{error::Error, io};
use std::path::Path;

pub mod args;
pub mod spawn;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let should_spawn_gui = args.len() <= 1 || (args.len() == 2 && args[1] == ".");

    if !should_spawn_gui {
        let commands = Commands::parse();

        let op: io::Result<Vec<String>> = match &commands.cmd {
            Subcommands::Files(dir_args) => {
                let path = dir_args.path.as_deref().map(Path::new).unwrap_or_else(|| Path::new("."));
                app_core::filesystem::list_file_names(path)
            },
            Subcommands::Folders(dir_args) => {
                let path = dir_args.path.as_deref().map(Path::new).unwrap_or_else(|| Path::new("."));
                app_core::filesystem::list_folder_names(path)
            },
            Subcommands::HiddenFiles(dir_args) => {
                let path = dir_args.path.as_deref().map(Path::new).unwrap_or_else(|| Path::new("."));
                app_core::filesystem::list_hidden_file_names(path)
            },
            Subcommands::HiddenFolders(dir_args) => {
                let path = dir_args.path.as_deref().map(Path::new).unwrap_or_else(|| Path::new("."));
                app_core::filesystem::list_hidden_folder_names(path)
            },
        };

        match op {
            Ok(res) => {
                for item in res {
                    println!("{}", item);
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
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
