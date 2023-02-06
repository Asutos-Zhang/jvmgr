
 pub trait Cmd {
    fn execute(&self,args:Vec<String>);
}