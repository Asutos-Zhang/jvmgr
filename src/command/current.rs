use super::Cmd;
pub struct Current;

impl Current {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Current {
    fn execute(&self, args: Vec<String>) {
        println!("Executing Current");
    }
}
