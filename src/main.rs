use std::cell::Cell;
use std::fmt::Debug;
use std::ops::Deref;

use crate::lex_parser::{collapse, parse_command};

mod lex_parser;

fn main() {
    let s = "1 2 3";
    let l = create_lazy(s);
    let command2 = "(2+3*3+C2*2)";

    let c = lex_parser::LexItem::parse(command2);
    lex_parser::check_parenthesis(&c);
    println!("{:?}", c);
    let root = collapse(c);
    println!("{}", root.get_float(&l));
    let command3 = "(2+3*3+sin(C2*2))";
    let root = parse_command(command3);
    println!("{}", root.get_float(&l));
}

pub struct Pow<'a>
{
    a: Root<'a>,
    b: Root<'a>
}

impl<'a> Pow<'a>
{
    fn new<A, B>(a: A, b: B) -> Self
    where A: Into<Root<'a>>,
        B: Into<Root<'a>>
    {
        Self { a: a.into(), b: b.into() }
    }
}

impl<'a> GetFloat for Pow<'a> {
    fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
        self.a.get_float(data).powf(self.b.get_float(data))
    }
}

pub struct Sin<'a>
{
    a: Root<'a>
}

impl<'a> Sin<'a>
{
    pub fn new(root: Root<'a>) -> Self
    {
        Self{a: root}
    }
}

impl<'a> GetFloat for Sin<'a>
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
        self.a.get_float(data).sin()
    }
}


pub struct Exp<'a>
{
    a: Root<'a>
}

impl<'a> Exp<'a>
{
    fn new<T>(t: T) -> Self
    where T: Into<Root<'a>>
    {
        Self{
            a: t.into()
        }
    }
}

impl<'a> GetFloat for Exp<'a>
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
        self.a.get_float(data).exp()
    }
}

pub struct MulBranch<'a>
{
    a: Root<'a>,
    b: Root<'a>
}

impl<'a> MulBranch<'a>
{
    pub fn new<A, B>(a: A, b: B) -> Self
    where A: Into<Root<'a>>,
        B: Into<Root<'a>>
    {
        Self{
            a: a.into(),
            b: b.into()
        }
    }
}

impl<'a> GetFloat for MulBranch<'a>
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
        self.a.get_float(data) * self.b.get_float(data)
    }
}

pub struct AddBranch<'a>
{
    a: Root<'a>,
    b: Root<'a>
}



impl<'a> AddBranch<'a>
{
    fn new<A, B>(a: A, b: B) -> Self
    where A: Into<Root<'a>>,
        B: Into<Root<'a>>
    {
        Self{
            a: a.into(),
            b: b.into()
        }
    }
}


impl<'a> GetFloat for AddBranch<'a>
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
        self.a.get_float(data) + self.b.get_float(data)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Column
{
    col: usize
}

impl Column{
    pub fn new(col: usize) -> Self
    {
        Self{col}
    }
}

impl GetFloat for Column
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
        data[self.col].value()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Value
{
    val: f64
}

impl Value{
    pub fn new(val: f64) -> Self
    {
        Value { val }
    }
}

impl GetFloat for Value {
    fn get_float(&self, _: &[LazyFloatParser]) -> f64 {
        self.val
    }
}

pub trait GetFloat
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64;
}

pub struct Root<'a>
{
    value_getter: Box<dyn GetFloat + 'a >
}

impl<'a> Debug for Root<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Root")
            .finish()
    }
}

impl<'a, T: GetFloat + 'a> From<T> for Root<'a>
{
    fn from(t: T) -> Self {
        Self{
            value_getter: Box::new(t)
        }
    }
}

impl<'a> Deref for Root<'a>
{
    type Target = Box<dyn GetFloat + 'a>;

    fn deref(&self) -> &Self::Target {
        &self.value_getter
    }
}


fn create_lazy(line: &'_ str) -> Vec<LazyFloatParser::<'_>>
{
    line.split_whitespace()
        .map(LazyFloatParser::new)
        .collect()
}


#[derive(Debug)]
pub struct LazyFloatParser<'a>
{
    s: &'a str,
    // I know the cell is unnessessary for this usecase
    // But I wanted to use it for once XD
    lazy: Cell<Option<f64>>
}

impl<'a> LazyFloatParser<'a>
{
    fn new(s: &'a str) -> Self
    {
        Self{
            s,
            lazy: Cell::new(None)
        }
    }

    fn value(&self) -> f64
    {
        match self.lazy.get()
        {
            Some(v) => v,
            None => {
                let val = self.s.parse().unwrap();
                self.lazy.set(Some(val));
                val
            }
        }
    }
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