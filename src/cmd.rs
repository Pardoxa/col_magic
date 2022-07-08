use structopt::StructOpt;

#[derive(StructOpt)]
/// Used to do table calculations on a text file.
/// For each command you give, a column will be outputted
///
/// 
/// C0 stands for the index
/// 
/// C1 stands for column 1
/// 
/// C<n> stands for column n (exchange <n> for the column number!)
/// 
/// Currently implemented operators:
/// + - * /
/// 
/// Example: col_magic table.dat "C0 + C1 * C2 ^ C3"
/// 
/// You can also use brackets
/// 
/// Example: col_magic table.dat "(C0 + C1) * C2 ^ C3"
/// 
/// There are also functions implemented:
/// abs, sin, sinh, cos, cosh, ln, log10, asin, asinh, acos, acosh, sqrt, cbrt, signum, round, floor, ceil
/// 
/// These can be used like this: col_magic table.dat "sin(ln(C0 + C1) * C2) ^ round(C3)"
/// 
/// You can also use numbers and pi will be changed to the respective value like so: col_magic table.dat "2/pi+sin(ln(C0 + C1) * C2) ^ round(C3)*0.231"
/// 
/// Lastly there are functions that need two inputs:
/// 
/// min, max
/// 
/// They can be used like so:
/// 
/// col_magic table.dat "min(C0,pi)" "max(C0, pi)" "max(C1, C2)" "min(C0*C1+23.9,C0*abs(C1)/0.1*min(C0,C1))"
/// 
pub struct Cmd
{
    pub file: String,
    pub commands: Vec<String>,

    #[structopt(short, long)]
    /// Prints information during command parsing
    pub verbose: bool
}