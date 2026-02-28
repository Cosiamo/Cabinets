use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Author Name", version, about)]
pub struct Commands {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {#[command(about = "List files in a directory")]
    Files(DirArgs),
    #[command(about = "List folders in a directory")]
    Folders(DirArgs),
    #[command(about = "List hidden files in a directory")]
    HiddenFiles(DirArgs),
    #[command(about = "List hidden folders in a directory")]
    HiddenFolders(DirArgs),
}

#[derive(Parser, Debug)]
pub struct DirArgs {
    #[arg(short = 'p', long = "path")]
    pub path: Option<String>,
}