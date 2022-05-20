use super::*;

#[derive(Debug, PartialEq)]
pub enum Operation{
    Plus,
    Mul
}

pub fn parse_command(command: &str) -> Root<'static>
{
    let command = LexItem::parse(command);
    println!("{:?}", command);
    collapse(command)
}

pub fn check_parenthesis(slice: &[LexItem]) -> u32
{
    let mut counter = 0_u32;
    let mut max_counter = counter;
    for item in slice 
    {
        if let LexItem::Parentesis(p) = item
        {
            match p{
                Par::Open => {
                    counter +=1;
                    if counter > max_counter {
                        max_counter = counter;
                    }
                },
                Par::Close => {
                    counter = match counter.checked_sub(1)
                    {
                        Some(v) => v,
                        None => {
                            eprintln!("ERROR: You closed a parenthesis before opening it!");
                            std::process::exit(2);
                        }
                    }
                }
            }
        }
    }

    if counter != 0{
        eprintln!("ERROR: You opened {counter} more parenthesis than you closed!");
        std::process::exit(2)
    }
    max_counter
}

fn collapse_inside_parenthesis(mut sequence: Vec<LexItem<'static>>) -> Root<'static>
{
    println!("{:?}", sequence);
    if sequence.is_empty(){
        panic!("PARSING ERROR - empty sequence passed");
    }
    else if sequence.len() == 1 {
        match sequence.pop().unwrap()
        {
            LexItem::Root(root) => return root,
            _ => {
                panic!("Invalid command")
            }
        }
    }
    let pos = sequence.iter().position(|i| matches!(i, LexItem::Expression(_)));

    if let Some(p) = pos {
        let mut iter = sequence.drain(p..=p+1);
        let expression = match iter.next().unwrap()
        {
            LexItem::Expression(e) => e,
            _ => unreachable!()
        };
        let root = match iter.next().unwrap()
        {
            LexItem::Root(r) => r,
            _ => {
                eprintln!("Invalid expression placement!");
                std::process::exit(1);
            }
        };
        let root = match expression{
            Expression::Exp => {
                let exp = Exp::new(root);
                exp.into()
            }, 
            Expression::Sin => {
                let sin = Sin::new(root);
                sin.into()
            }
        };
        drop(iter);
        sequence.insert(p, LexItem::Root(root));
        return collapse_inside_parenthesis(sequence);
    }

    let pos = sequence.iter().position(|i| matches!(i, LexItem::Operation(Operation::Mul)));

    if let Some(pos) = pos {
        if pos == 0 || pos+1>= sequence.len() {
            eprintln!("ERROR: Multiplication invalid");
            std::process::exit(1);
        }
        let mut iter = sequence.drain(pos-1..=pos+1);

        let first = iter.next().unwrap();
        let left = match first 
        {
            LexItem::Root(root) => root,
            _ => panic!("ERROR: Multiplication, left is not reducable to number :(")
        };
        let last = iter.nth(1).unwrap();
        let right = match last 
        {
            LexItem::Root(root) => root,
            _ => panic!("ERROR: Multiplication, right is not reducable to number :(")
        };
        let mul = MulBranch::new(left, right);
        drop(iter);
        sequence.insert(pos-1, LexItem::Root(mul.into()));
        return collapse_inside_parenthesis(sequence);
    }

    let pos = sequence.iter().position(|i| matches!(i, LexItem::Operation(Operation::Plus) | LexItem::Minus));

    if let Some(pos) = pos {

        
        if pos == 0 || pos+1>= sequence.len() {
            eprintln!("ERROR: Addition invalid");
            std::process::exit(1);
        }
        let mut iter = sequence.drain(pos-1..=pos+1);

        let first = iter.next().unwrap();
        let left = match first 
        {
            LexItem::Root(root) => root,
            _ => panic!("ERROR: Addition, left is not reducable to number :(")
        };
        let operation = iter.next().unwrap();
        let last = iter.next().unwrap();
        let right = match last 
        {
            LexItem::Root(root) => root,
            _ => {
                panic!("ERROR: Addition, right is not reducable to number :( {:?}", last)
            }
        };
        drop(iter);
        let root = if matches!(&operation, LexItem::Operation(Operation::Plus)){
            let add = AddBranch::new(left, right);
            add.into()
        } else {
            let min = MinusBranch::new(left, right);
            min.into()
        };

        sequence.insert(pos-1, LexItem::Root(root));
        return collapse_inside_parenthesis(sequence);
        

        
    }
    println!("{:?}", sequence);
    unimplemented!()
}

pub fn collapse(mut command_sequence: Vec<LexItem<'static>>) -> Root<'static>
{
    let max_par = check_parenthesis(&command_sequence);
    if max_par == 0 {
        return collapse_inside_parenthesis(command_sequence);
    }
    let mut pos_start = 0;
    let mut pos_end = 0;
    let mut counter = 0;

    for (i, c) in command_sequence.iter().enumerate()
    {
        if let LexItem::Parentesis(p) = c {
            match p {
                Par::Open => {
                    counter += 1;
                    if counter == max_par{
                        pos_start = i;
                    }
                },
                Par::Close => {
                    if counter == max_par {
                        pos_end = i;
                        break
                    }
                    counter -= 1;
                }
            }
        }
    }
    let mut iter = command_sequence.drain(pos_start..=pos_end);
    let last = iter.next_back();
    assert!(matches!(last, Some(LexItem::Parentesis(Par::Close))));
    let first = iter.next();
    assert!(matches!(first, Some(LexItem::Parentesis(Par::Open))));
    let inside = iter.collect();
    let root = collapse_inside_parenthesis(inside);
    command_sequence.insert(pos_start, LexItem::Root(root));

    collapse(command_sequence)
}

#[derive(Debug)]
pub enum Par{
    Open,
    Close
}

#[derive(Debug)]
pub enum Expression{
    Exp,
    Sin
}

#[derive(Debug)]
pub enum LexItem<'a>{
    Root(Root<'a>),
    Parentesis(Par),
    Operation(Operation),
    Expression(Expression),
    // note: minus can be sign or operator!
    Minus
}

impl<'a> LexItem<'a>
{
    pub fn parse(command: &str) -> Vec<Self>
    {
        let mut result = Vec::new();
        let mut s = command.trim_start();
        while !s.is_empty()
        {
            let l;
            (l, s) = Self::get_next(s);
            s = s.trim_start();
            result.push(l);
        }
        result
    }

    fn get_next(substr: &str) -> (Self, &str)
    {

        if let Some(p) = substr.strip_prefix('(') {
            return (LexItem::Parentesis(Par::Open), p);
        } else if let Some(p) = substr.strip_prefix(')')
        {
            return (LexItem::Parentesis(Par::Close), p);
        } else if let Some(p) = substr.strip_prefix('+')
        {
            return (LexItem::Operation(Operation::Plus), p);
        } else if let Some(p) = substr.strip_prefix('*')
        {
            return (LexItem::Operation(Operation::Mul), p);
        } else if let Some(p) = substr.strip_prefix('C')
        {
            let integer = r"^\d+";
            let re = regex::Regex::new(integer).unwrap();

            return match re.find(p)
            {
                Some(m) => {
                    let idx = &p[m.start()..m.end()];
                    let idx = idx.parse().unwrap();
                    let col = Column::new(idx);
                    (LexItem::Root(col.into()), &p[m.end()..])
                }, 
                None => {
                    panic!("Error at parsing colum index")
                }
            };
        } else if let Some(p) = substr.strip_prefix("sin")
        {
            return (LexItem::Expression(Expression::Sin), p);
        } else if let Some(p) = substr.strip_prefix("exp") {
            return (LexItem::Expression(Expression::Exp), p);
        } else if let Some(p) = substr.strip_prefix('-')
        {
            return (LexItem::Minus, p);
        }
        // match floats
        let float = r"^\d*\.?\d*";
        let re = regex::Regex::new(float).unwrap();
        
        if let Some(m) = re.find(substr) {
            let sub = &substr[m.start()..m.end()];
            let num = sub.parse().unwrap();
            let val = Value::new(num);
            let root = val.into();
            return (LexItem::Root(root), &substr[m.end()..]);
        }
        unimplemented!()
    }
}