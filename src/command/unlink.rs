use super::Cmd;

pub struct Unlink();

impl Unlink {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Unlink {
    fn execute(&self, args: Vec<String>) {
        println!("Executing Unlink,  args:{:?}", args);
    }
}