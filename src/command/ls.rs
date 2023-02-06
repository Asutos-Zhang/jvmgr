use super::Cmd;

pub struct List();

impl List {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for List {
    fn execute(&self, args: Vec<String>) {
        println!("Executing ls");
    }
}