use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cmd
{
    pub file: String,
    pub commands: Vec<String>,

    #[structopt(short, long)]
    /// Prints information during command parsing
    pub verbose: bool
}