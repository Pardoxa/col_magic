use std::ops::*;
use std::fmt::Debug;
use super::io::*;

pub trait GetFloat
{
    fn get_float(&self, data: &[LazyFloatParser]) -> f64;
    fn get_float_const(&self) -> Option<f64>
    {
        None
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
    /// get a number that depends on the table
    fn get_float(&self, _: &[LazyFloatParser]) -> f64 {
        self.val
    }

    /// If the value is not dependant on the table
    fn get_float_const(&self) -> Option<f64> {
        Some(self.val)
    }
}

// Note: I think I might be able to avoid all live times by 
// using 'static here.
// But all the 'a don't really bother me,
// so I keep them
pub struct Calculation<'a>
{
    value_getter: Box<dyn GetFloat + 'a >
}

impl<'a> Calculation<'a>
{
    /// tries to avoid calculating something over and 
    /// over again, if the result is KNOWN to
    /// never change
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

macro_rules! create_branch_struct {
    ($t:ident, $operation:ident) => {
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
    }
}

macro_rules! create_expr_struct {
    ($t: ident, $f:ident) => {
        pub struct $t<'a>
        {
            a: Calculation<'a>
        }

        impl<'a> $t<'a>
        {
            pub fn new(root: Calculation<'a>) -> Self
            {
                Self{a: root}
            }
        }


        impl<'a> GetFloat for $t<'a>
        {
            fn get_float(&self, data: &[LazyFloatParser]) -> f64 {
                self.a.get_float(data).$f()
            }
        
            fn get_float_const(&self) -> Option<f64> {
                self.a.get_float_const().map(|v| v.$f())
            }
        }
    };
}


create_branch_struct!(AddBranch, add);
create_branch_struct!(SubBranch, sub);
create_branch_struct!(MulBranch, mul);
create_branch_struct!(DivBranch, div);
create_branch_struct!(PowBranch, powf);

// I am quite sure there is a way to get all of these in one macro call,
// but this is short enough for me
create_expr_struct!(Sin, sin);
create_expr_struct!(Minus, neg);
create_expr_struct!(Exp, exp);
create_expr_struct!(Cos, cos);
create_expr_struct!(Ln, ln);
create_expr_struct!(Tan, tan);
create_expr_struct!(Asin, asin);
create_expr_struct!(Acos, acos);
create_expr_struct!(Atan, atan);


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


