use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "jun")]
#[command(about = "A JSON to C# codegen tool primarily for Unity projects.", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long)]
    pub class: Option<String>,
}
