use std::ops::*;
use std::fmt::Debug;
use super::io::*;

// Of course I could also combine these two macros easily, 
// I just chose not to
macro_rules! create_branch_struct {
    ($t:ident) => {
        pub struct $t<'a> {
            a: Calculation<'a>,
            b: Calculation<'a>
        }

        impl<'a> $t<'a>
        {
            pub fn new<A, B>(a: A, b: B) -> Self
                where A: Into<Calculation<'a>>,
                      B: Into<Calculation<'a>>
            {
                Self{
                    a: a.into(),
                    b: b.into()
                }
            }
        }
    }
}

macro_rules! impl_get_float_for_branch {
    ($t:ident, $operation:ident) => {
        impl<'a> GetFloat for $t<'a> {
            fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
                self.a.get_float(data).$operation(self.b.get_float(data))
            }
            
            fn get_float_const(&self) -> Option<f64> {
               let a = self.a.get_float_const()?;
               let b = self.b.get_float_const()?;
               Some(a.$operation(b))
            }
        }
    };
}

// pow is jet to be implemented into the commands
// create_branch_struct!(Pow);
// impl_get_float_for_branch!(Pow, powf);


pub struct Sin<'a>
{
    a: Calculation<'a>
}

impl<'a> Sin<'a>
{
    pub fn new(root: Calculation<'a>) -> Self
    {
        Self{a: root}
    }
}


impl<'a> GetFloat for Sin<'a>
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
        self.a.get_float(data).sin()
    }

    fn get_float_const(&self) -> Option<f64> {
        self.a.get_float_const().map(f64::sin)
    }
}

create_branch_struct!(MinusBranch);
impl_get_float_for_branch!(MinusBranch, sub);

#[derive(Debug)]
pub struct Minus<'a>
{
    item: Calculation<'a>
}

impl<'a> Minus<'a> 
{
    pub fn new<T>(t: T) -> Self
    where T: Into<Calculation<'a>>
    {
        Self{item: t.into()}
    }
}

impl<'a> GetFloat for Minus<'a>
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
        -self.item.get_float(data)
    }

    fn get_float_const(&self) -> Option<f64> {
        self.item.get_float_const().map(|v| -v)
    }
}


pub struct Exp<'a>
{
    a: Calculation<'a>
}

impl<'a> Exp<'a>
{
    pub fn new<T>(t: T) -> Self
    where T: Into<Calculation<'a>>
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

    fn get_float_const(&self) -> Option<f64> {
        self.a.get_float_const().map(f64::exp)
    }
}

create_branch_struct!(MulBranch);
impl_get_float_for_branch!(MulBranch, mul);

create_branch_struct!(DivBranch);
impl_get_float_for_branch!(DivBranch, div);

create_branch_struct!(AddBranch);
impl_get_float_for_branch!(AddBranch, add);

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

    fn get_float_const(&self) -> Option<f64> {
        Some(self.val)
    }
}

pub trait GetFloat
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64;
    fn get_float_const(&self) -> Option<f64>
    {
        None
    }
}

pub struct Calculation<'a>
{
    value_getter: Box<dyn GetFloat + 'a >
}

impl<'a> Calculation<'a>
{
    pub fn shortcircuit_or_self(mut self) -> Self
    {
        if let Some(val) = self.get_float_const()
        {
            self.value_getter = Box::new(Value::new(val));
        }
        self
    }
}

impl<'a> Debug for Calculation<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self.get_float_const()
        {
            None => {
                f.debug_struct("Table Calculation")
            },
            Some(v) => {
                let mut dbg = f.debug_struct("Calculated");
                dbg.field("value", &format!("{v}"));
                dbg
            }
        }.finish()
    }
}

impl<'a, T: GetFloat + 'a> From<T> for Calculation<'a>
{
    fn from(t: T) -> Self {
        Self{
            value_getter: Box::new(t)
        }.shortcircuit_or_self()
    }
}

impl<'a> Deref for Calculation<'a>
{
    type Target = Box<dyn GetFloat + 'a>;

    fn deref(&self) -> &Self::Target {
        &self.value_getter
    }
}


pub fn create_lazy(line: &'_ str) -> Vec<LazyFloatParser::<'_>>
{
    line.split_whitespace()
        .map(LazyFloatParser::new)
        .collect()
}

