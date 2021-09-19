use crate::ports::cli::Info;
use crate::components::dft::cli::Cli;

pub fn run () {
    let cli = Cli::new();
    cli.wellcome();
}