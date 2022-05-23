use std::io::{BufRead, BufReader};
use std::cell::Cell;
use std::fs::File;

use crate::*;



pub fn process(file: &str, commands: &[Calculation])
{
    let file = File::open(file)
        .expect("unable to open file");
    
    let buf = BufReader::new(file);

    buf.lines()
        .filter_map(
            |l| 
            {
                match l {
                    Ok(s) => {
                        if s.starts_with('#'){
                            None
                        } else {
                            Some(s)
                        }
                    },
                    Err(e) => {
                        eprintln!("IO ERROR {e}");
                        std::process::exit(1);
                    }
                }
            }
        ).enumerate()
        .for_each(
            |(index, l)|
            {
                let mut lazy = vec![LazyFloatParser{s: "", lazy: Cell::new(Some(index as f64))}];

                lazy.extend(
                    l.split_whitespace()
                        .map(LazyFloatParser::new)
                );

                let mut iter = commands.iter();
                let first = iter.next().expect("No commands given? Abbort!");
                print!("{:e}", first.get_float(&lazy));
                for c in iter
                {
                    print!(" {:e}", c.get_float(&lazy));
                }
                println!()
            }
        )
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
    pub fn new(s: &'a str) -> Self
    {
        Self{
            s,
            lazy: Cell::new(None)
        }
    }

    pub fn value(&self) -> f64
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

pub fn create_lazy(line: &'_ str) -> Vec<LazyFloatParser::<'_>>
{
    line.split_whitespace()
        .map(LazyFloatParser::new)
        .collect()
}