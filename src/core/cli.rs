use crate::ports::cli::*;

pub fn run () {
    let cli: Cli = Cli::new();
    cli.wellcome();
}