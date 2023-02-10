use super::Cmd;

pub struct Link();

impl Link {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Link {
    fn execute(&self, args: Vec<String>) {
        if(args.len()!=2) {panic!("need  two args");}
        println!("Executing Link,  args:{:?}", args);
    }
}
