use super::Cmd;

pub struct Link();

impl Link {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Link {
    fn execute(&self, args: Vec<String>) {
        println!("Executing Link,  args:{:?}", args);
    }
}
