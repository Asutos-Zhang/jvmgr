use super::current::Current;
use super::link::Link;
use super::ls::List;
use super::unlink::Unlink;
use super::use_cmd::Use;
use super::Cmd;

pub struct CmdHolder {}

impl CmdHolder {
    pub fn generate(subCmd: String) -> Box<dyn Cmd> {

        match subCmd.as_str()  {
            "use" => Box::new(Use::new()),
            "link" => Box::new(Link::new()),
            "unlink" => Box::new(Unlink::new()),
            "list" => Box::new(List::new()),
            "current" => Box::new(Current::new()),
            __ => panic!("Not Support {}",subCmd),
        }
    }
}
