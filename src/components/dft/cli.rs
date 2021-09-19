use crate::ports::cli;

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        Cli {}
    }
}

impl cli::Info for Cli {
    fn wellcome(&self) {
        println!("Wellcome to Categorizer CLI!");
    }
}
