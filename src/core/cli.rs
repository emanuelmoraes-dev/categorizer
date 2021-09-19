use crate::components::dft::cli::Cli;
use crate::ports::cli::Info;

pub fn run() {
    let cli = Cli::new();
    cli.wellcome();
}
