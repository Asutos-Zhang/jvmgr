use super::Cmd;

pub struct Unlink();

impl Unlink {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cmd for Unlink {
    fn execute(&self, args: Vec<String>) {
        if(args.len()==0) {
            panic!("need at least one arg");
        }
        println!("Executing Unlink,  args:{:?}", args);
    }
}