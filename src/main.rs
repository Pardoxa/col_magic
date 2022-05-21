use crate::lex_parser::{parse_command};

mod lex_parser;
mod magic_chain;
pub use magic_chain::*;
pub mod io;
mod cmd;
use structopt::StructOpt;

// Since I am single threaded here. 
// Otherwise an Atomic would be used
static mut VERBOSITY: bool = false; 

fn main() {

    let opt = cmd::Cmd::from_args();

    if opt.verbose{
        unsafe{VERBOSITY = true;}
    }

    let commands: Vec<_>= opt.commands
        .iter()
        .map(|s| parse_command(s))
        .collect();
    
    io::process(&opt.file, &commands);
}


#[cfg(test)]
mod tests{
    use crate::lex_parser::parse_command;
    use lex_parser::collapse;
    use super::*;
    #[test]
    fn check()
    {
        let s = "1 2 3";
        let l = io::create_lazy(s);
        let command2 = "(2+3*3+C2*2)";

        let c = lex_parser::LexItem::parse(command2);
        
        let root = collapse(c);
        assert_eq!(17.0, root.get_float(&l));

        let command = parse_command("1+1*2");
        assert_eq!(command.get_float(&l), 3.0);
        let command = parse_command("(1+1)*2");
        assert_eq!(command.get_float(&l), 4.0);
        let command = parse_command("(1+1)*2+0.5*C0");
        assert_eq!(command.get_float(&l), 4.5);
        let command = parse_command("1--1");
        assert_eq!(command.get_float(&l), 2.0);
        let command = parse_command("1---1");
        assert_eq!(command.get_float(&l), 0.0);
        let command = parse_command("1--(-1)");
        assert_eq!(command.get_float(&l), 0.0);

        let command = parse_command("1+1-1*12");
        assert_eq!(command.get_float(&l), -10.0);

        let command = parse_command("1+1-1*-12");
        assert_eq!(command.get_float(&l), 14.0);

        let command = parse_command("exp(-12)");
        assert_eq!(command.get_float(&l), (-12.0_f64).exp());

        let command = parse_command("12*12/12");
        assert_eq!(command.get_float(&l), 12.0);
        let command = parse_command("sin(0)");
        assert_eq!(command.get_float(&l), 0.0);
    }
}