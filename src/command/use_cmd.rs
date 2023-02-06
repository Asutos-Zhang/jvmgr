use super::Cmd;

pub struct Use();

impl Use {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Use {
    fn execute(&self, args: Vec<String>) {
        println!("Executing Use,  args:{:?}", args);
    }
}
