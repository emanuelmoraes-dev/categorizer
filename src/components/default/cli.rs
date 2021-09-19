use crate::ports::cli;

impl cli::Info for cli::Cli {
    fn new() -> Self {
        cli::Cli{}
    }

    fn wellcome(&self){
        println!("Wellcome to Categorizer CLI!");
    }
}