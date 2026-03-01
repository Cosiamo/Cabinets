use crate::{
    args::{Commands, Subcommands},
    spawn::spawn_gui,
};
use clap::Parser;
use std::path::Path;
use std::{error::Error, io};

pub mod args;
pub mod spawn;

fn path_or_current(path: Option<&str>) -> &Path {
    path.map(Path::new).unwrap_or_else(|| Path::new("."))
}

fn print_items(items: Vec<String>) {
    for item in items {
        println!("{}", item);
    }
}

fn run_default_listing() {
    let path = Path::new(".");
    let operations: [io::Result<Vec<String>>; 4] = [
        app_core::filesystem::list_file_names(path),
        app_core::filesystem::list_folder_names(path),
        app_core::filesystem::list_hidden_file_names(path),
        app_core::filesystem::list_hidden_folder_names(path),
    ];

    let mut all_items = Vec::new();
    for op in operations {
        match op {
            Ok(items) => all_items.extend(items),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    print_items(all_items);
}

fn run_subcommand(cmd: &Subcommands) -> io::Result<()> {
    match cmd {
        Subcommands::Files(args) => {
            app_core::filesystem::list_file_names(path_or_current(args.path.as_deref()))
                .map(print_items)
        }
        Subcommands::Folders(args) => {
            app_core::filesystem::list_folder_names(path_or_current(args.path.as_deref()))
                .map(print_items)
        }
        Subcommands::HiddenFiles(args) => {
            app_core::filesystem::list_hidden_file_names(path_or_current(args.path.as_deref()))
                .map(print_items)
        }
        Subcommands::HiddenFolders(args) => {
            app_core::filesystem::list_hidden_folder_names(path_or_current(args.path.as_deref()))
                .map(print_items)
        }
        Subcommands::OpenFile(args) => app_core::open_file(path_or_current(args.path.as_deref())),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw_args: Vec<String> = std::env::args().collect();

    match raw_args.as_slice() {
        [_, arg] if arg == "." => match spawn_gui(raw_args) {
            Ok(_status) => {}
            Err(e) => {
                eprintln!("Failed to wait for instance: {}", e);
                std::process::exit(1);
            }
        },
        [_] => run_default_listing(),
        _ => {
            let commands = Commands::parse();
            match run_subcommand(&commands.cmd) {
                Ok(()) => {}
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    }

    Ok(())
}
