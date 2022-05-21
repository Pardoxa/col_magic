use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cmd
{
    pub file: String,
    pub commands: Vec<String>,

    #[structopt(short, long)]
    pub verbose: bool
}