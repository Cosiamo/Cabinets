use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Author Name", version, about)]
pub struct Commands {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    #[command(about = "Add two numbers together")]
    Add(Args),
    #[command(about = "Subtract two numbers")]
    Subtract(Args),
    #[command(about = "Multiply two numbers")]
    Multiply(Args),
    #[command(about = "Divide two numbers")]
    Divide(Args),
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'a', long = "left")]
    pub number_a: f64,
    #[arg(short = 'b', long = "right")]
    pub number_b: f64,
}
