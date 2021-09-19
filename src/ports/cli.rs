pub struct Cli {}

pub trait Info {
    fn new() -> Self;
    fn wellcome(&self);
}
