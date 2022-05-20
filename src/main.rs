



use crate::lex_parser::{collapse, parse_command};

mod lex_parser;
mod magic_chain;
pub use magic_chain::*;
pub mod io;

fn main() {
    let s = "1 2 3";
    let l = create_lazy(s);
    let command2 = "(2+3*3 +C2*2)";

    let c = lex_parser::LexItem::parse(command2);
    lex_parser::check_parenthesis(&c);
    println!("{:?}", c);
    let root1 = collapse(c);
    println!("{}", root1.get_float(&l));
    let command3 = "(2+3*3+sin (C2*2))";
    let root2 = parse_command(command3);
    println!("{}", root2.get_float(&l));

    let command0 = "C0 - 1";
    let root = parse_command(command0);

    let commands = vec![root, root1, root2];
    io::process("table.dat", &commands);
}


#[cfg(test)]
mod tests{
    use crate::lex_parser::parse_command;

    use super::*;
    #[test]
    fn check()
    {
        let s = "1 2 3";
        let l = create_lazy(s);
        let command2 = "(2+3*3+C2*2)";

        let c = lex_parser::LexItem::parse(command2);
        
        let root = collapse(c);
        assert_eq!(17.0, root.get_float(&l));

        let command = parse_command("1+1*2");
        assert_eq!(command.get_float(&l), 3.0);
        let command = parse_command("(1+1)*2");
        assert_eq!(command.get_float(&l), 4.0);
        let command = parse_command("(1+1)*2+0.5*C0");
        assert_eq!(command.get_float(&l), 4.5)
    }
}