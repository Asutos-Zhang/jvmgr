use clap:: Parser;

mod command;
use command::CmdHolder;

#[derive(Debug,Parser)]
#[clap(author="asutos", version="0.0.1",about="http://asutos.top",long_about="aaaabbbbccccddd")]
struct SubCommand {
     #[clap(short,long)]
    subCmd:String,
    args:Vec<String>,
}

fn main() {
    let command = SubCommand::parse();

    println!("{:?}",&command);

    CmdHolder::generate(command.subCmd).execute(command.args);
}