use crate::ports::cli;

pub struct Cli {}

impl cli::Info for Cli {
    fn new() -> Self {
        Cli{}
    }

    fn wellcome(&self){
        println!("Wellcome to Categorizer CLI!");
    }
}